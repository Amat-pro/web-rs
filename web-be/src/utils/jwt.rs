use crate::config::CONFIG;
use jsonwebtoken::{
    decode, encode, errors::Result, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Serialize};

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = CONFIG.get_security_config().get_jwt_config().get_secret();
    Keys::new(secret.as_bytes())
});

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub fn generate_token<T: Serialize>(data: &T) -> Result<String> {
    encode(&Header::default(), data, &KEYS.encoding)
}

pub fn decode_token<T: DeserializeOwned>(token: &str) -> Result<TokenData<T>> {
    decode(token, &KEYS.decoding, &Validation::default())
}