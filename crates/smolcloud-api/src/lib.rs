pub mod notes;

use models::login::{LoginRequest, LoginResponse};
use notes::NotesApi;

use anyhow::Result;
use reqwest::header::HeaderMap;
use secrecy::SecretString;

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
            .default_headers(HeaderMap::from_iter(
                [(
                    reqwest::header::AUTHORIZATION,
                    format!("Bearer {}", response.access_token).parse().unwrap(),
                )]
                .into_iter(),
            ))
            .build()?;

        Ok(Self {
            http_client,
            base_url: self.base_url,
            refresh_token: Some(response.refresh_token.into()),
        })
    }

    // pub async fn refresh_token(&mut self) -> Result<Self> {
    //     let response: LoginResponse = self
    //         .http_client
    //         .post(self.base_url.join("auth/refresh_token")?)
    //         .json(&())
    //         .send()
    //         .await?
    //         .json()
    //         .await?;
    //
    //     self.refresh_token = Some(response.refresh_token.into());
    //
    //     self.http_client = reqwest::ClientBuilder::new()
    //         .default_headers(HeaderMap::from_iter(
    //             [(
    //                 reqwest::header::AUTHORIZATION,
    //                 HeaderValue::from_str("here will go the access token").unwrap(),
    //             )]
    //             .into_iter(),
    //         ))
    //         .build()?;
    //
    //     Ok(Self {
    //     })
    // }

    pub fn notes_api(&self) -> NotesApi {
        NotesApi(self.clone())
    }
}
