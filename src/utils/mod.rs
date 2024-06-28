use std::time::SystemTime;

pub mod cache;
mod cache_util;
pub mod db;
pub mod json;
pub mod jwt_util;
mod macros;
pub mod password;
pub mod redis;

pub fn get_timestamp() -> u64 {
    let now = SystemTime::now();
    now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}
