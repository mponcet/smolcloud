use crate::error::ApiError;
use crate::models::notes::{Note, NoteId, NoteMetadata};
use crate::services::notes::NoteService;
use bindings::Bindings;

use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;

#[cfg_attr(feature = "cloudflare", worker::send)]
pub async fn get<B>(
    State(service): State<NoteService<B>>,
    Path(id): Path<NoteId>,
) -> Result<Json<Note>, ApiError>
where
    B: Bindings,
{
    service
        .get(id)
        .await
        .map(Json::from)
        .map_err(ApiError::from)
}

#[cfg_attr(feature = "cloudflare", worker::send)]
pub async fn list<B>(
    State(service): State<NoteService<B>>,
) -> Result<Json<Vec<NoteMetadata>>, ApiError>
where
    B: Bindings,
{
    service.list().await.map(Json::from).map_err(ApiError::from)
}

#[cfg_attr(feature = "cloudflare", worker::send)]
pub async fn post<B>(
    State(service): State<NoteService<B>>,
    Json(note): Json<Note>,
) -> Result<impl IntoResponse, ApiError>
where
    B: Bindings,
{
    service.create(note).await.map_err(ApiError::from)
}

#[cfg_attr(feature = "cloudflare", worker::send)]
pub async fn put<B>(
    State(service): State<NoteService<B>>,
    Path(id): Path<NoteId>,
    Json(note): Json<Note>,
) -> Result<impl IntoResponse, ApiError>
where
    B: Bindings,
{
    service.update(id, note).await.map_err(ApiError::from)
}

#[cfg_attr(feature = "cloudflare", worker::send)]
pub async fn delete<B>(
    State(service): State<NoteService<B>>,
    Path(id): Path<NoteId>,
) -> Result<impl IntoResponse, ApiError>
where
    B: Bindings,
{
    service.delete(id).await.map_err(ApiError::from)
}
