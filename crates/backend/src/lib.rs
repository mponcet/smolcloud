pub mod config;
mod extract;
mod jwt;
mod middleware;
mod routes;
mod services;

use bindings::Bindings;
use routes::*;
use services::notes::NoteService;

use axum::Router;
use axum::routing::{delete, get, post, put};

// async fn log_app_errors(request: Request, next: Next) -> Response {
//     let response = next.run(request).await;
//
//     tracing::debug!("error handler = {response:?}");
//     // If the response contains an AppError Extension, log it.
//     if let Some(err) = response.extensions().get::<ApiError>() {
//         tracing::debug!("error handler: error !");
//         tracing::error!(?err, "an unexpected error occurred inside a handler");
//     }
//     response
// }
// .layer(axum::middleware::from_fn(log_app_errors))

pub fn router<B>(bindings: B) -> Router
where
    B: Bindings,
{
    let note_service = NoteService::new(bindings.clone());

    let auth_router = Router::new()
        .route("/login", post(auth::login::<B>))
        .route("/refresh_token", post(auth::refresh_token::<B>))
        .with_state(bindings.clone());

    let notes_router = Router::new()
        .route("/", get(notes::get_all::<B>))
        .route("/{id}", get(notes::get::<B>))
        .route("/{id}", put(notes::update::<B>))
        .route("/", post(notes::create::<B>))
        .route("/{id}", delete(notes::delete::<B>))
        .layer(axum::middleware::from_fn_with_state(
            bindings,
            middleware::auth::<B>,
        ))
        .with_state(note_service);

    Router::new()
        .nest("/auth", auth_router)
        .nest("/notes", notes_router)
}
