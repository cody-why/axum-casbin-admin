/*
 * @Date: 2024-06-28 15:21:48
 * @LastEditTime: 2024-08-19 11:10:35
 */

extern crate rbatis;

use std::net::SocketAddr;

pub mod config;
pub mod error;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod service;
pub mod utils;
pub mod vo;

use config::init_logger;
pub use error::*;
pub use service::context;
pub use utils::json::Json;

use handler::root::*;
use rbatis::RBatis;
use tracing::info;

// #[cfg(not(target_env = "msvc"))]
// use tikv_jemallocator::Jemalloc;

// #[cfg(not(target_env = "msvc"))]
// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;

pub struct AppState {
    pub batis: RBatis,
}

#[tokio::main]
async fn main() {
    init_logger();

    let addr = context().config.addr.as_str();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("listening on {}", addr);
    let app = app();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
