use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;

use crate::common::*;
use crate::service::sys_menu_service;
use crate::vo::sys_menu_vo::*;
use crate::Json;

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/menu/list", post(menu_list))
        .route("/menu", post(menu_save).put(menu_update).delete(menu_delete))
}

/// 查询菜单
#[utoipa::path(
    post,
    tag = "menu",
    path = "/admin/menu/list",
    request_body = MenuListReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=[MenuListData]", body = Response<Page<MenuListData>>),
    )
)]
pub async fn menu_list(Json(item): Json<MenuListReq>) -> impl IntoResponse {
    let result = sys_menu_service::menu_list(item).await;
    Response::from(result)
}

/// 添加菜单
#[utoipa::path(
    post,
    tag = "menu",
    path = "/admin/menu",
    request_body = MenuSaveReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = Response<u64>),
    )
)]
pub async fn menu_save(Json(item): Json<MenuSaveReq>) -> impl IntoResponse {
    let result = sys_menu_service::menu_save(item).await;
    Response::from(result)
}

/// 更新菜单
#[utoipa::path(
    put,
    tag = "menu",
    path = "/admin/menu",
    request_body = MenuUpdateReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = Response<u64>),
    )
)]
pub async fn menu_update(Json(item): Json<MenuUpdateReq>) -> impl IntoResponse {
    let result = sys_menu_service::menu_update(item).await;
    Response::from(result)
}

/// 删除菜单
#[utoipa::path(
    delete,
    tag = "menu",
    path = "/admin/menu",
    request_body = MenuDeleteReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = Response<u64>),
    )
)]
pub async fn menu_delete(Json(item): Json<MenuDeleteReq>) -> impl IntoResponse {
    let result = sys_menu_service::menu_delete(item).await;
    Response::from(result)
}
