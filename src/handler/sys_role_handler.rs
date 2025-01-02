use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;

use crate::common::*;
use crate::service::sys_role_service;
use crate::vo::sys_role_vo::*;
use crate::Json;

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/role/list", post(role_list))
        .route("/role", post(role_save).put(role_update).delete(role_delete))
        .route("/role/menu", post(query_role_menu).put(update_role_menu))
}

/// 查询角色列表
#[utoipa::path(
    post,
    tag = "role",
    path = "/admin/role/list",
    request_body = RoleListReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=[RoleListData]", body = Response<Page<RoleListData>>),
    )
)]
pub async fn role_list(Json(item): Json<RoleListReq>) -> impl IntoResponse {
    let result = sys_role_service::role_list(item).await;
    PageResponse::from(result)
}

/// 添加角色
#[utoipa::path(
    post,
    tag = "role",
    path = "/admin/role",
    request_body = RoleSaveReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = Response<u64>),
    )
)]
pub async fn role_save(Json(item): Json<RoleSaveReq>) -> impl IntoResponse {
    let result = sys_role_service::role_save(item).await;

    Response::from(result)
}

/// 更新角色
#[utoipa::path(
    put,
    tag = "role",
    path = "/admin/role",
    request_body = RoleUpdateReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = Response<u64>),
    )
)]
pub async fn role_update(Json(item): Json<RoleUpdateReq>) -> impl IntoResponse {
    let result = sys_role_service::role_update(item).await;
    Response::from(result)
}

/// 删除角色
#[utoipa::path(
    delete,
    tag = "role",
    path = "/admin/role",
    request_body = RoleDeleteReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = Response<u64>),
    )
)]
pub async fn role_delete(Json(item): Json<RoleDeleteReq>) -> impl IntoResponse {
    let result = sys_role_service::role_delete(item).await;
    Response::from(result)
}

/// 查询角色菜单
#[utoipa::path(
    post,
    tag = "role",
    path = "/admin/role/menu",
    request_body = QueryRoleMenuReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=QueryRoleMenuData", body = Response<Vec<QueryRoleMenuData>>),
    )
)]
pub async fn query_role_menu(Json(item): Json<QueryRoleMenuReq>) -> impl IntoResponse {
    let result = sys_role_service::query_role_menu(item).await;
    Response::from(result)
}

/// 更新角色菜单
#[utoipa::path(
    put,
    tag = "role",
    path = "/admin/role/menu",
    request_body = UpdateRoleMenuReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = Response<u64>),
    )
)]
pub async fn update_role_menu(Json(item): Json<UpdateRoleMenuReq>) -> impl IntoResponse {
    let result = sys_role_service::update_role_menu(item).await;
    Response::from(result)
}
