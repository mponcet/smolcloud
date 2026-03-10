use axum::Router;
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};

use bindings::{Bindings, Bucket};

pub fn router<B: Bindings>(bindings: B) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/note/{id}", get(get_note::<B>))
        .route("/note/{id}", post(post_note::<B>))
        .with_state(bindings)
}

pub async fn root() -> &'static str {
    "Nothing here."
}

#[cfg_attr(feature = "cloudflare", worker::send)]
pub async fn get_note<B>(Path(id): Path<u32>, State(bindings): State<B>) -> String
where
    B: Bindings,
{
    let bucket = bindings.bucket();
    bucket.get(id.to_string()).await
}

#[cfg_attr(feature = "cloudflare", worker::send)]
pub async fn post_note<B>(
    State(bindings): State<B>,
    Path(id): Path<u32>,
    mut multipart: Multipart,
) -> Result<(), (StatusCode, String)>
where
    B: Bindings,
{
    let bucket = bindings.bucket();

    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
    {
        while let Some(chunk) = field
            .chunk()
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
        {
            tracing::info!("received {} bytes", chunk.len());
            bucket
                .put(id.to_string(), String::from_utf8_lossy(&chunk).into_owned())
                .await;
        }
    }

    Ok(())
}
