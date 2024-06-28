
use tokio::sync::OnceCell;
use bb8_redis::{bb8::Pool, RedisConnectionManager};

#[macro_export]
macro_rules! redis_conn {
    () => {
        async {
            let pool = $crate::utils::redis::get_redis_pool().await;
            pool.get().await
        }.await
    };
}

pub async fn get_redis_pool() -> &'static Pool<RedisConnectionManager> {
    static REDIS_POOL: OnceCell<Pool<RedisConnectionManager>> = OnceCell::const_new();
    
    REDIS_POOL.get_or_init(|| async{
        let manager = RedisConnectionManager::new("redis://:789789@172.17.99.45:6379/").unwrap();
        
        Pool::builder().max_size(100).build(manager).await.expect("Failed to create redis pool.")
    }).await
    
}



#[ignore]
#[tokio::test]
async fn test_redis_pool() {
    use crate::redis_conn;
    use bb8_redis::redis::AsyncCommands;
    let mut conn = redis_conn!().unwrap();
    // let mut conn = get_redis_pool().await.get().await.unwrap();
    let result:bool = conn.set_ex("test_key", "cache_value", 100).await.unwrap();
    println!("set result: {:?}", result);
    let value: String = conn.get("test_key").await.unwrap();
    println!("get value: {:?}", value);

    let now = std::time::Instant::now();
    for i in 0..100 {
        let _:bool = conn.set_ex(&format!("key{}", i), "value", 100).await.unwrap();
    //  let _: String = redis::cmd("SETEX").arg(&format!("key{}", i)).arg(100).arg("value").query_async(&mut *conn).await.unwrap();

    };
    println!("time set: {:?}", now.elapsed());

}


#[tokio::test]
async fn test_redis_conn() {
    // let mut conn = init_redis().await;
    // let now = std::time::Instant::now();
    // for i in 0..100 {
    //      let _: String = redis::cmd("SETEX").arg(&format!("key{}", i)).arg(100).arg("value").query_async(&mut conn).await.unwrap();
        
    // };
    // println!("time set: {:?}", now.elapsed());
}