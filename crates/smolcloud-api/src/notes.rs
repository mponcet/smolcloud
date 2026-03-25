use crate::BaseClient;
use models::notes::{Note, NoteId, NoteMetadata};

use anyhow::Result;

pub struct NotesApi(pub(crate) BaseClient);

impl std::ops::Deref for NotesApi {
    type Target = BaseClient;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NotesApi {
    pub async fn get(&self, id: NoteId) -> Result<Note> {
        Ok(self
            .http_client
            .get(self.base_url.join(&format!("notes/{id}"))?)
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn get_all(&self) -> Result<Vec<NoteMetadata>> {
        Ok(self
            .http_client
            .get(self.base_url.join("notes")?)
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn create(&self, note: Note) -> Result<NoteId> {
        Ok(self
            .http_client
            .post(self.base_url.join("notes")?)
            .json(&note)
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn update(&self, id: NoteId, note: Note) -> Result<()> {
        self.http_client
            .put(self.base_url.join(&format!("notes/{id}"))?)
            .json(&note)
            .send()
            .await?;

        Ok(())
    }

    pub async fn delete(&self, id: NoteId) -> Result<()> {
        self.http_client
            .delete(self.base_url.join(&format!("notes/{id}"))?)
            .send()
            .await?;

        Ok(())
    }
}
