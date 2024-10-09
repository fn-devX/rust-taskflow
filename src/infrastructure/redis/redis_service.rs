use bb8_redis::{bb8::Pool, RedisConnectionManager};
use redis::{AsyncCommands, RedisError};
use thiserror::Error;
use serde::{Serialize, Deserialize};
use std::marker::PhantomData;

use bb8_redis::bb8::RunError;

#[derive(Debug, Error)]
pub enum RedisServiceError {
    #[error("Redis connection error: {0}")]
    ConnectionError(#[from] RunError<RedisError>),
    #[error("Redis command error: {0}")]
    CommandError(#[from] RedisError),
}

pub struct RedisService<T> {
    pool: Pool<RedisConnectionManager>,
    _marker: PhantomData<T>,
}

impl<T> RedisService<T> {
    pub fn new(pool: Pool<RedisConnectionManager>) -> Self {
        Self {
            pool,
            _marker: PhantomData,
        }
    }

    pub async fn set_value<K, V>(&self, key: K, value: V) -> Result<(), RedisServiceError>
    where
        K: AsRef<str>,
        V: Serialize,
    {
        let mut conn = self.pool.get().await.expect("Failed to get connection");
        let serialized_value = serde_json::to_string(&value).map_err(|e| {
            RedisServiceError::CommandError(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Serialization error",
            )))
        })?;
        conn.set(key.as_ref(), serialized_value).await?;
        Ok(())
    }

    pub async fn get_value<K, V>(&self, key: K) -> Result<Option<V>, RedisServiceError>
    where
        K: AsRef<str>,
        V: for<'de> Deserialize<'de>,
    {
        let mut conn = self.pool.get().await.expect("Failed to get connection");
        let result: Option<String> = conn.get(key.as_ref()).await?;
        if let Some(data) = result {
            let deserialized_value = serde_json::from_str(&data).map_err(|e| {
                RedisServiceError::CommandError(redis::RedisError::from((
                    redis::ErrorKind::TypeError,
                    "Deserialization error",
                )))
            })?;
            Ok(Some(deserialized_value))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_value<K>(&self, key: K) -> Result<(), RedisServiceError>
    where
        K: AsRef<str>,
    {
        let mut conn = self.pool.get().await.expect("Failed to get connection");
        conn.del(key.as_ref()).await?;
        Ok(())
    }

    pub async fn key_exists<K>(&self, key: K) -> Result<bool, RedisServiceError>
    where
        K: AsRef<str>,
    {
        let mut conn = self.pool.get().await.expect("Failed to get connection");
        let exists: bool = conn.exists(key.as_ref()).await?;
        Ok(exists)
    }
}
