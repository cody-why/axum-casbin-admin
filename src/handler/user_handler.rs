/*
 * @Date: 2024-07-02 23:17:43
 * @LastEditTime: 2024-07-24 21:24:20
 */

use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use tracing::info;

use crate::middleware::context::UserContext;
use crate::service::user_service;
use crate::vo::user_vo::*;
use crate::vo::*;

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/login", post(login))
        .route("/query_user_menu", get(query_user_menu))
        .route("/query_user_role", post(query_user_role))
        .route("/update_user_role", post(update_user_role))
        .route("/user_list", post(user_list))
        .route("/user_save", post(user_save))
        .route("/user_delete", post(user_delete))
        .route("/user_update", post(user_update))
        .route("/update_user_password", post(update_user_password))
}

/// 用户登录
#[utoipa::path(
    post,
    tag = "user",
    path = "/admin/login",
    request_body = UserLoginReq,
    responses(
        (status = 200, description = "操作成功, code=0, data=token", body = Response<String>),
    )
)]
pub async fn login(Json(item): Json<UserLoginReq>) -> impl IntoResponse {
    let result = user_service::login(item).await;
    Response::result(result)
}

/// 查询用户角色
#[utoipa::path(
    post,
    tag = "user",
    path = "/admin/query_user_role",
    request_body = QueryUserRoleReq,
    security(("token" = [])),
    responses(
        (status = 200, description = "操作成功, code=0, data=QueryUserRoleData", body = Response<Vec<QueryUserRoleData>>),
    )
)]
pub async fn query_user_role(Json(item): Json<QueryUserRoleReq>) -> impl IntoResponse {
    let result = user_service::query_user_role(item).await;
    Response::result(result)
}

/// 更新用户角色
#[utoipa::path(
    post,
    tag = "user",
    path = "/admin/update_user_role",
    request_body = UpdateUserRoleReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=bool", body = Response<bool>),
    )
)]
pub async fn update_user_role(Json(item): Json<UpdateUserRoleReq>) -> impl IntoResponse {
    let result = user_service::update_user_role(item).await;
    Response::result(result)
}

/// 查询用户菜单
#[utoipa::path(
    get,
    tag = "user",
    path = "/admin/query_user_menu",
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=QueryUserMenuData", body = Response<Vec<QueryUserMenuData>>),
    )
)]
pub async fn query_user_menu(content: UserContext) -> impl IntoResponse {
    info!("query user menu params {:?}", content);
    let result = user_service::query_user_menu(content).await;
    Response::result(result)
}

/// 查询用户列表
#[utoipa::path(
    post,
    tag = "user",
    path = "/admin/user_list",
    request_body = UserListReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=[UserListData]", body = Response<Vec<UserListData>>),
    )
)]
pub async fn user_list(Json(item): Json<UserListReq>) -> impl IntoResponse {
    let result = user_service::user_list(item).await;
    let total = result.as_ref().map(|data| data.total).unwrap_or(0);
    // 转换成前端需要的格式
    let result = result.map(|data| data.records);
    Response::page(result, total)
}

/// 添加用户
#[utoipa::path(
    post,
    tag = "user",
    path = "/admin/user_save",
    request_body = UserSaveReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = Response<u64>),
    )
)]
pub async fn user_save(Json(item): Json<UserSaveReq>) -> impl IntoResponse {
    let result = user_service::user_save(item).await;
    Response::result(result)
}

/// 更新用户信息
#[utoipa::path(
    post,
    tag = "user",
    path = "/admin/user_update",
    request_body = UserUpdateReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = Response<u64>),
    )
)]
pub async fn user_update(Json(item): Json<UserUpdateReq>) -> impl IntoResponse {
    let result = user_service::user_update(item).await;
    Response::result(result)
}

/// 删除用户
#[utoipa::path(
    post,
    tag = "user",
    path = "/admin/user_delete",
    request_body = UserDeleteReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = Response<u64>),
    )
)]
pub async fn user_delete(Json(item): Json<UserDeleteReq>) -> impl IntoResponse {
    let result = user_service::user_delete(item).await;
    Response::result(result)
}

/// 更新用户密码
#[utoipa::path(
    post,
    tag = "user",
    path = "/admin/update_user_password",
    request_body = UpdateUserPwdReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = Response<u64>),
    )
)]
pub async fn update_user_password(Json(item): Json<UpdateUserPwdReq>) -> impl IntoResponse {
    let result = user_service::update_user_password(item).await;
    Response::result(result)
}
