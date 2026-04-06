use crate::extract::jwt::ExtractAccessToken;
use crate::routes::auth::AuthError;
use bindings::Bindings;

use axum::extract::Request;
use axum::extract::{FromRequestParts, State};
use axum::middleware::Next;
use axum::response::IntoResponse;

pub async fn auth<B>(
    State(bindings): State<B>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, AuthError>
where
    B: Bindings,
{
    let (mut parts, body) = req.into_parts();
    let jwt = ExtractAccessToken::from_request_parts(&mut parts, &bindings).await?;

    tracing::debug!("jwt: {:?}", jwt.0);

    let req = Request::from_parts(parts, body);
    Ok(next.run(req).await)
}
