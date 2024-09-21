
// This is the interface to the JVM that we'll call the majority of our
// methods on.
//use jni::*;

use jni::JNIEnv;
use jni::objects::{JClass, JObject};

#[no_mangle]
pub unsafe extern "C" fn Java_com_cogia_vodozemac_internal_Native_keepAlive(
    _env: JNIEnv,
    _class: JClass,
    _obj: JObject,
) {
}

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
