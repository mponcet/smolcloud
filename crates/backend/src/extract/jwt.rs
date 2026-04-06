use crate::jwt::{self, Audience, Claims};
use crate::routes::auth::AuthError;

use axum::extract::FromRequestParts;
use axum::http;
use bindings::Bindings;

#[derive(Debug)]
pub struct ExtractAccessToken(pub Claims);

impl<B> FromRequestParts<B> for ExtractAccessToken
where
    B: Bindings,
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        bindings: &B,
    ) -> Result<Self, Self::Rejection> {
        let jwt_secret = bindings.secret("JWT_SECRET").map_err(|e| {
            tracing::error!("JWT_SECRET is missing");
            AuthError::Secret(e)
        })?;

        if let Some(header) = parts.headers.get(http::header::AUTHORIZATION)
            && let Ok(header) = header.to_str()
            && let Some((bearer, jwt)) = header.split_once(' ')
            && bearer == "Bearer"
        {
            let claims =
                jwt::Claims::decode(jwt, Audience::Access, &jwt_secret).map_err(AuthError::Jwt)?;
            Ok(ExtractAccessToken(claims))
        } else {
            Err(AuthError::JwtNotFound)
        }
    }
}

#[derive(Debug)]
pub struct ExtractRefreshToken(pub Claims);

impl<B> FromRequestParts<B> for ExtractRefreshToken
where
    B: Bindings,
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        bindings: &B,
    ) -> Result<Self, Self::Rejection> {
        let jwt_secret = bindings.secret("JWT_SECRET").map_err(|e| {
            tracing::error!("JWT_SECRET is missing");
            AuthError::Secret(e)
        })?;

        if let Some(header) = parts.headers.get(http::header::AUTHORIZATION)
            && let Ok(header) = header.to_str()
            && let Some((bearer, jwt)) = header.split_once(' ')
            && bearer.eq_ignore_ascii_case("Bearer")
        {
            let claims =
                jwt::Claims::decode(jwt, Audience::Refresh, &jwt_secret).map_err(AuthError::Jwt)?;
            Ok(ExtractRefreshToken(claims))
        } else {
            Err(AuthError::JwtNotFound)
        }
    }
}
