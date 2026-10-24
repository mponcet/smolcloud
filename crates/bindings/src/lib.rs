mod bucket;
mod kv;
mod secret;

pub use bucket::{
    Bucket, BucketError, BucketGetOptionsBuilder, BucketListOptionsBuilder, BucketObject,
    BucketPutOptionsBuilder,
};
pub use kv::{KvError, KvPutOptionsBuilder, KvStore};
pub use secret::{SecretError, SecretFn};

pub use secrecy::{ExposeSecret, SecretString};

pub trait Bindings: Clone + Send + Sync + 'static {
    type B: Bucket;
    fn bucket(&self, name: &str) -> Option<Self::B>;

    type K: KvStore;
    fn kv(&self, name: &str) -> Option<Self::K>;

    fn secret(&self, name: &str) -> Result<SecretString, SecretError>;
}
