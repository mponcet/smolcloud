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
use crate::extract::jwt::ExtractRefreshToken;
use crate::jwt::{Audience, Claims};
use argon2::password_hash::{Encoding, PasswordHash};
use argon2::{Argon2, PasswordVerifier};
use bindings::{Bindings, ExposeSecret, KvError, KvPutOptionsBuilder, KvStore, SecretError};
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
    let access_jwt_secret = bindings.secret("ACCESS_JWT_SECRET").map_err(|e| {
        tracing::error!("ACCESS_JWT_SECRET is missing");
        AuthError::Secret(e)
    })?;
    let refresh_jwt_secret = bindings.secret("REFRESH_JWT_SECRET").map_err(|e| {
        tracing::error!("REFRESH_JWT_SECRET is missing");
        AuthError::Secret(e)
    })?;

    let argon2 = Argon2::default();
    let password_hash =
        PasswordHash::parse(password_secret.expose_secret(), Encoding::B64).unwrap();

    if username_secret.expose_secret() == req.username
        && argon2
            .verify_password(req.password.as_bytes(), &password_hash)
            .is_ok()
    {
        let access_jwt = Claims::new(Audience::Access, req.username.clone())
            .encode(&access_jwt_secret)
            .map_err(AuthError::Jwt)?;
        let refresh_jwt_claims = Claims::new(Audience::Refresh, req.username);
        let refresh_jwt = refresh_jwt_claims
            .encode(&refresh_jwt_secret)
            .map_err(AuthError::Jwt)?;

        // Whitelist refresh token.
        let kv = bindings.kv();
        kv.put(&refresh_jwt_claims.jti, &[])
            .expiration(refresh_jwt_claims.exp)
            .execute()
            .await
            .map_err(AuthError::KvStore)?;

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
    let refresh_jwt_secret = bindings.secret("REFRESH_JWT_SECRET").map_err(|e| {
        tracing::error!("REFRESH_JWT_SECRET is missing");
        AuthError::Secret(e)
    })?;

    // Remove refresh token from whitelist.
    // FIXME: potential race condition. Two refresh tokens could be issued simultaneously
    // if two concurrent requests check the whitelist at the same time.
    // On Cloudflare, calling `delete()` on a non-existing key is a successful operation
    // so it won't prevent the race condition.
    if kv.get(&claims.jti).await?.is_none() {
        return Err(AuthError::JwtRevoked);
    }
    kv.delete(&claims.jti).await.map_err(AuthError::KvStore)?;

    let new_access_jwt = Claims::new(Audience::Access, claims.sub.clone())
        .encode(&refresh_jwt_secret)
        .map_err(AuthError::Jwt)?;
    let new_refresh_jwt_claims = Claims::new(Audience::Refresh, claims.sub);
    let new_refresh_jwt = new_refresh_jwt_claims
        .encode(&refresh_jwt_secret)
        .map_err(AuthError::Jwt)?;
    let response = LoginResponse {
        access_token: new_access_jwt,
        refresh_token: new_refresh_jwt,
        token_type: TokenType::Bearer,
    };

    // Whitelist new refresh token.
    kv.put(&new_refresh_jwt_claims.jti, &[])
        .expiration(new_refresh_jwt_claims.exp)
        .execute()
        .await
        .map_err(AuthError::KvStore)?;

    Ok(Json(response))
}
