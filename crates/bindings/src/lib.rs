mod bucket;
mod kv;
mod secret;

pub use bucket::{
    Bucket, BucketError, BucketGetOptionsBuilder, BucketListOptionsBuilder, BucketObject,
    BucketPutOptionsBuilder,
};
pub use kv::{KvError, KvStore};
pub use secret::{SecretError, SecretFn};

pub use secrecy::{ExposeSecret, SecretString};

pub trait Bindings: Clone + Send + Sync + 'static {
    type B: Bucket;
    fn bucket(&self) -> &Self::B;

    type K: KvStore;
    fn kv(&self) -> &Self::K;

    fn secret(&self, name: &str) -> Result<SecretString, SecretError>;
}
