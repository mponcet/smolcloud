use bindings::KvError;

pub struct KV(pub(crate) worker::KvStore);

impl bindings::KvStore for KV {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, KvError> {
        self.0.get(key).bytes().await.map_err(|e| KvError {
            message: "kv get failed".into(),
            source: e.into(),
        })
    }

    async fn put(&self, key: &str, value: &[u8]) -> Result<(), KvError> {
        self.0
            .put_bytes(key, value)
            .map_err(|e| KvError {
                message: "failed to prepare kv put".into(),
                source: e.into(),
            })?
            .execute()
            .await
            .map_err(|e| KvError {
                message: "kv put failed".into(),
                source: e.into(),
            })
    }

    async fn delete(&self, key: &str) -> Result<(), KvError> {
        self.0.delete(key).await.map_err(|e| KvError {
            message: "kv delete failed".into(),
            source: e.into(),
        })
    }
}
