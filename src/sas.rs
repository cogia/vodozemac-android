use std::error::Error;
use jni::JNIEnv;
use jni::objects::{JClass};
use jni::sys::{jlong, jstring};

pub struct Sas {
    inner: vodozemac::sas::Sas,
}


impl Sas {
    pub fn new() -> Self {
        Self {
            inner: vodozemac::sas::Sas::new(),
        }
    }

    pub fn public_key(&self) -> String {
        self.inner.public_key().to_base64()
    }

    pub fn diffie_hellman(self, key: String) -> Result<EstablishedSas, Box<dyn Error>> {
        let sass = self.inner.diffie_hellman_with_raw(&key)
            .map_err(|err: _| Box::new(err) as Box<dyn Error>)?;

        Ok(EstablishedSas { inner: sass })
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

/*#[no_mangle]
pub extern "C" fn Java_de_cogia_vodozemac_OlmSas__1diffie_1hellman(
    mut env: JNIEnv,
    _class: JClass,
    my_ptr: jlong,
    key: JString,
) -> jlong {
    let sas = unsafe { &mut *(my_ptr as *mut Sas) };

    let localKey = jstring_to_string(&mut env, key);
    let established = sas.diffie_hellman(localKey).unwrap();
    Box::into_raw(Box::new(established)) as jlong

    /// ^^^^^^^^^^ move occurs because `self.inner` has type `vodozemac::sas::Sas`, which does not implement the `Copy` trait
}*/
// diffie_hellman