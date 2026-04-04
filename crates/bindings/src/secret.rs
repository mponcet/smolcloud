use thiserror::Error;

#[derive(Error, Debug)]
#[error("secret error")]
pub struct SecretError {
    pub message: String,
    #[source]
    pub source: anyhow::Error,
}

pub type SecretFn = dyn Fn(&str) -> Result<String, SecretError> + Sync + Send + 'static;

