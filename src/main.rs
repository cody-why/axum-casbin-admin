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
    let app = app();
    let addr = context().config.addr.as_str();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("listening on {}", addr);
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}