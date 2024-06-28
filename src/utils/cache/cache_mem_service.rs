use dashmap::DashMap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::cache_service::ICacheService;
use crate::error::Result;

type Velue = (String, Option<Instant>);

/// Memory Cache Service
#[derive(Debug, Clone)]
pub struct MemCacheService {
    pub cache: Arc<DashMap<String, Velue>>,
    is_run: bool,
}

impl MemCacheService {
    pub fn new() -> Self {
        let s = Self {
            cache: Default::default(),
            is_run: true,
        };
        let s1 = s.clone();
        tokio::spawn(async move {
            loop {
                if !s1.is_run {
                    break;
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
                s1.recycling();
            }
        });
        s
    }

    pub fn recycling(&self) {
        let now = Instant::now();
        // iter map 遍历的性能比 for 循环要好, cloned()比 v.clone()要好
        // 用RWLock + par_iter 反而更慢, 所以改为DashMap + par_iter
        let need_remove = self
            .cache
            .par_iter()
            .filter(|v| {
                // if now > i, means expired
                v.1.is_some() && now.checked_duration_since(v.1.unwrap()).is_some()
            })
            .map(|e| e.key().to_string())
            .collect::<Vec<_>>();

        //  let mut locked = self.cache.write();
        for k in &need_remove {
            self.cache.remove(k);
        }
    }
}

impl Default for MemCacheService {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for MemCacheService {
    fn drop(&mut self) {
        self.is_run = false;
    }
}

#[async_trait::async_trait]
impl ICacheService for MemCacheService {
    async fn get_string(&self, k: &str) -> Result<Option<String>> {
        // self.recycling();
        //  let guard = self.cache.read();
        let v = match self.cache.get(k) {
            Some(r) => Some(r.0.clone()),
            None => None,
        };
        Ok(v)
    }

    async fn set_string(&self, k: &str, v: &str, ex: u64) -> Result<String> {
        let v = v.to_string();
        //  let mut locked = self.cache.write();

        if ex == 0 {
            self.cache.insert(k.to_string(), (v.clone(), None));
        } else {
            let t = Instant::now().checked_add(Duration::from_secs(ex));
            self.cache.insert(k.to_string(), (v.clone(), t));
        }

        Ok(v)
    }

    async fn ttl(&self, k: &str) -> Result<i64> {
        //  self.recycling();
        //  let locked = self.cache.read();
        let v = self.cache.get(k);
        //  drop(locked);
        let v = match v {
            None => -2,
            Some(_r) => match _r.1 {
                None => 0,
                Some(i) => {
                    let now = Instant::now();
                    if i > now {
                        (i - now).as_secs() as i64
                    } else {
                        -1
                    }
                },
            },
        };
        Ok(v)
    }

    async fn remove(&self, k: &str) -> Result<bool> {
        //  let mut locked = self.cache.write();
        let r = self.cache.remove(k).is_some();
        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test_mem() {
        let m = MemCacheService::new();
        let now = std::time::Instant::now();
        for i in 0..100000 {
            m.set_string(&i.to_string(), &i.to_string(), 0).await.unwrap();
        }
        println!("{:?}", m.get_string("1").await.unwrap());
        println!("time set: {:?}", now.elapsed());
        let now = std::time::Instant::now();

        for _i in 0..10000 {
            // m.recycling();
            m.get_string(&_i.to_string()).await.unwrap();
        }
        println!("time get: {:?}", now.elapsed());
    }

    #[tokio::test]
    async fn test_mem_ttl() {
        let m = MemCacheService::new();
        m.set_string("k1", "v1", 0).await.unwrap();
        m.set_string("k2", "v2", 1).await.unwrap();
        m.set_string("k3", "v3", 2).await.unwrap();
        tokio::time::sleep(Duration::from_secs(5)).await;
        println!("ttl: {}", m.ttl("k3").await.unwrap());
        println!("len: {}", m.cache.len());
    }
}
