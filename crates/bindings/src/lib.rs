use std::future::Future;

pub trait Bindings: Clone + Send + Sync + 'static {
    type B: Bucket;
    fn bucket(&self) -> &Self::B;
}

pub trait Bucket {
    fn get(&self, key: String) -> impl Future<Output = String> + Send;
    fn put(&self, key: String, data: String) -> impl Future<Output = ()> + Send;
}
