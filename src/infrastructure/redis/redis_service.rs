use bb8_redis::{bb8::Pool, RedisConnectionManager};
use redis::AsyncCommands;

pub struct RedisService {
    pool: Pool<RedisConnectionManager>,
}

impl RedisService {
    pub async fn set_value(&self, key: &str, value: &str) -> redis::RedisResult<()> {
        let mut conn = self.pool.get().await?;
        conn.set(key, value).await?;
        Ok(())
    }

    pub async fn get_value(&self, key: &str) -> redis::RedisResult<Option<String>> {
        let mut conn = self.pool.get().await?;
        conn.get(key).await
    }
}