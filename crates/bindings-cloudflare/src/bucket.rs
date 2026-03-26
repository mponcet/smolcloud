use bindings::{
    BucketError, BucketGetOptionsBuilder, BucketListOptionsBuilder, BucketObject,
    BucketPutOptionsBuilder,
};

use std::collections::HashMap;

use worker::Include;
use worker::send::IntoSendFuture;

struct CloudflareGetOptionsBuilder<'bucket> {
    bucket: &'bucket R2,
    key: String,
    range: Option<std::ops::Range<usize>>,
}

pub struct R2(pub worker::Bucket);

impl std::ops::Deref for R2 {
    type Target = worker::Bucket;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'bucket> BucketGetOptionsBuilder for CloudflareGetOptionsBuilder<'bucket> {
    fn range(mut self, range: std::ops::Range<usize>) -> Self {
        self.range = Some(range);
        self
    }

    async fn execute(self) -> Result<Option<BucketObject>, BucketError> {
        let mut options = self.bucket.get(self.key);
        if let Some(range) = self.range {
            // FIXME: invalid range
            options = options.range(
                worker_sys::R2Range {
                    offset: Some(range.start as f64),
                    length: Some(range.end.saturating_sub(range.start) as f64),
                    suffix: None,
                }
                .try_into()
                .expect("range bound should be checked at creation"),
            );
        }

        async move {
            match options.execute().await {
                Ok(Some(object)) => {
                    let key = object.key();
                    let body = if let Some(body) = object.body() {
                        body.bytes().await.ok()
                    } else {
                        None
                    };
                    let custom_metadata = object.custom_metadata().ok();

                    Ok(Some(BucketObject {
                        key,
                        body,
                        custom_metadata,
                    }))
                }
                Ok(None) => Ok(None),
                Err(e) => Err(BucketError {
                    message: "bucket get object failed".into(),
                    source: e.into(),
                }),
            }
        }
        .into_send()
        .await
    }
}

struct CloudflarePutOptionsBuilder<'bucket> {
    bucket: &'bucket R2,
    key: String,
    data: Vec<u8>,
    custom_metadata: Option<HashMap<String, String>>,
}

impl<'bucket> bindings::BucketPutOptionsBuilder for CloudflarePutOptionsBuilder<'bucket> {
    fn custom_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.custom_metadata = Some(metadata);
        self
    }

    async fn execute(self) -> Result<BucketObject, BucketError> {
        async move {
            let mut options = self.bucket.put(self.key, self.data);
            if let Some(custom_metadata) = self.custom_metadata {
                options = options.custom_metadata(custom_metadata);
            }

            match options.execute().await {
                Ok(object) => Ok(BucketObject {
                    key: object.key(),
                    body: None,
                    custom_metadata: object.custom_metadata().ok(),
                }),
                Err(e) => Err(BucketError {
                    message: "bucket put objet failed".into(),
                    source: e.into(),
                }),
            }
        }
        .into_send()
        .await
    }
}

struct CloudflareListOptionsBuilder<'bucket> {
    bucket: &'bucket R2,
    limit: Option<u32>,
    prefix: Option<String>,
    custom_metadata: bool,
}

impl<'bucket> bindings::BucketListOptionsBuilder for CloudflareListOptionsBuilder<'bucket> {
    fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    fn prefix(mut self, prefix: &str) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    fn include_custom_metadata(mut self) -> Self {
        self.custom_metadata = true;
        self
    }

    async fn execute(self) -> Result<Vec<BucketObject>, BucketError> {
        async move {
            let mut options = self.bucket.list();
            if let Some(limit) = self.limit {
                options = options.limit(limit);
            }
            if let Some(prefix) = self.prefix {
                options = options.prefix(prefix);
            }
            if self.custom_metadata {
                options = options.include(vec![Include::CustomMetadata]);
            }

            match options.execute().await {
                Ok(objects) => Ok(objects
                    .objects()
                    .iter()
                    .map(|object| BucketObject {
                        key: object.key(),
                        body: None,
                        custom_metadata: object.custom_metadata().ok(),
                    })
                    .collect()),
                Err(e) => Err(BucketError {
                    message: "bucket list objects failed".into(),
                    source: e.into(),
                }),
            }
        }
        .into_send()
        .await
    }
}

impl bindings::Bucket for R2 {
    async fn head(&self, key: &str) -> Result<Option<BucketObject>, BucketError> {
        async move {
            match self.0.head(key).await {
                Ok(Some(object)) => Ok(Some(BucketObject {
                    key: object.key(),
                    body: None,
                    custom_metadata: object.custom_metadata().ok(),
                })),
                Ok(None) => Ok(None),
                Err(e) => Err(BucketError {
                    message: "bucket head object failed".into(),
                    source: e.into(),
                }),
            }
        }
        .into_send()
        .await
    }

    fn get(&self, key: &str) -> impl BucketGetOptionsBuilder {
        CloudflareGetOptionsBuilder {
            bucket: self,
            key: key.into(),
            range: None,
        }
    }

    fn put(&self, key: &str, data: &[u8]) -> impl BucketPutOptionsBuilder {
        CloudflarePutOptionsBuilder {
            bucket: self,
            key: key.into(),
            data: data.into(),
            custom_metadata: None,
        }
    }

    async fn delete(&self, key: &str) -> Result<(), BucketError> {
        async move {
            let result = self.0.delete(key).await;
            result.map_err(|e| BucketError {
                message: "bucket delete object failed".into(),
                source: e.into(),
            })
        }
        .into_send()
        .await
    }

    fn list(&self) -> impl BucketListOptionsBuilder {
        CloudflareListOptionsBuilder {
            bucket: self,
            limit: None,
            prefix: None,
            custom_metadata: false,
        }
    }
}
