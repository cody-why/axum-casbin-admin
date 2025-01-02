

use axum::response::IntoResponse;
use axum::routing::get;
use axum::{middleware, Router};
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};

use super::{apidoc, sys_menu_handler, sys_role_handler, sys_user_handler};
use crate::middleware::limit::limit_layer;
use crate::middleware::logger::log_layer;
use crate::Json;
use crate::{middleware::auth::auth_layer, pool};

pub fn app() -> Router {
    // let origins = [
    //     "http://localhost:3000".parse().unwrap(),
    // ];
    // let _trace_layer = TraceLayer::new_for_http()
    //     .make_span_with(DefaultMakeSpan::new().level(Level::TRACE))
    //     .on_request(())
    //     .on_response(|res: &Response<_>, latency: Duration, _span: &_| {
    //         log::warn!("on_response: {:?}ms, {:?}", latency.as_millis(),
    // res.status());     });
    let cors_layer = CorsLayer::new().allow_methods(Any).allow_origin(Any).allow_headers(Any);

    Router::new()
        .nest(
            "/admin",
            Router::new()
                .merge(sys_user_handler::router())
                .merge(sys_role_handler::router())
                .merge(sys_menu_handler::router()), // .with_state(app_state)
        )
        .layer(middleware::from_fn(auth_layer))
        .layer(middleware::from_fn(limit_layer))
        .layer(middleware::from_fn(log_layer))
        // .layer(_trace_layer)
        .merge(apidoc::router())
        .route("/status", get(db_status))
        .layer(cors_layer)
        .merge(static_file())
}

async fn db_status() -> impl IntoResponse {
    let state = pool!().get_pool().expect("pool not init!").state().await;
    Json(state)
}

pub fn static_file() -> Router {
    Router::new().nest_service(
        "/static",
        ServeDir::new("static").not_found_service(ServeFile::new("static/index.html")),
    )
}
