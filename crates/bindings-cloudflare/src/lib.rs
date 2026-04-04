mod bucket;
mod kv;

pub use crate::bucket::R2;
pub use crate::kv::KV;
pub use bindings::{SecretError, SecretFn};

use std::sync::Arc;

#[derive(Clone)]
pub struct CloudflareBindings {
    inner: Arc<CloudflareBindingsInner>,
}

struct CloudflareBindingsInner {
    r2: R2,
    kv: KV,
    secret: Box<SecretFn>,
}

impl CloudflareBindings {
    pub fn new(r2: R2, kv: KV, secret: Box<SecretFn>) -> Self {
        Self {
            inner: Arc::new(CloudflareBindingsInner::new(r2, kv, secret)),
        }
    }
}

impl CloudflareBindingsInner {
    pub fn new(r2: R2, kv: KV, secret: Box<SecretFn>) -> CloudflareBindingsInner {
        Self { r2, kv, secret }
    }
}

impl bindings::Bindings for CloudflareBindings {
    type B = R2;
    fn bucket(&self) -> &Self::B {
        &self.inner.r2
    }

    type K = KV;
    fn kv(&self) -> &Self::K {
        &self.inner.kv
    }

    fn secret(&self, name: &str) -> Result<String, SecretError> {
        (self.inner.secret)(name)
    }
}

impl From<worker::Bucket> for R2 {
    fn from(bucket: worker::Bucket) -> R2 {
        Self(bucket)
    }
}

impl From<worker::kv::KvStore> for KV {
    fn from(value: worker::kv::KvStore) -> Self {
        Self(value)
    }
}
