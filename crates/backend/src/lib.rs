mod models;
mod routes;
mod services;

use bindings::Bindings;
use routes::*;
use services::notes::NoteService;

use axum::Router;
use axum::routing::{delete, get, post, put};

pub fn router<B>(bindings: B) -> Router
where
    B: Bindings,
{
    let note_service = NoteService::new(bindings);

    Router::new()
        .route("/notes", get(notes::list::<B>))
        .route("/notes/{id}", get(notes::get::<B>))
        .route("/notes/{id}", put(notes::put::<B>))
        .route("/notes", post(notes::post::<B>))
        .route("/notes/{id}", delete(notes::delete::<B>))
        .with_state(note_service)
}
