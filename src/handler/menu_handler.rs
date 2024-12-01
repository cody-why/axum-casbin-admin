/*
 * @Date: 2024-06-28 15:21:48
 * @LastEditTime: 2024-07-24 21:45:04
 */

use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;

use crate::service::menu_service;
use crate::vo::menu_vo::*;
use crate::vo::BaseResponse;
use crate::vo::Response;
use crate::Json;

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/menu_list", post(menu_list))
        .route("/menu_save", post(menu_save))
        .route("/menu_delete", post(menu_delete))
        .route("/menu_update", post(menu_update))
}

/// 查询菜单
#[utoipa::path(
    post,
    tag = "menu",
    path = "/admin/menu_list",
    request_body = MenuListReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=[MenuListData]", body = BaseResponse<Vec<MenuListData>>),
    )
)]
pub async fn menu_list(Json(item): Json<MenuListReq>) -> impl IntoResponse {
    let result = menu_service::menu_list(item).await;
    Response::result_page(result, 0)
}

/// 添加菜单
#[utoipa::path(
    post,
    tag = "menu",
    path = "/admin/menu_save",
    request_body = MenuSaveReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = BaseResponse<u64>),
    )
)]
pub async fn menu_save(Json(item): Json<MenuSaveReq>) -> impl IntoResponse {
    let result = menu_service::menu_save(item).await;
    Response::result(result)
}

/// 更新菜单
#[utoipa::path(
    post,
    tag = "menu",
    path = "/admin/menu_update",
    request_body = MenuUpdateReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = BaseResponse<u64>),
    )
)]
pub async fn menu_update(Json(item): Json<MenuUpdateReq>) -> impl IntoResponse {
    let result = menu_service::menu_update(item).await;
    Response::result(result)
}

/// 删除菜单
#[utoipa::path(
    post,
    tag = "menu",
    path = "/admin/menu_delete",
    request_body = MenuDeleteReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = BaseResponse<u64>),
    )
)]
pub async fn menu_delete(Json(item): Json<MenuDeleteReq>) -> impl IntoResponse {
    let result = menu_service::menu_delete(item).await;
    Response::result(result)
}
