use crate::services::error::ServiceError;
use bindings::{
    Bindings, Bucket, BucketGetOptionsBuilder, BucketListOptionsBuilder, BucketObject,
    BucketPutOptionsBuilder,
};
use models::notes::{Note, NoteId, NoteMetadata};

#[derive(Clone)]
pub struct NoteService<B: Bindings> {
    bindings: B,
}

impl<B: Bindings> NoteService<B> {
    pub fn new(bindings: B) -> Self {
        Self { bindings }
    }

    pub async fn get(&self, id: NoteId) -> Result<Note, ServiceError> {
        let result = self
            .bindings
            .bucket()
            .get(&id.to_string())
            .execute()
            .await
            .map_err(|_| ServiceError::NotFound)?;

        if let Some(object) = result {
            let BucketObject {
                body,
                custom_metadata,
                ..
            } = object;
            let custom_metadata = custom_metadata.ok_or(ServiceError::Internal)?;

            let title = custom_metadata
                .get("title")
                .ok_or(ServiceError::Internal)?
                .clone();
            let content = body.and_then(|b| {
                String::from_utf8(b)
                    .map_err(|e| {
                        tracing::debug!("invalid note content: {e}");
                    })
                    .ok()
            });
            Ok(Note { title, content })
        } else {
            Err(ServiceError::NotFound)
        }
    }

    pub async fn get_all(&self) -> Result<Vec<NoteMetadata>, ServiceError> {
        let result = self
            .bindings
            .bucket()
            .list()
            .include_custom_metadata()
            .execute()
            .await
            .map_err(ServiceError::Bucket)?;

        Ok(result
            .into_iter()
            .filter_map(|object| {
                let metadata = object.custom_metadata?;
                match NoteMetadata::try_from(metadata) {
                    Ok(metadata) => Some(metadata),
                    Err(e) => {
                        tracing::debug!("object '{}': {e}", object.key);
                        None
                    }
                }
            })
            .collect::<Vec<_>>())
    }

    pub async fn create(&self, note: Note) -> Result<NoteId, ServiceError> {
        let id = NoteId::now_v7();
        let metadata = NoteMetadata::new(id, note.title);
        let content = note.content.unwrap_or_default();

        self.bindings
            .bucket()
            .put(&id.to_string(), content.as_bytes())
            .custom_metadata(metadata.into())
            .execute()
            .await
            .map_err(ServiceError::Bucket)?;

        Ok(id)
    }

    pub async fn update(&self, id: NoteId, note: Note) -> Result<(), ServiceError> {
        if self
            .bindings
            .bucket()
            .head(&id.to_string())
            .await
            .map_err(ServiceError::Bucket)?
            .is_none()
        {
            return Err(ServiceError::NotFound);
        }

        let metadata = NoteMetadata::new(id, note.title);
        self.bindings
            .bucket()
            .put(&id.to_string(), note.content.unwrap_or_default().as_bytes())
            .custom_metadata(metadata.into())
            .execute()
            .await
            .map_err(ServiceError::Bucket)?;

        Ok(())
    }

    pub async fn delete(&self, id: NoteId) -> Result<(), ServiceError> {
        self.bindings
            .bucket()
            .delete(&id.to_string())
            .await
            .map_err(ServiceError::Bucket)?;

        Ok(())
    }
}
