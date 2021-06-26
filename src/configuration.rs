use std::env;

use crate::auth::JwtConfig;

pub fn env_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn env_encoding_key() -> String {
    env::var("JWT_ENCODING_KEY").expect("JWT_ENCODING_KEY must be set")
}

pub fn env_decoding_key() -> String {
    env::var("JWT_DECODING_KEY").expect("JWT_DECODING_KEY must be set")
}

pub fn load_jwt_config() -> JwtConfig {
    let encoding_key = env_encoding_key();
    let decoding_key = env_decoding_key();
    assert_eq!(encoding_key, decoding_key);
    JwtConfig::default_from_base64_secret(&encoding_key)
}
