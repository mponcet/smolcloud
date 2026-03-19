use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error(transparent)]
    BucketError(#[from] bindings::BucketError),
    #[error("could not find the request resource")]
    NotFound,
}
