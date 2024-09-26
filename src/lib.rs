#[allow(unused_variables)]
// This is the interface to the JVM that we'll call the majority of our
// methods on.
//use jni::*;

use jni::JNIEnv;
use jni::objects::{JClass, JObject};
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


// pub extern "C" fn
#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_SessionConfig_version1() -> jlong {
    Box::into_raw(Box::new(SessionConfig::version_1())) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_cogia_vodozemac_SessionConfig_version(mut env: JNIEnv,
                                                                       _class: JClass,
                                                                       counter_ptr: jlong,
) -> jlong {
    let counter = &mut *(counter_ptr as *mut SessionConfig);
    counter.version() as jlong
}