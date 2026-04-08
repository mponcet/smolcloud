// Authentification is token-based (JWT).
//
// The typical workflow runs as follow :
// 1. User logs in. An access token and a refresh token are issued (A1 and R1).
// 2. User accesses ressources with access token A1.
// 3. Access token expires (A1).
// 4. Refresh token (R1) is used to get both a new access token and refresh token (A2 and R2).
// 5. Refresh token R1 is invalidated (added to a blacklist).
// 6. If R1 is reused, both R1 and R2 are invalidated. The (legitimate) client can't use R2 to
// issue a new token pair: re-authenfication is required.
// TODO:
// - if a token is re-used, blacklist this token and those issued after
// - password hashing
// - kv store flushing, ttl on expired tokens
// - use a different secret for access and refresh token
use crate::extract::jwt::ExtractRefreshToken;
use crate::jwt::{Audience, Claims};
use bindings::{Bindings, ExposeSecret, KvError, KvStore, SecretError};
use models::login::{LoginRequest, LoginResponse, TokenType};

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("bad username or password")]
    WrongCredentials,
    #[error(transparent)]
    Secret(#[from] SecretError),
    #[error(transparent)]
    KvStore(#[from] KvError),
    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("jwt not found in headers")]
    JwtNotFound,
    #[error("jwt revoked")]
    JwtRevoked,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::WrongCredentials => StatusCode::UNAUTHORIZED,
            AuthError::Secret(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::KvStore(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::Jwt(_) => StatusCode::UNAUTHORIZED,
            AuthError::JwtNotFound => StatusCode::UNAUTHORIZED,
            AuthError::JwtRevoked => StatusCode::UNAUTHORIZED,
            AuthError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
        .into_response()
    }
}

pub async fn login<B>(
    State(bindings): State<B>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AuthError>
where
    B: Bindings,
{
    let username_secret = bindings.secret("USERNAME").map_err(|e| {
        tracing::error!("USERNAME secret is missing");
        AuthError::Secret(e)
    })?;
    let password_secret = bindings.secret("PASSWORD").map_err(|e| {
        tracing::error!("PASSWORD secret is missing");
        AuthError::Secret(e)
    })?;
    let jwt_secret = bindings.secret("JWT_SECRET").map_err(|e| {
        tracing::error!("JWT_SECRET is missing");
        AuthError::Secret(e)
    })?;

    let (username, password) = (req.username, req.password);
    // TODO: hash password
    if username_secret.expose_secret() == username && password_secret.expose_secret() == password {
        let access_jwt = Claims::new(Audience::Access, username.clone())
            .encode(&jwt_secret)
            .map_err(AuthError::Jwt)?;
        let refresh_jwt = Claims::new(Audience::Refresh, username)
            .encode(&jwt_secret)
            .map_err(AuthError::Jwt)?;
        let response = LoginResponse {
            access_token: access_jwt,
            refresh_token: refresh_jwt,
            token_type: TokenType::Bearer,
        };
        Ok(Json(response))
    } else {
        Err(AuthError::WrongCredentials)
    }
}

pub async fn refresh_token<B>(
    State(bindings): State<B>,
    ExtractRefreshToken(claims): ExtractRefreshToken,
) -> Result<Json<LoginResponse>, AuthError>
where
    B: Bindings,
{
    let kv = bindings.kv();
    let jwt_secret = bindings.secret("JWT_SECRET").map_err(|e| {
        tracing::error!("JWT_SECRET is missing");
        AuthError::Secret(e)
    })?;

    if let Some(_child_jwt) = kv.get(&claims.jti).await.map_err(AuthError::KvStore)? {
        // Token is being re-used !
        return Err(AuthError::JwtRevoked);
    }

    let access_jwt = Claims::new(Audience::Access, claims.sub.clone())
        .encode(&jwt_secret)
        .map_err(AuthError::Jwt)?;
    let refresh_jwt = Claims::new(Audience::Refresh, claims.sub)
        .encode(&jwt_secret)
        .map_err(AuthError::Jwt)?;
    let response = LoginResponse {
        access_token: access_jwt,
        refresh_token: refresh_jwt,
        token_type: TokenType::Bearer,
    };

    // Blacklist previous refresh token.
    // TODO: set ttl, store the jti of newer token (chain old and new token) for future revocation.
    kv.put(&claims.jti, &[]).await.map_err(AuthError::KvStore)?;
    Ok(Json(response))
}
