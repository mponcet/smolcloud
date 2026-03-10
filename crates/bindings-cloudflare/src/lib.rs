use std::sync::Arc;

#[derive(Clone)]
pub struct CloudflareBindings {
    inner: Arc<CloudflareBindingsInner>,
}

struct CloudflareBindingsInner {
    r2: R2,
}

pub struct R2 {
    bucket: worker::Bucket,
}

impl CloudflareBindings {
    pub fn new(r2: R2) -> Self {
        Self {
            inner: Arc::new(CloudflareBindingsInner::new(r2)),
        }
    }
}

impl CloudflareBindingsInner {
    pub fn new(r2: R2) -> CloudflareBindingsInner {
        Self { r2 }
    }
}

impl bindings::Bucket for R2 {
    #[worker::send]
    async fn get(&self, key: String) -> String {
        self.bucket
            .get(key)
            .execute()
            .await
            .unwrap()
            .unwrap()
            .body()
            .unwrap()
            .text()
            .await
            .unwrap()
            .to_string()
    }

    #[worker::send]
    async fn put(&self, key: String, data: String) {
        self.bucket.put(key, data).execute().await.unwrap();
    }
}

impl bindings::Bindings for CloudflareBindings {
    type B = R2;
    fn bucket(&self) -> &Self::B {
        &self.inner.r2
    }
}

impl From<worker::Bucket> for R2 {
    fn from(bucket: worker::Bucket) -> R2 {
        Self { bucket }
    }
}
