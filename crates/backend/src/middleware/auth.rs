use crate::extract::jwt::ExtractAccessToken;
use crate::routes::auth::AuthError;
use bindings::Bindings;

use axum::extract::Request;
use axum::extract::{FromRequestParts, State};
use axum::middleware::Next;
use axum::response::IntoResponse;

pub async fn jwt_auth<B>(
    State(bindings): State<B>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, AuthError>
where
    B: Bindings,
{
    let (mut parts, body) = req.into_parts();
    let jwt = ExtractAccessToken::from_request_parts(&mut parts, &bindings).await?;

    tracing::debug!(
        subject = jwt.0.sub,
        iat = ?time::UtcDateTime::from_unix_timestamp(i64::try_from(jwt.0.iat).unwrap()).unwrap(),
        exp = ?time::UtcDateTime::from_unix_timestamp(i64::try_from(jwt.0.exp).unwrap()).unwrap()
    );

    let req = Request::from_parts(parts, body);
    Ok(next.run(req).await)
}
