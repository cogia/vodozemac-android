use std::error::Error;
use jni::JNIEnv;
use jni::objects::{JClass, JLongArray, JString};
use jni::sys::{jboolean, jlong, jstring};
use crate::{jstring_to_string, result_or_java_exception, CustomError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Curve25519PublicKey(pub(crate) vodozemac::Curve25519PublicKey);

impl Curve25519PublicKey {
    pub fn from_base64(key: &str) -> Result<Box<Curve25519PublicKey>, Box<dyn Error>> {
        Ok(Curve25519PublicKey(vodozemac::Curve25519PublicKey::from_base64(key).map_err(|err: _| Box::new(err) as Box<dyn Error>)?).into())
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_base64(&self) -> String {
        self.0.to_base64()
    }
}
pub struct Sas {
    inner: Option<vodozemac::sas::Sas>,
}


impl Sas {
    pub fn new() -> Self {
        Self {
            inner: Some(vodozemac::sas::Sas::new()),
        }
    }

    pub fn public_key(&mut self) -> String {
        if let Some(sas) = self.inner.take() {
            return sas.public_key().to_base64();
        }
        return String::new();
    }

    pub fn diffie_hellman(&mut self, key: String) -> Result<EstablishedSas, Box<dyn Error>> {
        if let Some(sas) = self.inner.take() {
            let pub_key = Curve25519PublicKey::from_base64(&key).unwrap();
            let sass = sas.diffie_hellman(pub_key.0)
                .map_err(|err: _| Box::new(err) as Box<dyn Error>)?;
            Ok(EstablishedSas { inner: sass })
        } else {
            Err(Box::new(CustomError("Invalid message type, expected a pre-key message".to_owned())))
        }
    }
}

pub struct EstablishedSas {
    inner: vodozemac::sas::EstablishedSas,
}

impl EstablishedSas {
    pub fn bytes(&self, info: String) -> SasBytes {
        let bytes = self.inner.bytes(&info);
        SasBytes { inner: bytes }
    }

    pub fn calculate_mac(&self, input: String, info: String) -> String {
        self.inner.calculate_mac(&input, &info).to_base64()
    }

    pub fn calculate_mac_invalid_base64(&self, input: String, info: String) -> String {
        self.inner.calculate_mac_invalid_base64(&input, &info)
    }

    pub fn verify_mac(&self, input: String, info: String, tag: String) -> Result<(), Box<dyn Error>> {
        let tag = vodozemac::sas::Mac::from_base64(&tag)
            .map_err(|err: _| Box::new(err) as Box<dyn Error>)?;

        self.inner
            .verify_mac(&input, &info, &tag)
            .map_err(|err: _| Box::new(err) as Box<dyn Error>)?;

        Ok(())
    }
}

pub struct SasBytes {
    inner: vodozemac::sas::SasBytes,
}

impl SasBytes {
    pub fn emoji_indices(&self) -> Vec<u8> {
        self.inner.emoji_indices().to_vec()
    }

    pub fn decimals(&self) -> Vec<u16> {
        let (first, second, third) = self.inner.decimals();

        [first, second, third].to_vec()
    }
}


#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmSas__1new() -> jlong {
    Box::into_raw(Box::new(Sas::new())) as jlong
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmSas__1public_1key(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
) -> jstring {
    let sas = unsafe { &mut *(my_ptr as *mut Sas) };

    // Convert the output Rust String to a new jstring and return it
    let output_jstring: jstring = **env
        .new_string(sas.public_key())
        .expect("Failed to create output ed25519_key");

    output_jstring
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmSas__1diffie_1hellman(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
    key: JString,
) -> jlong {
    let sas = unsafe { &mut *(my_ptr as *mut Sas) };
    let localKey = jstring_to_string(&mut env, key);

    let res;
    match result_or_java_exception(&mut env, sas.diffie_hellman(localKey)) {
        Ok(value) => {
            res =  Box::into_raw(Box::new(value)) as jlong
        }
        Err(_) => {
            res = -1;
        }
    }
    res
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmEstablishedSas__1bytes(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
    info: JString,
) -> jlong {
    let sas = unsafe { &mut *(my_ptr as *mut EstablishedSas) };
    let localKey = jstring_to_string(&mut env, info);
    let established = sas.bytes(localKey);
    Box::into_raw(Box::new(established)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmEstablishedSas__1calculate_1mac(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
    input: JString,
    info: JString,
) -> jstring {
    let sas = unsafe { &mut *(my_ptr as *mut EstablishedSas) };
    let local_input = jstring_to_string(&mut env, input);
    let local_key = jstring_to_string(&mut env, info);

    let output_jstring: jstring = **env
        .new_string(sas.calculate_mac(local_input, local_key))
        .expect("Failed to create output ed25519_key");

    output_jstring
}

#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmEstablishedSas__1calculate_1mac_1invalid_1sbase64(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
    input: JString,
    info: JString,
) -> jstring {
    let sas = unsafe { &mut *(my_ptr as *mut EstablishedSas) };
    let local_input = jstring_to_string(&mut env, input);
    let local_key = jstring_to_string(&mut env, info);

    let output_jstring: jstring = **env
        .new_string(sas.calculate_mac_invalid_base64(local_input, local_key))
        .expect("Failed to create output ed25519_key");

    output_jstring
}


#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmEstablishedSas__1verify_1mac(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
    input: JString,
    info: JString,
    tag: JString
) -> jboolean {
    let sas = unsafe { &mut *(my_ptr as *mut EstablishedSas) };
    let local_input = jstring_to_string(&mut env, input);
    let local_key = jstring_to_string(&mut env, info);
    let local_tag = jstring_to_string(&mut env, tag);


    let res;
    match result_or_java_exception(&mut env, sas.verify_mac(local_input, local_key, local_tag)) {
        Ok(value) => {
            res =  true
        }
        Err(_) => {
            res = false;
        }
    }

    res
}



#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmSasBytes__1decimals<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    my_ptr: jlong,
) -> JLongArray<'local> {
    let sas = unsafe { &mut *(my_ptr as *mut SasBytes) };
    let long_vec: Vec<jlong> =  sas.decimals().iter().map(|&x| x as jlong).collect();
    let long_array = env.new_long_array(long_vec.len() as i32).unwrap();
    env.set_long_array_region(&long_array, 0, &long_vec).unwrap();
    long_array
}


#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmSasBytes__1emoji_1indices<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    my_ptr: jlong,
) -> JLongArray<'local> {
    let sas = unsafe { &mut *(my_ptr as *mut SasBytes) };
    let long_vec: Vec<jlong> =  sas.emoji_indices().iter().map(|&x| x as jlong).collect();
    let long_array = env.new_long_array(long_vec.len() as i32).unwrap();
    env.set_long_array_region(&long_array, 0, &long_vec).unwrap();
    long_array
}
