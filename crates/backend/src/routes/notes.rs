use crate::error::ApiError;
use crate::services::notes::NoteService;
use bindings::Bindings;
use models::notes::{Note, NoteId, NoteMetadata};

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
pub async fn get_all<B>(
    State(service): State<NoteService<B>>,
) -> Result<Json<Vec<NoteMetadata>>, ApiError>
where
    B: Bindings,
{
    service
        .get_all()
        .await
        .map(Json::from)
        .map_err(ApiError::from)
}

#[cfg_attr(feature = "cloudflare", worker::send)]
pub async fn create<B>(
    State(service): State<NoteService<B>>,
    Json(note): Json<Note>,
) -> Result<impl IntoResponse, ApiError>
where
    B: Bindings,
{
    service
        .create(note)
        .await
        .map(Json::from)
        .map_err(ApiError::from)
}

#[cfg_attr(feature = "cloudflare", worker::send)]
pub async fn update<B>(
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
