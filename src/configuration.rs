use std::env;
use std::str::FromStr;
use jsonwebtoken::{DecodingKey, EncodingKey, Algorithm};

use crate::auth::JwtConfig;

pub fn env_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn env_host() -> Option<String> {
    env::var("HOST").ok()
}

pub fn env_encoding_key() -> String {
    env::var("JWT_ENCODING_KEY").expect("JWT_ENCODING_KEY must be set")
}

pub fn env_decoding_key() -> String {
    env::var("JWT_DECODING_KEY").expect("JWT_DECODING_KEY must be set")
}

pub fn env_jwt_algorithm() -> String {
    env::var("JWT_ALGORITHM").expect("JWT_DECODING_KEY must be set")
}

pub fn load_jwt_config() -> JwtConfig {
    let encoding_key = env_encoding_key();
    let decoding_key = env_decoding_key();
    let algorithm = env_jwt_algorithm();

    let encoding_key = EncodingKey::from_base64_secret(&encoding_key).unwrap();
    let decoding_key = DecodingKey::from_base64_secret(&decoding_key).unwrap();
    let algorithm = Algorithm::from_str(&algorithm).unwrap_or_default();

    JwtConfig {
        encoding_key,
        decoding_key,
        algorithm
    }
}
