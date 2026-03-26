mod bucket;
mod kv;

pub use crate::bucket::R2;
pub use crate::kv::KV;

use std::sync::Arc;

#[derive(Clone)]
pub struct CloudflareBindings {
    inner: Arc<CloudflareBindingsInner>,
}

struct CloudflareBindingsInner {
    r2: R2,
    kv: KV,
}

impl CloudflareBindings {
    pub fn new(r2: R2, kv: KV) -> Self {
        Self {
            inner: Arc::new(CloudflareBindingsInner::new(r2, kv)),
        }
    }
}

impl CloudflareBindingsInner {
    pub fn new(r2: R2, kv: KV) -> CloudflareBindingsInner {
        Self { r2, kv }
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
