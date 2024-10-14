use std::error::Error;
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jlong, jstring};
use super::{jstring_to_string, result_or_java_exception, CustomError, SessionConfig};

use vodozemac::megolm::{ExportedSessionKey, MegolmMessage, SessionKey};

pub struct GroupSession {
    pub(super) inner: vodozemac::megolm::GroupSession,
}

impl GroupSession {
    pub fn new(config: &mut SessionConfig) -> Self {
        let _config = if config.version() == 2 { vodozemac::megolm::SessionConfig::version_2() } else { vodozemac::megolm::SessionConfig::version_1() };

        Self {
            inner: vodozemac::megolm::GroupSession::new(_config),
        }
    }

    pub fn session_id(&self) -> String {
        self.inner.session_id()
    }

    pub fn session_key(&self) -> String {
        self.inner.session_key().to_base64()
    }

    pub fn message_index(&self) -> u32 {
        self.inner.message_index()
    }

    pub fn encrypt(&mut self, plaintext: String) -> String {
        self.inner.encrypt(&plaintext).to_base64()
    }

    pub fn pickle(&self, pickle_key: String) -> Result<String, Box<dyn Error>> {
        let pickle_key: &[u8; 32] = pickle_key
            .as_bytes()
            .try_into()
            .map_err(|_| Box::new(CustomError("Invalid pickle key length, expected 32 bytes".to_owned())))?;

        Ok(self.inner.pickle().encrypt(pickle_key))
    }
    pub fn from_pickle(pickle: String, pickle_key: String) -> Result<GroupSession, Box<dyn Error>> {
        let pickle_key: &[u8; 32] = pickle_key
            .as_bytes()
            .try_into()
            .map_err(|_| Box::new(CustomError("Invalid pickle key length, expected 32 bytes".to_owned())))?;
        let pickle = vodozemac::megolm::GroupSessionPickle::from_encrypted(&pickle, pickle_key)
            .map_err(|err: _| Box::new(err) as Box<dyn Error>)?;

        let session = vodozemac::megolm::GroupSession::from_pickle(pickle);

        Ok(Self { inner: session })
    }
}

pub struct DecryptedMessage {
    pub plaintext: String,
    pub message_index: u32,
}

pub struct InboundGroupSession {
    pub(super) inner: vodozemac::megolm::InboundGroupSession,
}

impl InboundGroupSession {
    pub fn new(session_key: String, session_config: &SessionConfig) -> Result<InboundGroupSession, Box<dyn Error>> {
        let key = SessionKey::from_base64(&session_key).map_err(|err: _| Box::new(err) as Box<dyn Error>)?;
        let config = if session_config.version() == 2 { vodozemac::megolm::SessionConfig::version_2() } else { vodozemac::megolm::SessionConfig::version_1() };
        Ok(Self {
            inner: vodozemac::megolm::InboundGroupSession::new(&key, config),
        })
    }
    pub fn import(session_key: String, session_config: &SessionConfig) -> Result<InboundGroupSession, Box<dyn Error>> {

        let config = if session_config.version() == 2 { vodozemac::megolm::SessionConfig::version_2() } else { vodozemac::megolm::SessionConfig::version_1() };

        let key = ExportedSessionKey::from_base64(&session_key).map_err(|err: _| Box::new(err) as Box<dyn Error>)?;

        Ok(Self {
            inner: vodozemac::megolm::InboundGroupSession::import(&key, config),
        })
    }

    pub fn session_id(&self) -> String {
        self.inner.session_id()
    }

    pub fn first_known_index(&self) -> u32 {
        self.inner.first_known_index()
    }

    pub fn export_at(&mut self, index: u32) -> Option<String> {
        self.inner.export_at(index).map(|k| k.to_base64())
    }

    pub fn decrypt(&mut self, ciphertext: String) -> Result<DecryptedMessage, Box<dyn Error>> {
        let message = MegolmMessage::from_base64(&ciphertext).map_err(|err: _| Box::new(err) as Box<dyn Error>)?;
        let ret = self.inner.decrypt(&message).map_err(|err: _| Box::new(err) as Box<dyn Error>)?;

        Ok(DecryptedMessage {
            plaintext: String::from_utf8(ret.plaintext).unwrap(),
            message_index: ret.message_index,
        })
    }
    pub fn pickle(&self, pickle_key: &[u8]) -> Result<String, Box<dyn Error>> {
        let pickle_key: &[u8; 32] = pickle_key
            .try_into()
            .map_err(|_| Box::new(CustomError("Invalid pickle key length, expected 32 bytes".to_owned())))?;

        Ok(self.inner.pickle().encrypt(pickle_key))
    }
    pub fn from_pickle(pickle: String, pickle_key: String) -> Result<InboundGroupSession, Box<dyn Error>> {
        let pickle_key: &[u8; 32] = pickle_key
            .as_bytes()
            .try_into()
            .map_err(|_| Box::new(CustomError("Invalid pickle key length, expected 32 bytes".to_owned())))?;
        let pickle =
            vodozemac::megolm::InboundGroupSessionPickle::from_encrypted(&pickle, pickle_key)
                .map_err(|err: _| Box::new(err) as Box<dyn Error>)?;

        let session = vodozemac::megolm::InboundGroupSession::from_pickle(pickle);

        Ok(Self { inner: session })
    }
    pub fn from_libolm_pickle(
        pickle: String,
        pickle_key: String,
    ) -> Result<InboundGroupSession, Box<dyn Error>> {
        let inner = vodozemac::megolm::InboundGroupSession::from_libolm_pickle(&pickle, &pickle_key.as_bytes())
            .map_err(|err: _| Box::new(err) as Box<dyn Error>)?;

        Ok(Self { inner })
    }
}


#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmGroupSession__1new(
    mut env: JNIEnv,
    _class: JClass,
    config: jlong,
) -> jlong {
    let session_config = unsafe { &mut *(config as *mut SessionConfig) };
    Box::into_raw(Box::new(GroupSession::new(session_config))) as jlong
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmGroupSession__1session_1id(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
) -> jstring {
    let session = unsafe { &mut *(my_ptr as *mut GroupSession) };

    // Convert the output Rust String to a new jstring and return it
    let output_jstring: jstring = **env
        .new_string(session.session_id())
        .expect("Failed to create output session_id");

    output_jstring
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmGroupSession__1session_1key(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
) -> jstring {
    let session = unsafe { &mut *(my_ptr as *mut GroupSession) };

    // Convert the output Rust String to a new jstring and return it
    let output_jstring: jstring = **env
        .new_string(session.session_key())
        .expect("Failed to create output session_key");

    output_jstring
}

// message_index
#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmGroupSession__1message_1index(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
) -> jlong {
    let session = unsafe { &mut *(my_ptr as *mut GroupSession) };
    session.message_index() as jlong
}


#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmGroupSession__1encrypt(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
    message: JString
) -> jstring {
    let session = unsafe { &mut *(my_ptr as *mut GroupSession) };
    let local_message = jstring_to_string(&mut env, message);

    // Convert the output Rust String to a new jstring and return it
    let output_jstring: jstring = **env
        .new_string(session.encrypt(local_message))
        .expect("Failed to create output session_key");

    output_jstring
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmGroupSession__1pickle(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
    pswd: JString
) -> jstring {
    let session = unsafe { &mut *(my_ptr as *mut GroupSession) };
    let local_pswd = jstring_to_string(&mut env, pswd);

    let pickle;
    match result_or_java_exception(&mut env, session.pickle(local_pswd)) {
        Ok(value) => {
            pickle = value;
        }
        Err(_) => {
            pickle = String::from("Invalid pickle");
        }
    }
    // Convert the output Rust String to a new jstring and return it
    let output_jstring: jstring = **env
        .new_string(pickle)
        .expect("Failed to create output session_key");

    output_jstring
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmGroupSession__1from_1pickle(
    mut env: JNIEnv,
    _class: JClass,
    pickle: JString,
    pswd: JString
) -> jlong {

    let pickle = jstring_to_string(&mut env, pickle);
    let pickle_pswd = jstring_to_string(&mut env, pswd);

    let group = GroupSession::from_pickle(pickle, pickle_pswd);

    Box::into_raw(Box::new(group)) as jlong
}


#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmInboundGroupSession__1new(
    mut env: JNIEnv,
    _class: JClass,
    session_key: JString,
    config: jlong,
) -> jlong {
    let session_config = unsafe { &mut *(config as *mut SessionConfig) };
    let session_key_local = jstring_to_string(&mut env, session_key);
    Box::into_raw(Box::new(InboundGroupSession::new(session_key_local, session_config))) as jlong
}


#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmInboundGroupSession__1pickle(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
    pswd: JString
) -> jstring {
    let session = unsafe { &mut *(my_ptr as *mut InboundGroupSession) };
    let local_pswd = jstring_to_string(&mut env, pswd);

    let res;
    match result_or_java_exception(&mut env, session.pickle(local_pswd.as_bytes())) {
        Ok(value) => {
            res = value;
        }
        Err(_) => {
            res = String::from("Invalid pickle");
        }
    }
    // Convert the output Rust String to a new jstring and return it
    let output_jstring: jstring = **env
        .new_string(res)
        .expect("Failed to create output session_key");

    output_jstring
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmInboundGroupSession__1from_1pickle(
    mut env: JNIEnv,
    _class: JClass,
    pickle: JString,
    pswd: JString
) -> jlong {

    let pickle = jstring_to_string(&mut env, pickle);
    let pickle_pswd = jstring_to_string(&mut env, pswd);

    let group = InboundGroupSession::from_pickle(pickle, pickle_pswd);

    Box::into_raw(Box::new(group)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmInboundGroupSession__1from_1libolm_1pickle(
    mut env: JNIEnv,
    _class: JClass,
    pickle: JString,
    pswd: JString
) -> jlong {

    let pickle = jstring_to_string(&mut env, pickle);
    let pickle_pswd = jstring_to_string(&mut env, pswd);

    let group = InboundGroupSession::from_libolm_pickle(pickle, pickle_pswd);

    Box::into_raw(Box::new(group)) as jlong
}


#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmInboundGroupSession__1session_1id(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
) -> jstring {
    let session = unsafe { &mut *(my_ptr as *mut InboundGroupSession) };

    // Convert the output Rust String to a new jstring and return it
    let output_jstring: jstring = **env
        .new_string(session.session_id())
        .expect("Failed to create output session_id");

    output_jstring
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmInboundGroupSession__1first_1known_1index(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
) -> jlong {
    let session = unsafe { &mut *(my_ptr as *mut InboundGroupSession) };
    session.first_known_index() as jlong
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmInboundGroupSession__1import(
    mut env: JNIEnv,
    _class: JClass,
    session_key: JString,
    config: jlong,
) -> jlong {
    let session_config = unsafe { &mut *(config as *mut SessionConfig) };
    let session_key_local = jstring_to_string(&mut env, session_key);
    let session;
    match result_or_java_exception(&mut env, InboundGroupSession::import(session_key_local, session_config)) {
        Ok(value) => {
            session = value;
        }
        Err(_) => {
            return 0 as jlong;
        }
    }
    Box::into_raw(Box::new(session)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmInboundGroupSession__1export_1at(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
    index: jlong,
) -> jstring {
    let session = unsafe { &mut *(my_ptr as *mut InboundGroupSession) };

    // Convert the output Rust String to a new jstring and return it
    let output_jstring: jstring = **env
        .new_string(session.export_at(index as u32).unwrap())
        .expect("Failed to create output session_id");

    output_jstring
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmInboundGroupSession__1decrypt<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass,
    my_ptr: jlong,
    chipertext: JString<'a>,
) -> JObject<'a> {
    let session = unsafe { &mut *(my_ptr as *mut InboundGroupSession) };

    let chipertext_local = jstring_to_string(&mut env, chipertext);
    let res;
    match result_or_java_exception(&mut env, session.decrypt(chipertext_local)) {
        Ok(value) => {
            res = value;
        }
        Err(_) => {
            return JObject::null();
        }
    }

    let decrypted_message = env.new_string(res.plaintext).unwrap();
    let decrypted_message_index = res.message_index as jlong;


    let java_class = env.find_class("de/cogia/vodozemac/OlmDecryptedMessage").unwrap();

    let args: &[JValue] = &[
        (&decrypted_message).into(),
        (decrypted_message_index).into(),
    ];

    let java_object = env.new_object(
        java_class,
        "(Ljava/lang/String;J)V",
        args
    ).unwrap();

    java_object.into()
}

