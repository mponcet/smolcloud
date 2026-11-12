use axum::http::header::ALLOW;
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};

pub async fn handler() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header(ALLOW, HeaderValue::from_static("GET, HEAD, OPTIONS"))
        .header("DAV", "1")
        .body(axum::body::Body::empty())
        .unwrap()
}
