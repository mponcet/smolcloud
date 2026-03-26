use thiserror::Error;

#[derive(Error, Debug)]
#[error("kv error")]
pub struct KvError {
    pub message: String,
    #[source]
    pub source: anyhow::Error,
}

pub trait KvStore {
    fn get(&self, key: &str) -> impl Future<Output = Result<Option<Vec<u8>>, KvError>>;
    fn put(&self, key: &str, value: &[u8]) -> impl Future<Output = Result<(), KvError>>;
    fn delete(&self, key: &str) -> impl Future<Output = Result<(), KvError>>;
}
