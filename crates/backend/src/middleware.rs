use bindings::Bindings;

use std::time::Duration;

use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{extract::Request, response::Response};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::UtcDateTime;

#[derive(Clone)]
pub(crate) struct MiddlewareState<B>
where
    B: Bindings,
{
    bindings: B,
}

impl<B> MiddlewareState<B>
where
    B: Bindings,
{
    pub fn new(bindings: B) -> Self {
        Self { bindings }
    }
}

#[derive(Serialize, Deserialize)]
struct Claims {
    pub exp: u64,
    pub iat: u64,
    pub user: String,
}

#[derive(Debug, Error)]
#[error("authentification error")]
pub(crate) enum AuthError {
    #[error("jwt not found in http headers")]
    JwtNotFound,
    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Secret(#[from] bindings::SecretError),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        StatusCode::FORBIDDEN.into_response()
    }
}

pub(crate) async fn auth<B>(
    State(state): State<MiddlewareState<B>>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, AuthError>
where
    B: Bindings,
{
    let jwt_secret = state
        .bindings
        .secret("JWT_SECRET")
        .map_err(AuthError::Secret)?;

    let auth_header = req.headers().get(header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(h) => h.to_str().map_err(|_| AuthError::JwtNotFound)?,
        None => return Err(AuthError::JwtNotFound),
    };

    let (_, token) = auth_header.split_once(' ').ok_or(AuthError::JwtNotFound)?;
    decode_jwt(token, &jwt_secret)?;

    Ok(next.run(req).await)
}

fn encode_jwt(user: String, secret: &str) -> Result<String, AuthError> {
    let now = UtcDateTime::now().unix_timestamp() as u64;
    let expire = now + Duration::from_hours(1).as_secs();

    let claims = Claims {
        exp: expire,
        iat: now,
        user,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(AuthError::Jwt)
}

fn decode_jwt(jwt: &str, secret: &str) -> Result<Claims, AuthError> {
    let token_data = decode(
        jwt,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(AuthError::Jwt)?;

    Ok(token_data.claims)
}
