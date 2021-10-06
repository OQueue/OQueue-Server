use std::future::Ready;
use std::marker::PhantomData;

use actix_web::dev::{Payload, PayloadStream, ServiceRequest};
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::web::Data;
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Eq, PartialEq, Clone, Hash, Serialize, Deserialize, Debug)]
pub struct Auth {
    pub id: Uuid,
    pub exp: u64,
}

impl FromRequest for Auth {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload<PayloadStream>) -> Self::Future {
        let auth = req
            .extensions_mut()
            .remove::<Auth>()
            .ok_or_else(|| ErrorUnauthorized(""));
        std::future::ready(auth)
    }
}

pub struct JwtConfig {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}

impl JwtConfig {
    pub fn default_from_base64_secret(secret: &str) -> Self {
        let encoding_key = EncodingKey::from_base64_secret(secret).unwrap();
        let decoding_key = DecodingKey::from_base64_secret(secret).unwrap();
        Self {
            encoding_key,
            decoding_key,
        }
    }
}

pub fn decode_token(token: &str, config: &JwtConfig) -> jsonwebtoken::errors::Result<Auth> {
    let JwtConfig { decoding_key, .. } = config;

    let mut validation = {
        let mut v = Validation::new(Algorithm::EdDSA);
        // TODO: use exp
        v.validate_exp = false;
        v
    };

    let token_data = jsonwebtoken::decode::<Auth>(token, decoding_key, &validation)?;

    Ok(token_data.claims)
}

pub fn encode_token(auth: &Auth, config: &JwtConfig) -> jsonwebtoken::errors::Result<String> {
    let JwtConfig { encoding_key, .. } = config;

    let token = jsonwebtoken::encode(&Header::default(), auth, encoding_key);

    token
}

pub async fn bearer_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    let config = req.app_data::<Data<JwtConfig>>().unwrap().get_ref();

    let auth =
        decode_token(credentials.token(), config).map_err(|_e| ErrorInternalServerError(""))?;

    req.extensions_mut().insert(auth);

    Ok(req)
}
