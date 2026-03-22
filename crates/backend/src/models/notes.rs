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
        HashMap::from([(metadata.id.to_string(), metadata.title)])
    }
}
