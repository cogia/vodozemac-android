use std::collections::HashMap;
use vodozemac::base64_decode;
use vodozemac::olm::{InboundCreationResult, SessionConfig};
use std::error::Error;
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jlong, jstring};
use super::{session::Session, OlmMessage, IdentityKeys, CustomError, jstring_to_string};


pub struct Account {
    inner: vodozemac::olm::Account,
}


impl Account {
    pub fn new() -> Self {
        Self {
            inner: vodozemac::olm::Account::new(),
        }
    }

    pub fn identity_keys(&self) -> Result<IdentityKeys, &'static str> {
        let identity_keys = self.inner.identity_keys();//.map_err(|_| {});
        Ok(
            IdentityKeys {
                ed25519: identity_keys.ed25519.to_base64(),
                curve25519: identity_keys.curve25519.to_base64(),
            }
        )
    }

    pub fn from_pickle(pickle: String, pickle_key: String) -> Result<Account, Box<dyn Error>> {
        let pickle_key: &[u8; 32] = pickle_key
            .as_bytes()
            .try_into()
            .map_err(|err: _| Box::new(err) as Box<dyn Error>)?;

        let pickle = vodozemac::olm::AccountPickle::from_encrypted(&pickle, pickle_key)
            .map_err(|err: _| Box::new(err) as Box<dyn Error>)?;


        let inner = vodozemac::olm::Account::from_pickle(pickle);

        Ok(Self { inner })
    }

    pub fn from_libolm_pickle(pickle: String, pickle_key: String) -> Result<Account, Box<dyn Error>> {
        let inner =
            vodozemac::olm::Account::from_libolm_pickle(&pickle, &pickle_key.as_bytes())
                .map_err(|err: _| Box::new(err) as Box<dyn Error>)?;

        Ok(Self { inner })
    }

    pub fn pickle(&self, pickle_key: String) -> Result<String, Box<dyn Error>> {
        let pickle_key: &[u8; 32] = pickle_key
            .as_bytes()
            .try_into()
            .map_err(|err: _| Box::new(err) as Box<dyn Error>)?;

        Ok(self.inner.pickle().encrypt(pickle_key))
    }

    pub fn ed25519_key(&self) -> String {
        self.inner.ed25519_key().to_base64()
    }

    pub fn curve25519_key(&self) -> String {
        self.inner.curve25519_key().to_base64()
    }

    pub fn sign(&self, message: String) -> String {
        self.inner.sign(&message).to_base64()
    }

    pub fn max_number_of_one_time_keys(&self) -> u32 {
        self.inner.max_number_of_one_time_keys().try_into().unwrap()
    }

    pub fn one_time_keys(&self) -> Result<HashMap<String, String>, &'static str> {
        let _keys: HashMap<_, _> = self
            .inner
            .one_time_keys()
            .into_iter()
            .map(|(k, v)| (k.to_base64(), v.to_base64()))
            .collect();

        Ok(_keys)
    }

    pub fn generate_one_time_keys(&mut self, count: u32) {
        self.inner.generate_one_time_keys(count.try_into().unwrap());
    }


    pub fn fallback_key(&self) -> Result<HashMap<String, String>, &'static str> {
        let _keys: HashMap<String, String> = self
            .inner
            .fallback_key()
            .into_iter()
            .map(|(k, v)| (k.to_base64(), v.to_base64()))
            .collect();

        Ok(_keys)
    }

    pub fn generate_fallback_key(&mut self) {
        self.inner.generate_fallback_key()
        ;
    }

    pub fn mark_keys_as_published(&mut self) {
        self.inner.mark_keys_as_published()
    }

    pub fn create_outbound_session(
        &self,
        identity_key: String,
        one_time_key: String,
        config: &mut SessionConfig
    ) -> Result<Session, Box<dyn Error>> {
        let _config = if config.version() == 2 { vodozemac::megolm::SessionConfig::version_2() } else { vodozemac::megolm::SessionConfig::version_1() };

        let identity_key =
            vodozemac::Curve25519PublicKey::from_base64(&identity_key).map_err(|err: _| Box::new(err) as Box<dyn Error>)?;
        let one_time_key =
            vodozemac::Curve25519PublicKey::from_base64(&one_time_key).map_err(|err: _| Box::new(err) as Box<dyn Error>)?;
        let session = self
            .inner
            .create_outbound_session(SessionConfig::version_2(), identity_key, one_time_key);

        Ok(Session { inner: session })
    }

    pub fn create_inbound_session(
        &mut self,
        identity_key: String,
        message: &OlmMessage,
    ) -> Result<InboundCreationResult, Box<dyn Error>> {
        let identity_key =
            vodozemac::Curve25519PublicKey::from_base64(&identity_key)
                .map_err(|err: _| Box::new(err) as Box<dyn Error>)?;

        let _message = vodozemac::olm::OlmMessage::from_parts(
            message.message_type.try_into().unwrap(),
            &(base64_decode(&message.ciphertext).unwrap())
            // &message.ciphertext.as_bytes()
        )
            .map_err(|err: _| Box::new(err) as Box<dyn Error>)?;

        if let vodozemac::olm::OlmMessage::PreKey(m) = _message {
            let res = self
                .inner
                .create_inbound_session(identity_key, &m)
                .map_err(|err: _| Box::new(err) as Box<dyn Error>)?;

            Ok(res)
        } else {
            Err(Box::new(CustomError("Invalid message type, expected a pre-key message".to_owned())))
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmAccount__1new() -> jlong {
    Box::into_raw(Box::new(Account::new())) as jlong
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmAccount__1identity_1keys(mut env: JNIEnv, my_ptr: jlong) -> JObject {
    let acc = unsafe { &mut *(my_ptr as *mut Account) };
    let keys = acc.identity_keys().unwrap();
    let java_class = env.find_class("de/cogia/vodozemac/IdentityKeys").unwrap();

    let ed25519 = env.new_string(keys.ed25519).unwrap();
    let curve25519 = env.new_string(keys.curve25519).unwrap();
    let args: &[JValue] = &[
        (&ed25519).into(),
        (&curve25519).into(),
    ];

    let java_object = env.new_object(
        java_class,
        "(Ljava/lang/String;Ljava/lang/String;)V",
        args
    )
        .unwrap();
    java_object
}


#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmAccount__1pickle(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
    pickle_key: JString
) -> jstring {
    let acc = unsafe { &mut *(my_ptr as *mut Account) };
    let p_key: String = env.get_string(&pickle_key).expect("Couldn't get Java string").into();

    // Convert the output Rust String to a new jstring and return it
    let output_jstring: jstring = **env
        .new_string(acc.pickle(p_key).unwrap())
        .expect("Failed to create output jstring");

    output_jstring
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmAccount__1from_1pickle(
    mut env: JNIEnv,
    _class: JClass,
    pickle: JString,
    pickle_key: JString
) -> jlong {

    let acc = Account::from_pickle(
        jstring_to_string(&mut env, pickle),
        jstring_to_string(&mut env, pickle_key)
    );

    Box::into_raw(Box::new(acc)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmAccount__1from_1pickle_1lib_1olm(
    mut env: JNIEnv,
    _class: JClass,
    pickle: JString,
    pickle_key: JString
) -> jlong {

    let acc = Account::from_libolm_pickle(
        jstring_to_string(&mut env, pickle),
        jstring_to_string(&mut env, pickle_key)
    );

    Box::into_raw(Box::new(acc)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmAccount__1ed25519_1key(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
) -> jstring {
    let acc = unsafe { &mut *(my_ptr as *mut Account) };

    // Convert the output Rust String to a new jstring and return it
    let output_jstring: jstring = **env
        .new_string(acc.ed25519_key())
        .expect("Failed to create output ed25519_key");

    output_jstring
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmAccount__1curve25519Key(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
) -> jstring {
    let acc = unsafe { &mut *(my_ptr as *mut Account) };

    // Convert the output Rust String to a new jstring and return it
    let output_jstring: jstring = **env
        .new_string(acc.curve25519_key())
        .expect("Failed to create output curve25519_key");

    output_jstring
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmAccount__1sign(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
    message: JString,
) -> jstring {
    let acc = unsafe { &mut *(my_ptr as *mut Account) };

    let msg = jstring_to_string(&mut env, message);
    // Convert the output Rust String to a new jstring and return it
    let output_jstring: jstring = **env
        .new_string(acc.sign(msg))
        .expect("Failed to create output curve25519_key");

    output_jstring
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmAccount__1maxNumberOfOneTimeKeys(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
) -> jlong {
    let acc = unsafe { &mut *(my_ptr as *mut Account) };
    acc.max_number_of_one_time_keys() as jlong
}


#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmAccount__1oneTimeKeys(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
) -> jstring {
    let acc = unsafe { &mut *(my_ptr as *mut Account) };

    let keys = acc.one_time_keys().unwrap();
    let output_jstring: jstring = **env
        .new_string(serde_json::to_string(&keys).unwrap())
        .expect("Failed to create output ed25519_key");

    output_jstring
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmAccount__1generateOneTimeKeys(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
    amount: jlong,
) {
    let acc = unsafe { &mut *(my_ptr as *mut Account) };
    acc.generate_one_time_keys(amount as u32);
}

// fallback_key

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmAccount__1fallbackKey(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
) -> jstring {
    let acc = unsafe { &mut *(my_ptr as *mut Account) };

    let keys = acc.fallback_key().unwrap();
    let output_jstring: jstring = **env
        .new_string(serde_json::to_string(&keys).unwrap())
        .expect("Failed to create output fallback_key");

    output_jstring
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmAccount__1generateFallbackKey(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
) {
    let acc = unsafe { &mut *(my_ptr as *mut Account) };
    acc.generate_fallback_key();
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmAccount__1markKeysAsPublished(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
) {
    let acc = unsafe { &mut *(my_ptr as *mut Account) };
    acc.mark_keys_as_published();
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmAccount__1createOutboundSession(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
    identity_key: JString,
    one_time_key: JString,
    config: jlong
) -> jlong {
    let acc = unsafe { &mut *(my_ptr as *mut Account) };
    let session_config = unsafe { &mut *(config as *mut SessionConfig) };
    let ik = jstring_to_string(&mut env, identity_key);
    let otk = jstring_to_string(&mut env, one_time_key);
    let session = acc.create_outbound_session(ik, otk, session_config).unwrap();

    Box::into_raw(Box::new(session)) as jlong
}


#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmAccount__1createInboundSession<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass,
    my_ptr: jlong,
    identity_key: JString<'a>,
    chipertext: JString<'a>,
    message_type: jlong
) -> JObject<'a> {
    let acc = unsafe { &mut *(my_ptr as *mut Account) };

    let ik = jstring_to_string(&mut env, identity_key);
    let message = OlmMessage {
        ciphertext: jstring_to_string(&mut env, chipertext),
        message_type: message_type as u32
    };

    let res = acc.create_inbound_session(ik, &message).unwrap();

    let session = Session { inner: res.session };
    let ptr = Box::into_raw(Box::new(session)) as jlong;
    let message = String::from_utf8_lossy(&res.plaintext).to_string();
    let jmessage  =  env.new_string(&message).unwrap();

    let java_class = env.find_class("de/cogia/vodozemac/InboundCreationResult").unwrap();

    let args: &[JValue] = &[
        (&jmessage).into(),
        (ptr).into(),
    ];

    let java_object = env.new_object(
        java_class,
        "(Ljava/lang/String;J)V",
        args
    ).unwrap();

    java_object.into()
}

