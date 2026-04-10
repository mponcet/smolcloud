use thiserror::Error;

#[derive(Error, Debug)]
#[error("kv error")]
pub struct KvError {
    pub message: String,
    #[source]
    pub source: anyhow::Error,
}

pub trait KvPutOptionsBuilder {
    fn expiration(self, utc_timestamp: u64) -> Self;
    fn execute(self) -> impl Future<Output = Result<(), KvError>> + Send;
}

pub trait KvStore: Send + Sync {
    fn get(&self, key: &str) -> impl Future<Output = Result<Option<Vec<u8>>, KvError>> + Send;
    // fn put(&self, key: &str, value: &[u8]) -> impl Future<Output = Result<(), KvError>> + Send;
    fn put(&self, key: &str, value: &[u8]) -> impl KvPutOptionsBuilder;
    fn delete(&self, key: &str) -> impl Future<Output = Result<(), KvError>> + Send;
}

// impl<K: KvStore + ?Sized> KvStore for &K {
//     fn get(&self, key: &str) -> impl Future<Output = Result<Option<Vec<u8>>, KvError>> + Send {
//         (*self).get(key)
//     }
//
//     fn put(&self, key: &str, value: &[u8]) -> impl KvPutOptionsBuilder {
//         (*self).put(key, value)
//     }
//
//     fn delete(&self, key: &str) -> impl Future<Output = Result<(), KvError>> + Send {
//         (*self).delete(key)
//     }
// }
