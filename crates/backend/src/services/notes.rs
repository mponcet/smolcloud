use crate::models::notes::Note;
use crate::services::error::ServiceError;
use bindings::{
    Bindings, Bucket, BucketGetOptionsBuilder, BucketListOptionsBuilder, BucketPutOptionsBuilder,
};

#[derive(Clone)]
pub struct NoteService<B: Bindings> {
    bindings: B,
}

impl<B: Bindings> NoteService<B> {
    pub fn new(bindings: B) -> Self {
        Self { bindings }
    }

    pub async fn get(&self, id: u32) -> Result<Note, ServiceError> {
        let result = self
            .bindings
            .bucket()
            .get(&id.to_string())
            .execute()
            .await
            .map_err(|_| ServiceError::NotFound)?;

        if let Some(object) = result {
            Ok(Note {
                name: object.key().into(),
                content: String::from_utf8_lossy(object.body().unwrap_or(&[])).into_owned(),
            })
        } else {
            Err(ServiceError::NotFound)
        }
    }

    pub async fn create(&self, note: Note) -> Result<(), ServiceError> {
        self.bindings
            .bucket()
            .put(&note.name, note.content.as_bytes())
            .execute()
            .await
            .map_err(ServiceError::BucketError)?;
        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<String>, ServiceError> {
        let result = self
            .bindings
            .bucket()
            .list()
            .execute()
            .await
            .map_err(ServiceError::BucketError)?;

        Ok(result
            .into_iter()
            .map(|object| object.key().into())
            .collect())
    }

    pub async fn delete(&self, name: &str) -> Result<(), ServiceError> {
        self.bindings
            .bucket()
            .delete(name)
            .await
            .map_err(ServiceError::BucketError)?;
        Ok(())
    }
}
