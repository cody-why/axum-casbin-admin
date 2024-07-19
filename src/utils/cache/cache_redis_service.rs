use crate::error::{Error, Result};
use crate::redis_conn;
use crate::utils::redis::AsyncCommands;
use async_trait::async_trait;

use super::cache_service::ICacheService;

/// Redis Cache service
#[derive(Debug)]
pub struct RedisCacheService {}

impl RedisCacheService {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for RedisCacheService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ICacheService for RedisCacheService {
    //  async fn set_string(&self, k: &str, v: &str) -> Result<String> {
    //      self.set_string_ex(k, v, 0).await
    //  }

    async fn get_string(&self, k: &str) -> Result<Option<String>> {
        let mut conn = redis_conn!().unwrap();
        // let result: Option<String> = redis::cmd("GET").arg(&[k]).query_async(&mut conn).await?;
        let result: Option<String> = conn
            .get(k)
            .await
            .map_err(|e| Error::Internal(format!("Redis get_string({}) fail: {}", k, e)))?;
        Ok(result)
    }

    ///set_string Automatically expire
    async fn set_string(&self, k: &str, v: &str, ex: u64) -> Result<String> {
        let mut conn = redis_conn!().unwrap();

        if ex == 0 {
            return conn
                .set(k, v)
                .await
                .map_err(|e| Error::Internal(format!("Redis set_string fail: {}", e)))
                .map(|_v: bool| v.to_string());
        }

        conn.set_ex(k, v, ex)
            .await
            .map_err(|e| Error::Internal(format!("Redis set_string fail: {}", e)))
            .map(|()| v.to_string())
    }

    ///set_string Automatically expire
    async fn ttl(&self, k: &str) -> Result<i64> {
        let mut conn = redis_conn!().unwrap();

        conn.ttl(k)
            .await
            .map(|v: i64| v)
            .map_err(|e| Error::Internal(format!("Redis ttl fail:{}", e)))
    }

    async fn remove(&self, k: &str) -> Result<bool> {
        let mut conn = redis_conn!().unwrap();
        conn.del(k)
            .await
            .map_err(|e| Error::Internal(format!("Redis del fail: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test_redis() {
        let rs = RedisCacheService::new();

        let now = std::time::Instant::now();
        for i in 0..100 {
            let _ = rs.set_string(&format!("key{}", i), "value", 100).await;
        }
        println!("time set: {:?}", now.elapsed());
        let v = rs.get_string(&format!("key{}", 1)).await;
        println!("get: {:?}", v);

        let now = std::time::Instant::now();
        for _i in 0..1000 {
            let _ = rs.get_string(&format!("key{}", _i)).await;
        }
        println!("time get: {:?}", now.elapsed());
    }
}
