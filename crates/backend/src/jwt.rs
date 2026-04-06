use bindings::{ExposeSecret, SecretString};

use std::time::Duration;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, errors::Error};
use serde::{Deserialize, Serialize};
use time::UtcDateTime;

pub enum Audience {
    Access,
    Refresh,
}

impl From<Audience> for String {
    fn from(aud: Audience) -> Self {
        match aud {
            Audience::Access => "access".into(),
            Audience::Refresh => "refresh".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub aud: String,
    pub exp: u64,
    pub iat: u64,
    pub jti: String,
}

impl Claims {
    pub fn new(audience: Audience, subject: String) -> Self {
        let now = UtcDateTime::now().unix_timestamp() as u64;

        let expire = match audience {
            Audience::Access => now + Duration::from_secs(600).as_secs(),
            Audience::Refresh => now + Duration::from_hours(12).as_secs(),
        };

        Self {
            sub: subject,
            aud: audience.into(),
            exp: expire,
            iat: now,
            jti: uuid::Uuid::new_v4().into(),
        }
    }

    pub fn encode(&self, secret: &SecretString) -> Result<String, Error> {
        jsonwebtoken::encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.expose_secret().as_bytes()),
        )
    }

    pub fn decode(jwt: &str, audience: Audience, secret: &SecretString) -> Result<Self, Error> {
        let mut validation = Validation::default();
        validation.set_audience::<String>(&[audience.into()]);
        let token_data = jsonwebtoken::decode(
            jwt,
            &DecodingKey::from_secret(secret.expose_secret().as_bytes()),
            &validation,
        )?;

        Ok(token_data.claims)
    }
}
