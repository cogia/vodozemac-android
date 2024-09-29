use std::error::Error;
use super::{CustomError, SessionConfig};

use vodozemac::megolm::{ExportedSessionKey, MegolmMessage, SessionKey};

pub struct GroupSession {
    pub(super) inner: vodozemac::megolm::GroupSession,
}

impl GroupSession {
    pub fn new(config: &SessionConfig) -> Self {
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
