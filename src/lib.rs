mod account;
mod session;
mod sas;
mod group_sessions;

use std::{fmt};
use std::error::Error;
use jni::descriptors::Desc;
#[allow(unused_variables)]
// This is the interface to the JVM that we'll call the majority of our
// methods on.
//use jni::*;

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JThrowable, JValue};
use jni::signature::ReturnType::Object;
use jni::sys::jlong;

#[no_mangle]
pub unsafe extern "C" fn Java_de_cogia_vodozemac_internal_Native_keepAlive(
    _env: JNIEnv,
    _class: JClass,
    _obj: JObject,
) {
}

#[repr(C)]
pub struct SessionConfig {
    _version: u8,
}


impl SessionConfig {
    /// Get the numeric version of this `SessionConfig`.
    pub const fn version(&self) -> u8 {
        self._version
    }

    /// Create a `SessionConfig` for the Olm version 1. This version of Olm will
    /// use AES-256 and HMAC with a truncated MAC to encrypt individual
    /// messages. The MAC will be truncated to 8 bytes.
    pub const fn version_1() -> Self {
        SessionConfig { _version: 1 }
    }

    /// Create a `SessionConfig` for the Olm version 2. This version of Olm will
    /// use AES-256 and HMAC to encrypt individual messages. The MAC won't be
    /// truncated.
    pub const fn version_2() -> Self {
        SessionConfig { _version: 2 }
    }
}

#[repr(C)]
pub struct IdentityKeys {
    pub ed25519: String,
    pub curve25519: String,
}


pub struct OlmMessage {
    pub ciphertext: String,
    pub message_type: u32,
}

impl OlmMessage {

    pub fn new(message_type: u32, ciphertext: String) -> Self {
        Self {
            ciphertext,
            message_type,
        }
    }
}

pub fn jstring_to_string(env: &mut JNIEnv, obj: JString) -> String {
    env.get_string(&obj).expect("Couldn't get Java string").into()
}

// "de/cogia/vodozemac/OlmException"
pub fn result_or_java_exception<'a, T>(
    env: &mut JNIEnv<'a>,
    result: Result<T, Box<dyn Error>>,
) -> Result<T, Box<dyn Error>>
where
    T: 'a,
{
    return get_result_or_java_exception(env, result, "de/cogia/vodozemac/OlmException");
}

pub fn get_result_or_java_exception<'a, T>(
    env: &mut JNIEnv<'a>,
    result: Result<T, Box<dyn Error>>,
    exception_class: &str,
) -> Result<T, Box<dyn Error>>
where
    T: 'a,
{
    match result {
        Ok(value) => Ok(value),
        Err(error) => {
            let msg_obj = env.new_string(error.to_string()).unwrap();
            let obj = env.new_object(
                exception_class,
                "(Ljava/lang/String;)V",
                &[(&msg_obj).into()]
            ).expect("Couldn't create java.lang.Throwable");
            let throwable = JThrowable::from(obj);
            env.throw(throwable).unwrap();
            Err(Box::new(CustomError(error.to_string().to_owned())))
        }
    }
}

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CustomError {}



#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_SessionConfig__1version1() -> jlong {
    Box::into_raw(Box::new(SessionConfig::version_1())) as jlong
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_SessionConfig__1version2() -> jlong {
    Box::into_raw(Box::new(SessionConfig::version_2())) as jlong
}


#[no_mangle]
pub unsafe extern "C" fn Java_de_cogia_vodozemac_SessionConfig__1version(mut _env: JNIEnv,
                                                                       _class: JClass,
                                                                       counter_ptr: jlong,
) -> jlong {
    let counter = &mut *(counter_ptr as *mut SessionConfig);
    counter.version() as jlong
}
