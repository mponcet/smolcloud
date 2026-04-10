mod bucket;
mod kv;

pub use bindings::{SecretError, SecretFn, SecretString};
pub use bucket::CloudflareBucket;
pub use kv::CloudflareKv;

use std::sync::Arc;

#[derive(Clone)]
pub struct CloudflareBindings {
    inner: Arc<CloudflareBindingsInner>,
}

struct CloudflareBindingsInner {
    r2: CloudflareBucket,
    kv: CloudflareKv,
    secret: Box<SecretFn>,
}

impl CloudflareBindings {
    pub fn new(r2: CloudflareBucket, kv: CloudflareKv, secret: Box<SecretFn>) -> Self {
        Self {
            inner: Arc::new(CloudflareBindingsInner::new(r2, kv, secret)),
        }
    }
}

impl CloudflareBindingsInner {
    pub fn new(
        r2: CloudflareBucket,
        kv: CloudflareKv,
        secret: Box<SecretFn>,
    ) -> CloudflareBindingsInner {
        Self { r2, kv, secret }
    }
}

impl bindings::Bindings for CloudflareBindings {
    type B = CloudflareBucket;
    fn bucket(&self) -> &Self::B {
        &self.inner.r2
    }

    type K = CloudflareKv;
    fn kv(&self) -> &Self::K {
        &self.inner.kv
    }

    fn secret(&self, name: &str) -> Result<SecretString, SecretError> {
        (self.inner.secret)(name)
    }
}

impl From<worker::Bucket> for CloudflareBucket {
    fn from(bucket: worker::Bucket) -> CloudflareBucket {
        Self(bucket)
    }
}

impl From<worker::kv::KvStore> for CloudflareKv {
    fn from(value: worker::kv::KvStore) -> Self {
        Self(value)
    }
}
