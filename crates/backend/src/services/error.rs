use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error(transparent)]
    Bucket(#[from] bindings::BucketError),
    #[error("could not find the request resource")]
    NotFound,
    #[error("internal error")]
    Internal,
}
