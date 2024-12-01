/*
 * @Date: 2024-06-28 15:21:48
 * @LastEditTime: 2024-07-24 21:41:22
 */

use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;

use crate::service::role_service;
use crate::vo::role_vo::*;
use crate::vo::*;
use crate::Json;

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/query_role_menu", post(query_role_menu))
        .route("/update_role_menu", post(update_role_menu))
        .route("/role_list", post(role_list))
        .route("/role_save", post(role_save))
        .route("/role_delete", post(role_delete))
        .route("/role_update", post(role_update))
}

/// 查询角色列表
#[utoipa::path(
    post,
    tag = "role",
    path = "/admin/role_list",
    request_body = RoleListReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=RoleListData", body = BaseResponse<Vec<RoleListData>>),
    )
)]
pub async fn role_list(Json(item): Json<RoleListReq>) -> impl IntoResponse {
    let result = role_service::role_list(item).await;
    let total = result.as_ref().map(|data| data.total).unwrap_or(0);
    // 转换成前端需要的格式
    let result = result.map(|data| data.records);
    Response::result_page(result, total)
}

/// 添加角色
#[utoipa::path(
    post,
    tag = "role",
    path = "/admin/role_save",
    request_body = RoleSaveReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = BaseResponse<u64>),
    )
)]
pub async fn role_save(Json(item): Json<RoleSaveReq>) -> impl IntoResponse {
    let result = role_service::role_save(item).await;

    Response::result(result)
}

/// 更新角色
#[utoipa::path(
    post,
    tag = "role",
    path = "/admin/role_update",
    request_body = RoleUpdateReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = BaseResponse<u64>),
    )
)]
pub async fn role_update(Json(item): Json<RoleUpdateReq>) -> impl IntoResponse {
    let result = role_service::role_update(item).await;
    Response::result(result)
}

/// 删除角色
#[utoipa::path(
    post,
    tag = "role",
    path = "/admin/role_delete",
    request_body = RoleDeleteReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = BaseResponse<u64>),
    )
)]
pub async fn role_delete(Json(item): Json<RoleDeleteReq>) -> impl IntoResponse {
    let result = role_service::role_delete(item).await;
    Response::result(result)
}

/// 查询角色菜单
#[utoipa::path(
    post,
    tag = "role",
    path = "/admin/query_role_menu",
    request_body = QueryRoleMenuReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=QueryRoleMenuData", body = BaseResponse<Vec<QueryRoleMenuData>>),
    )
)]
pub async fn query_role_menu(Json(item): Json<QueryRoleMenuReq>) -> impl IntoResponse {
    let result = role_service::query_role_menu(item).await;
    Response::result(result)
}

/// 更新角色菜单
#[utoipa::path(
    post,
    tag = "role",
    path = "/admin/update_role_menu",
    request_body = UpdateRoleMenuReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = BaseResponse<u64>),
    )
)]
pub async fn update_role_menu(Json(item): Json<UpdateRoleMenuReq>) -> impl IntoResponse {
    let result = role_service::update_role_menu(item).await;
    Response::result(result)
}
