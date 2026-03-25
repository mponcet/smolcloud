pub mod notes;

use notes::NotesApi;

use anyhow::Result;

#[derive(Clone)]
pub struct BaseClient {
    http_client: reqwest::Client,
    base_url: reqwest::Url,
}

impl BaseClient {
    pub fn try_new(base_url: &str) -> Result<Self> {
        Ok(Self {
            http_client: reqwest::Client::new(),
            base_url: base_url.try_into()?,
        })
    }

    pub fn notes_api(&self) -> NotesApi {
        NotesApi(self.clone())
    }
}
