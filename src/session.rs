use std::error::Error;
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jboolean, jlong, jstring};
use vodozemac::{base64_decode, base64_encode};
use super::{jstring_to_string, CustomError, OlmMessage};

pub struct Session {
    pub(super) inner: vodozemac::olm::Session,
}

impl Session {
    pub fn pickle(&self, pickle_key: String) -> Result<String, Box<dyn Error>> {
        let pickle_key: &[u8; 32] = pickle_key
            .as_bytes()
            .try_into()
            .map_err(|_| Box::new(CustomError("Invalid pickle key length, expected 32 bytes".to_owned())))?;

        Ok(self.inner.pickle().encrypt(pickle_key))
    }

    pub fn from_pickle(pickle: String, pickle_key: String) -> Result<Session, Box<dyn Error>> {
        let pickle_key: &[u8; 32] = pickle_key
            .as_bytes()
            .try_into()
            .map_err(|_| Box::new(CustomError("Invalid pickle key length, expected 32 bytes".to_owned())))?;
        let pickle = vodozemac::olm::SessionPickle::from_encrypted(&pickle, pickle_key)
            .map_err(|err: _| Box::new(err) as Box<dyn Error>)?;

        let session = vodozemac::olm::Session::from_pickle(pickle);

        Ok(Self { inner: session })
    }

    pub fn from_libolm_pickle(pickle: String, pickle_key: String) -> Result<Session, Box<dyn Error>> {
        let session =
            vodozemac::olm::Session::from_libolm_pickle(&pickle, &pickle_key.as_bytes()).map_err(|err: _| Box::new(err) as Box<dyn Error>)?;

        Ok(Self { inner: session })
    }

    pub fn session_id(&self) -> String {
        self.inner.session_id()
    }

    pub fn session_matches(&self, message: &OlmMessage) -> bool {
        let message =
            vodozemac::olm::OlmMessage::from_parts(
                message.message_type.try_into().unwrap(),
                &base64_decode(&message.ciphertext).unwrap()
            );

        match message {
            Ok(m) => {
                if let vodozemac::olm::OlmMessage::PreKey(m) = m {
                    self.inner.session_keys() == m.session_keys()
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }

    pub fn encrypt(&mut self, plaintext: String) -> OlmMessage {
        let message = self.inner.encrypt(plaintext);

        let (message_type, ciphertext) = message.to_parts();

        OlmMessage {
            ciphertext: base64_encode(ciphertext), //String::from_utf8_lossy(&ciphertext).into_owned(),
            message_type: message_type.try_into().unwrap(),
        }
    }

    pub fn decrypt(&mut self, message: &OlmMessage) -> Result<String, Box<dyn Error>> {
        /*let _message =
            vodozemac::olm::OlmMessage::from_parts(message.message_type.try_into().unwrap(), &message.ciphertext.as_bytes())
                .map_err(|err: _| Error::new(Status::GenericFailure, err.to_string().to_owned()))?;

        Ok(self.inner.decrypt(&_message).map_err(|err: _| Error::new(Status::GenericFailure, err.to_string().to_owned()))?)*/
        let _message = vodozemac::olm::OlmMessage::from_parts(
            message.message_type.try_into().unwrap(),
            &base64_decode(&message.ciphertext).unwrap()
        )
            .map_err(|err: _| Box::new(err) as Box<dyn Error>)?;

        let decrypted_message = self.inner.decrypt(&_message)
            .map_err(|err: _| Box::new(err) as Box<dyn Error>)?;

        let decrypted_message = String::from_utf8(decrypted_message)
            .map_err(|err| Box::new(err) as Box<dyn Error>)?;

        Ok(decrypted_message)
    }
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmSession__1pickle(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
    pickle_key: JString
) -> jstring {
    let session = unsafe { &mut *(my_ptr as *mut Session) };
    let p_key: String = env.get_string(&pickle_key).expect("Couldn't get Java string").into();

    // Convert the output Rust String to a new jstring and return it
    let output_jstring: jstring = **env
        .new_string(session.pickle(p_key).unwrap())
        .expect("Failed to create output session pickle");

    output_jstring
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmSession__1from_1pickle(
    mut env: JNIEnv,
    _class: JClass,
    pickle: JString,
    pickle_key: JString
) -> jlong {

    let acc = Session::from_pickle(
        jstring_to_string(&mut env, pickle),
        jstring_to_string(&mut env, pickle_key)
    );

    Box::into_raw(Box::new(acc)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmSession__1from_1pickle_1lib_1olm(
    mut env: JNIEnv,
    _class: JClass,
    pickle: JString,
    pickle_key: JString
) -> jlong {

    let acc = Session::from_libolm_pickle(
        jstring_to_string(&mut env, pickle),
        jstring_to_string(&mut env, pickle_key)
    );

    Box::into_raw(Box::new(acc)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmAccount__1session_1id(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
) -> jstring {
    let session = unsafe { &mut *(my_ptr as *mut Session) };

    // Convert the output Rust String to a new jstring and return it
    let output_jstring: jstring = **env
        .new_string(session.session_id())
        .expect("Failed to create output session_id");

    output_jstring
}

// session_matches
#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmSession__1session_1matches(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
    ciphertext: JString,
    message_type: jlong,
) -> jboolean {
    let session = unsafe { &mut *(my_ptr as *mut Session) };

    let message = OlmMessage {
        ciphertext: jstring_to_string(&mut env, ciphertext),
        message_type: message_type.try_into().unwrap(),
    };
    let res = session.session_matches(&message);

    res
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmSession__1decrypt(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
    ciphertext: JString,
    message_type: jlong,
) -> jstring {
    let session = unsafe { &mut *(my_ptr as *mut Session) };

    let message = OlmMessage {
        ciphertext: jstring_to_string(&mut env, ciphertext),
        message_type: message_type.try_into().unwrap(),
    };

    let output_jstring: jstring = **env
        .new_string(session.decrypt(&message).unwrap())
        .expect("Failed to create output session_id");

    output_jstring
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmSession__1encrypt<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    my_ptr: jlong,
    message: JString<'a>,
) -> JObject<'a> {
    let session = unsafe { &mut *(my_ptr as *mut Session) };


    let crypted = jstring_to_string(&mut env, message);
    let res = session.encrypt(crypted);

    let java_class = env.find_class("de/cogia/vodozemac/OlMessage").unwrap();

    let msg_type = res.message_type as jlong;
    let msg = env.new_string(res.ciphertext).unwrap();

    let args: &[JValue] = &[
        JValue::Object(&msg),
        JValue::Long(msg_type),
    ];

    let java_object = env.new_object(
        java_class,
        "(Ljava/lang/String;Ljava/lang/Long;)V",
        args
    ).unwrap();

    java_object
}
//encrypt