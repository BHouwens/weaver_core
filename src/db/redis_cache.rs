use crate::{db::handler::KvStoreConnection, utils::deserialize_data, utils::serialize_data};
use async_trait::async_trait;
use redis::{aio::ConnectionManager, AsyncCommands};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone)]
pub struct RedisCacheConn {
    pub connection: ConnectionManager,
}

#[async_trait]
impl KvStoreConnection for RedisCacheConn {
    async fn init(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let redis_client = redis::Client::open(url)?;
        let redis_connection_manager = ConnectionManager::new(redis_client).await?;

        Ok(RedisCacheConn {
            connection: redis_connection_manager,
        })
    }

    async fn set_data<T: Serialize + Send>(
        &mut self,
        key: &str,
        value: T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let serialized_data = serialize_data(&value);
        let _: () = self.connection.set(key, serialized_data).await?;
        Ok(())
    }

    async fn get_data<T: DeserializeOwned>(
        &mut self,
        key: &str,
    ) -> Result<Option<T>, Box<dyn std::error::Error>> {
        let result: Option<String> = self.connection.get(key).await?;

        if let Some(data) = result {
            let deserialized: T = deserialize_data(data);
            return Ok(Some(deserialized));
        }

        Ok(None)
    }
}
