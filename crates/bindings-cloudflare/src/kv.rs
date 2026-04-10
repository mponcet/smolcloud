use bindings::{KvError, KvPutOptionsBuilder, KvStore};

use worker::send::IntoSendFuture;

pub struct CloudflareKv(pub worker::KvStore);

struct CloudflareKvPutOptionsBuilder<'kv, 'key, 'value> {
    kv: &'kv CloudflareKv,
    key: &'key str,
    value: &'value [u8],
    expiration_unix_timestamp: Option<u64>,
}

impl<'kv, 'key, 'value> KvPutOptionsBuilder for CloudflareKvPutOptionsBuilder<'kv, 'key, 'value> {
    fn expiration(mut self, unix_timestamp: u64) -> Self {
        self.expiration_unix_timestamp = Some(unix_timestamp);
        self
    }

    async fn execute(self) -> Result<(), KvError> {
        let mut options = self
            .kv
            .0
            .put_bytes(self.key, self.value)
            .map_err(|e| KvError {
                message: "failed to prepare kv put".into(),
                source: e.into(),
            })?;
        if let Some(timestamp) = self.expiration_unix_timestamp {
            options = options.expiration(timestamp);
        }

        async move {
            options.execute().await.map_err(|e| KvError {
                message: "kv put failed".into(),
                source: e.into(),
            })
        }
        .into_send()
        .await
    }
}

impl KvStore for CloudflareKv {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, KvError> {
        async move {
            self.0.get(key).bytes().await.map_err(|e| KvError {
                message: "kv get failed".into(),
                source: e.into(),
            })
        }
        .into_send()
        .await
    }

    fn put(&self, key: &str, value: &[u8]) -> impl KvPutOptionsBuilder {
        CloudflareKvPutOptionsBuilder {
            kv: self,
            key,
            value,
            expiration_unix_timestamp: None,
        }
    }

    async fn delete(&self, key: &str) -> Result<(), KvError> {
        async move {
            self.0.delete(key).await.map_err(|e| KvError {
                message: "kv delete failed".into(),
                source: e.into(),
            })
        }
        .into_send()
        .await
    }
}
