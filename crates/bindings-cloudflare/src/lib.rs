mod bucket;
mod kv;

pub use bindings::{SecretError, SecretFn, SecretString};
pub use bucket::CloudflareBucket;
pub use kv::CloudflareKv;
use worker::Env;

use std::sync::Arc;

#[derive(Clone)]
pub struct CloudflareBindings {
    env: Arc<Env>,
}

impl CloudflareBindings {
    pub fn new(env: Env) -> Self {
        Self { env: Arc::new(env) }
    }
}

impl bindings::Bindings for CloudflareBindings {
    type B = CloudflareBucket;
    fn bucket(&self, name: &str) -> Option<Self::B> {
        self.env.bucket(name).ok().map(Into::into)
    }

    type K = CloudflareKv;
    fn kv(&self, name: &str) -> Option<Self::K> {
        self.env.kv(name).ok().map(Into::into)
    }

    fn secret(&self, name: &str) -> Result<SecretString, SecretError> {
        self.env
            .secret(name)
            .map(|s| SecretString::new(s.to_string().into()))
            .map_err(|e| SecretError {
                message: format!("failed to get secret '{name}'"),
                source: e.into(),
            })
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
