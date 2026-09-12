pub mod notes;

use models::login::{LoginRequest, LoginResponse};
use notes::NotesApi;

use anyhow::{Result, anyhow};
use reqwest::header::{self, HeaderMap};
use secrecy::{ExposeSecret, SecretString};

#[derive(Clone)]
pub struct BaseClient {
    http_client: reqwest::Client,
    base_url: reqwest::Url,
    refresh_token: Option<SecretString>,
}

impl BaseClient {
    pub fn try_new(base_url: &str) -> Result<Self> {
        Ok(Self {
            http_client: reqwest::Client::new(),
            base_url: base_url.try_into()?,
            refresh_token: None,
        })
    }

    pub async fn login(self, username: String, password: String) -> Result<Self> {
        let response: LoginResponse = self
            .http_client
            .post(self.base_url.join("auth/login")?)
            .json(&LoginRequest { username, password })
            .send()
            .await?
            .json()
            .await
            .unwrap();

        let http_client = reqwest::ClientBuilder::new()
            .default_headers(HeaderMap::from_iter([(
                header::AUTHORIZATION,
                format!("Bearer {}", response.access_token).parse().unwrap(),
            )]))
            .build()?;

        Ok(Self {
            http_client,
            base_url: self.base_url,
            refresh_token: Some(response.refresh_token.into()),
        })
    }

    pub async fn refresh_token(self) -> Result<Self> {
        let refresh_token = self.refresh_token.ok_or(anyhow!("missing refresh token"))?;
        let response: LoginResponse = self
            .http_client
            .post(self.base_url.join("auth/refresh_token")?)
            .bearer_auth(refresh_token.expose_secret())
            .send()
            .await?
            .json()
            .await?;

        let http_client = reqwest::ClientBuilder::new()
            .default_headers(HeaderMap::from_iter([(
                header::AUTHORIZATION,
                format!("Bearer {}", response.access_token).parse().unwrap(),
            )]))
            .build()?;

        Ok(Self {
            http_client,
            base_url: self.base_url,
            refresh_token: Some(response.refresh_token.into()),
        })
    }

    pub fn notes_api(&self) -> NotesApi {
        NotesApi(self.clone())
    }
}
