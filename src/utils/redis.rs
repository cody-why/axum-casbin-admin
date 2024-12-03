use std::sync::OnceLock;

pub use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::Pool;

use crate::context;

#[macro_export]
macro_rules! redis_conn {
    () => {
        async {
            let pool = $crate::utils::redis::redis_pool();
            pool.get().await
        }
        .await
    };
}

// pub async fn redis_pool_bb8() -> &'static Pool<RedisConnectionManager> {
//     static REDIS_POOL: OnceCell<Pool<RedisConnectionManager>> = OnceCell::const_new();

//     REDIS_POOL
//         .get_or_init(|| async {
//             let url = context().config.redis_url.as_str();
//             let manager = RedisConnectionManager::new(url).unwrap();

//             Pool::builder()
//                 .max_size(10)
//                 .build(manager)
//                 .await
//                 .expect("Failed to create redis pool.")
//         })
//         .await
// }

pub fn redis_pool() -> &'static Pool {
    use deadpool_redis::{Config, Runtime};
    static REDIS_POOL: OnceLock<Pool> = OnceLock::new();

    REDIS_POOL.get_or_init(|| {
        let url = context().config.redis_url.as_str();
        let cfg = Config::from_url(url);
        // let pcfg = deadpool_redis::PoolConfig::new(12);
        // cfg.pool = Some(pcfg);
        cfg.create_pool(Some(Runtime::Tokio1)).unwrap()
    })
}

#[ignore]
#[tokio::test(flavor = "multi_thread")]
async fn test_redis_pool() {
    use crate::redis_conn;
    use deadpool_redis::redis::AsyncCommands;

    // let mut conn = get_redis_pool().get().await.unwrap();
    let mut conn = redis_conn!().unwrap();

    let result: bool = conn.set_ex("test_key", "test_value", 100).await.unwrap();
    println!("set result: {:?}", result);

    let value: String = conn.get("test_key").await.unwrap();
    println!("get value: {:?}", value);

    let now = std::time::Instant::now();
    for i in 0..100 {
        // let _: bool = redis::cmd("SETEX").arg(&format!("key{}", i)).arg(100).arg("value")
        //     .query_async(&mut *conn).await.unwrap();
        let _: bool = conn
            .set_ex(format!("key{}", i), "value", 100)
            .await
            .unwrap();
    }
    println!("time set: {:?}", now.elapsed());
}
