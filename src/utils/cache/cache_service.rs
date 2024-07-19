use crate::error::Result;
use crate::utils::cache::cache_mem_service::MemCacheService;
use crate::utils::cache::cache_redis_service::RedisCacheService;
use crate::Error;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::ops::Deref;

// use std::future::Future;
// pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[async_trait]
pub trait ICacheService: Sync + Send + Debug {
    /// set key-value, ex seconds expire, 0 = no expire
    async fn set_string(&self, k: &str, v: &str, ex: u64) -> Result<String>;
    // fn set_string(&self, k: &str, v: &str, ex: u64) -> dyn Future<Output = Result<String>> + Send;

    async fn get_string(&self, k: &str) -> Result<Option<String>>;

    /// get key  Time To Live(secs), -2 = key does not exist, -1 = expire, 0 = no expire, >0 = seconds until expire
    async fn ttl(&self, k: &str) -> Result<i64>;

    async fn remove(&self, k: &str) -> Result<bool>;
}

pub struct CacheService {
    pub inner: Box<dyn ICacheService>,
}

pub enum CacheType {
    Mem,
    Redis,
}

impl From<&str> for CacheType {
    fn from(s: &str) -> Self {
        match s {
            "mem" => CacheType::Mem,
            "redis" => CacheType::Redis,
            _ => panic!("not support cache type"),
        }
    }
}

impl CacheService {
    pub fn build(cache_type: &str) -> Result<Self> {
        CacheService::new(CacheType::from(cache_type))
    }
    pub fn new(cache_type: CacheType) -> Result<Self> {
        match cache_type {
            CacheType::Mem => {
                println!("[cache] cache_type: mem");
                Ok(Self {
                    inner: Box::new(MemCacheService::new()),
                })
            },
            CacheType::Redis => {
                println!("[cache] cache_type: redis");
                Ok(Self {
                    inner: Box::new(RedisCacheService::new()),
                })
            },
        }
    }

    // pub async fn set_string(&self, k: &str, v: &str) -> Result<String> {
    //     self.inner.set_string(k, v).await
    // }

    // pub async fn get_string(&self, k: &str) -> Result<String> {
    //     self.inner.get_string(k).await
    // }

    pub async fn set_json<T>(&self, k: &str, v: &T, ex: u64) -> Result<String>
    where
        T: Serialize + Sync,
    {
        let data = sonic_rs::to_string(v).map_err(|e| Error::Internal(format!("MemCacheService set_json fail:{}", e)))?;

        let data = self.set_string(k, data.as_str(), ex).await?;
        Ok(data)
    }

    pub async fn get_json<T>(&self, k: &str) -> Result<T>
    where
        T: DeserializeOwned + Sync,
    {
        let r = self.get_string(k).await?;
        let r = r.unwrap_or("null".to_string());
        let data: T =
            sonic_rs::from_str(r.as_str()).map_err(|e| Error::Internal(format!("CacheService get_json fail:{}", e)))?;

        Ok(data)
    }

    // pub async fn ttl(&self, k: &str) -> Result<i64> {
    //     self.inner.ttl(k).await
    // }
}

impl Deref for CacheService {
    type Target = Box<dyn ICacheService>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Default for CacheService {
    fn default() -> Self {
        Self::new(CacheType::Mem).unwrap()
    }
}

#[tokio::test]
async fn test_cache_service() {
    let cache = CacheService::new(CacheType::Mem).unwrap();
    let _ = cache.set_string("test", "123", 0).await;
    let v = cache.get_string("test").await.unwrap();
    assert_eq!(v, Some("123".to_string()));
    let _ = cache.set_json("test_json", &123, 0).await;
    let v = cache.get_json::<i32>("test_json").await.unwrap();
    assert_eq!(v, 123);
}
