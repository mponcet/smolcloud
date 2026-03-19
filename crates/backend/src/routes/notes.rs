use crate::error::ApiError;
use crate::models::notes::Note;
use crate::routes::templates;
use crate::services::notes::NoteService;
use bindings::Bindings;

use axum::Form;
use axum::extract::{Path, State};
use axum::response::{Html, IntoResponse};
use serde::Deserialize;

pub async fn root() -> impl IntoResponse {
    Html(templates::notes::root())
}

#[cfg_attr(feature = "cloudflare", worker::send)]
pub async fn list_note<B>(State(service): State<NoteService<B>>) -> Result<Html<String>, ApiError>
where
    B: Bindings,
{
    service
        .list()
        .await
        .map(|notes| Html(templates::notes::list(notes)))
        .map_err(Into::into)
}

#[cfg_attr(feature = "cloudflare", worker::send)]
pub async fn get_note<B>(
    State(service): State<NoteService<B>>,
    Path(id): Path<u32>,
) -> Result<impl IntoResponse, ApiError>
where
    B: Bindings,
{
    service
        .get(id)
        .await
        .map(|note| note.content)
        .map_err(Into::into)
}

#[derive(Deserialize)]
pub struct NoteForm {
    name: String,
    content: String,
}

#[cfg_attr(feature = "cloudflare", worker::send)]
pub async fn post_note<B>(
    State(service): State<NoteService<B>>,
    Form(form): Form<NoteForm>,
) -> Result<impl IntoResponse, ApiError>
where
    B: Bindings,
{
    service
        .create(Note {
            name: form.name,
            content: form.content,
        })
        .await
        .map_err(Into::into)
}

#[cfg_attr(feature = "cloudflare", worker::send)]
pub async fn new_note() -> impl IntoResponse {
    Html(templates::notes::new())
}

#[cfg_attr(feature = "cloudflare", worker::send)]
pub async fn edit_note<B>(
    State(service): State<NoteService<B>>,
    Path(id): Path<u32>,
) -> Result<impl IntoResponse, ApiError>
where
    B: Bindings,
{
    service
        .get(id)
        .await
        .map(|note| templates::notes::edit(id, &note.content))
        .map_err(Into::into)
}
