mod bucket;
mod kv;

pub use bucket::{
    Bucket, BucketError, BucketGetOptionsBuilder, BucketListOptionsBuilder, BucketObject,
    BucketPutOptionsBuilder,
};
pub use kv::{KvError, KvStore};

pub trait Bindings: Clone + Send + Sync + 'static {
    type B: Bucket;
    fn bucket(&self) -> &Self::B;

    type K: KvStore;
    fn kv(&self) -> &Self::K;
}
