use std::net::SocketAddr;

use axum::extract::{ConnectInfo, Request};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response;

struct Limits;

crate::impl_cache!(Limits=>bool, 0, 1);

pub async fn limit_layer(req: Request, next: Next) -> Result<response::Response, StatusCode> {
    let ip = req.extensions().get::<ConnectInfo<SocketAddr>>().unwrap();
    // println!("limit_layer: {:?}", ip.ip());
    let key = format!("{}-{:?}", req.uri(), ip.ip());
    if Limits::get_cache(&key).is_some() {
        return Err(StatusCode::FORBIDDEN);
    }
    Limits::set_cache(key, true);

    Ok(next.run(req).await)
}
