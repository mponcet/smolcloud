use std::collections::HashMap;
use std::future::Future;
use std::ops::Range;

use thiserror::Error;

#[derive(Error, Debug)]
#[error("bucket error")]
pub struct BucketError {
    pub message: String,
    #[source]
    pub source: anyhow::Error,
}

pub trait BucketGetOptionsBuilder {
    fn range(self, range: Range<usize>) -> Self;
    fn execute(self) -> impl Future<Output = Result<Option<BucketObject>, BucketError>> + Send;
}

pub trait BucketPutOptionsBuilder {
    fn custom_metadata(self, metadata: HashMap<String, String>) -> Self;
    fn execute(self) -> impl Future<Output = Result<BucketObject, BucketError>> + Send;
}

pub trait BucketListOptionsBuilder {
    fn limit(self, limit: u32) -> Self;
    fn prefix(self, prefix: &str) -> Self;
    fn include_custom_metadata(self) -> Self;
    fn execute(self) -> impl Future<Output = Result<Vec<BucketObject>, BucketError>> + Send;
}

pub struct BucketObject {
    pub key: String,
    pub body: Option<Vec<u8>>,
    pub custom_metadata: Option<HashMap<String, String>>,
}

impl BucketObject {
    pub fn key(&self) -> &str {
        self.key.as_str()
    }

    pub fn body(&self) -> Option<&[u8]> {
        self.body.as_deref()
    }
}

pub trait Bucket: Send + Sync {
    fn head(
        &self,
        key: &str,
    ) -> impl Future<Output = Result<Option<BucketObject>, BucketError>> + Send;
    fn get(&self, key: &str) -> impl BucketGetOptionsBuilder;
    fn put(&self, key: &str, data: &[u8]) -> impl BucketPutOptionsBuilder;
    fn delete(&self, key: &str) -> impl Future<Output = Result<(), BucketError>> + Send;
    fn list(&self) -> impl BucketListOptionsBuilder;
}

impl<B: Bucket + ?Sized> Bucket for &B {
    fn head(
        &self,
        key: &str,
    ) -> impl Future<Output = Result<Option<BucketObject>, BucketError>> + Send {
        (*self).head(key)
    }

    fn get(&self, key: &str) -> impl BucketGetOptionsBuilder {
        (*self).get(key)
    }

    fn put(&self, key: &str, data: &[u8]) -> impl BucketPutOptionsBuilder {
        (*self).put(key, data)
    }

    fn delete(&self, key: &str) -> impl Future<Output = Result<(), BucketError>> + Send {
        (*self).delete(key)
    }

    fn list(&self) -> impl BucketListOptionsBuilder {
        (*self).list()
    }
}
