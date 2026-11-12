mod options;

use axum::Router;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::any;
use bindings::Bindings;

async fn webdav_router<B>(State(state): State<B>, req: Request) -> impl IntoResponse
where
    B: Bindings,
{
    tracing::info!("METHOD: {}", req.method());
    match req.method().as_str() {
        "OPTIONS" => options::handler().await.into_response(),
        _ => StatusCode::METHOD_NOT_ALLOWED.into_response(),
    }
}

pub fn router<B>(bindings: B) -> Router
where
    B: Bindings,
{
    Router::new()
        .route("/", any(webdav_router::<B>))
        .route("/{*path}", any(webdav_router::<B>))
        .with_state(bindings)
}
