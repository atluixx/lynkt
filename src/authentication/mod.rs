use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode,
    errors::{Error, ErrorKind},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_time: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JwtAuth {
    pub config: JwtConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(dead_code)]
pub enum VerifyStatus {
    Valid(Claims),
    Expired,
    Invalid,
}

impl JwtConfig {
    pub fn new<T: Into<String>>(secret: T, expiration_time: i64) -> Self {
        Self {
            secret: secret.into(),
            expiration_time,
        }
    }

    pub fn decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.secret.as_ref())
    }

    pub fn encoding_key(&self) -> EncodingKey {
        EncodingKey::from_secret(self.secret.as_ref())
    }
}

impl JwtAuth {
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }

    pub fn encode<T: Into<String>>(&self, sub: T) -> Result<String, Error> {
        let iat = Utc::now().timestamp() as usize;
        let exp =
            (Utc::now() + Duration::minutes(self.config.expiration_time)).timestamp() as usize;

        let claims = Claims {
            sub: sub.into(),
            iat,
            exp,
        };

        encode(&Header::default(), &claims, &self.config.encoding_key())
    }

    pub fn decode(&self, token: &str) -> Result<Claims, Error> {
        let validation = Validation::new(Algorithm::HS256);

        let token_data = decode::<Claims>(token, &self.config.decoding_key(), &validation)?;

        Ok(token_data.claims)
    }

    #[allow(dead_code)]
    pub fn verify(&self, token: &str) -> VerifyStatus {
        match self.decode(token) {
            Ok(c) => VerifyStatus::Valid(c),
            Err(e) => match e.kind() {
                ErrorKind::ExpiredSignature => VerifyStatus::Expired,
                _ => VerifyStatus::Invalid,
            },
        }
    }
}
