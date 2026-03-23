use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type NoteId = Uuid;

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NoteMetadata {
    pub id: NoteId,
    pub title: String,
}

impl NoteMetadata {
    pub fn new(id: NoteId, title: String) -> Self {
        Self { id, title }
    }
}

impl From<NoteMetadata> for HashMap<String, String> {
    fn from(metadata: NoteMetadata) -> Self {
        HashMap::from([
            ("id".into(), metadata.id.to_string()),
            ("title".into(), metadata.title),
        ])
    }
}

impl TryFrom<HashMap<String, String>> for NoteMetadata {
    type Error = &'static str;

    fn try_from(metadata: HashMap<String, String>) -> Result<Self, Self::Error> {
        let id = metadata.get("id").ok_or("missing metadata id")?;
        let id = NoteId::try_parse(id).map_err(|_| "could not parse uuid")?;
        let title = metadata
            .get("title")
            .ok_or("missing title metadata")?
            .to_string();

        Ok(NoteMetadata::new(id, title))
    }
}
