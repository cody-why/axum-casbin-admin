use axum::response::IntoResponse;
use axum::routing::{get, post, put};
use axum::Json;
use axum::Router;
use tracing::info;

use crate::common::*;
use crate::middleware::context::UserContext;
use crate::service::sys_user_service;
use crate::vo::sys_user_vo::*;

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/login", post(login))
        .route("/user/menu", get(query_user_menu))
        .route("/user/role", post(query_user_role).put(update_user_role))
        .route("/user/list", post(user_list))
        .route("/user", post(user_save).put(user_update).delete(user_delete))
        .route("/user/update_password", put(update_user_password))
        .route("/user/reset_password", put(reset_user_password))
    // .route("/google_qrcode", post(google_qrcode).put(google_qrcode_clear))
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
    let result = sys_user_service::login(item).await;
    Response::from(result)
}

/// 查询用户角色
#[utoipa::path(
    post,
    tag = "user",
    path = "/admin/user/role",
    request_body = UserIdReq,
    security(("token" = [])),
    responses(
        (status = 200, description = "操作成功, code=0, data=QueryUserRoleData", body = Response<Vec<QueryUserRoleData>>),
    )
)]
pub async fn query_user_role(Json(item): Json<UserIdReq>) -> impl IntoResponse {
    let result = sys_user_service::query_user_role(item).await;
    Response::from(result)
}

/// 更新用户角色
#[utoipa::path(
    put,
    tag = "user",
    path = "/admin/user/role",
    request_body = UpdateUserRoleReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=bool", body = Response<bool>),
    )
)]
pub async fn update_user_role(Json(item): Json<UpdateUserRoleReq>) -> impl IntoResponse {
    let result = sys_user_service::update_user_role(item).await;
    Response::from(result)
}

/// 查询用户菜单
#[utoipa::path(
    get,
    tag = "user",
    path = "/admin/user/menu",
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=[QueryUserMenuData]", body = Response<Page<QueryUserMenuData>>),
    )
)]
pub async fn query_user_menu(content: UserContext) -> impl IntoResponse {
    info!("query user menu params {:?}", content);
    let result = sys_user_service::query_user_menu(content).await;
    Response::from(result)
}

/// 查询用户列表
#[utoipa::path(
    post,
    tag = "user",
    path = "/admin/user/list",
    request_body = UserListReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=[UserListData]", body = Response<Vec<UserListData>>),
    )
)]
pub async fn user_list(Json(item): Json<UserListReq>) -> impl IntoResponse {
    let result = sys_user_service::user_list(item).await;
    PageResponse::from(result)
}

/// 添加用户
#[utoipa::path(
    post,
    tag = "user",
    path = "/admin/user",
    request_body = UserSaveReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = Response<u64>),
    )
)]
pub async fn user_save(Json(item): Json<UserSaveReq>) -> impl IntoResponse {
    let result = sys_user_service::user_save(item).await;
    Response::from(result)
}

/// 更新用户信息
#[utoipa::path(
    put,
    tag = "user",
    path = "/admin/user",
    request_body = UserUpdateReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = Response<u64>),
    )
)]
pub async fn user_update(Json(item): Json<UserUpdateReq>) -> impl IntoResponse {
    let result = sys_user_service::user_update(item).await;
    Response::from(result)
}

/// 删除用户
#[utoipa::path(
    delete,
    tag = "user",
    path = "/admin/user",
    request_body = UserDeleteReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = Response<u64>),
    )
)]
pub async fn user_delete(Json(item): Json<UserDeleteReq>) -> impl IntoResponse {
    let result = sys_user_service::user_delete(item).await;
    Response::from(result)
}

/// 更新用户密码
#[utoipa::path(
    put,
    tag = "user",
    path = "/admin/user/password",
    request_body = UpdateUserPwdReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = Response<u64>),
    )
)]
pub async fn update_user_password(Json(item): Json<UpdateUserPwdReq>) -> impl IntoResponse {
    let result = sys_user_service::update_user_password(item).await;
    Response::from(result)
}

/// 重置用户密码
#[utoipa::path(
    put,
    tag = "user",
    path = "/admin/user/reset_password",
    request_body = UserIdReq,
    security(("token"=[])),
    responses(
        (status = 200, description = "操作成功, code=0, data=id", body = Response<u64>),
    )
)]
pub async fn reset_user_password(Json(item): Json<UserIdReq>) -> impl IntoResponse {
    let result = sys_user_service::reset_user_password(item).await;
    Response::from(result)
}
