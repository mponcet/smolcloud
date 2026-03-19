mod models;
mod routes;
mod services;

use bindings::Bindings;
use routes::*;
use services::notes::NoteService;

use axum::Router;
use axum::routing::{get, post, put};

pub fn router<B>(bindings: B) -> Router
where
    B: Bindings,
{
    let note_service = NoteService::new(bindings);

    Router::new()
        .route("/", get(notes::root))
        .route("/notes", get(notes::list_note::<B>))
        .route("/notes/{id}", get(notes::get_note::<B>))
        .route("/notes/{id}", put(notes::post_note::<B>))
        .route("/notes/new", get(notes::new_note))
        .route("/notes", post(notes::post_note::<B>))
        .route("/notes/{id}/edit", get(notes::edit_note::<B>))
        .with_state(note_service)
}
