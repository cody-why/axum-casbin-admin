use axum::response::IntoResponse;
use serde::Serialize;
use std::fmt::Debug;
use utoipa::ToSchema;

use crate::{error::Error, Json};

pub mod menu_vo;
pub mod role_vo;
pub mod user_vo;

/// 统一返回Response
#[derive(Serialize, Debug, Clone, ToSchema)]
pub struct BaseResponse<T>
where
    T: Serialize + Debug,
{
    #[schema(example = 0)]
    pub code: i32,
    #[schema(example = "")]
    pub msg: Option<String>,
    pub data: Option<T>,
    /// 统一分页总数
    #[schema(example = 0)]
    pub total: Option<u64>,
}

impl<T> IntoResponse for BaseResponse<T>
where
    T: Serialize + Debug,
{
    fn into_response(self) -> axum::response::Response {
        tracing::info!("{:?}", self);
        Json(self).into_response()
    }
}

impl<T> From<Result<T, Error>> for BaseResponse<T>
where
    T: Serialize + Debug,
{
    fn from(value: Result<T, Error>) -> Self {
        match value {
            Ok(data) => Self::from(data),
            Err(e) => Self::from(e),
        }
    }
}

impl<T> From<Error> for BaseResponse<T>
where
    T: Serialize + Debug,
{
    fn from(e: Error) -> Self {
        tracing::warn!("error: {:?}", e);
        let (code, msg) = e.to_msg();
        Self {
            code,
            msg: Some(msg),
            data: None,
            total: None,
        }
    }
}

impl<T> From<T> for BaseResponse<T>
where
    T: Serialize + Debug,
{
    fn from(data: T) -> Self {
        Self {
            code: 0,
            msg: Some("Success".to_string()),
            data: Some(data),
            total: None,
        }
    }
}

/// 统一返回结果
pub struct Response<T>(pub Result<T, Error>);

impl<T> Response<T>
where
    T: Serialize + Debug,
{
    pub fn ok(data: T) -> Self {
        Self(Ok(data))
    }

    pub fn err(err: impl Into<Error>) -> Self {
        Self(Err(err.into()))
    }

    pub fn result(result: Result<T, impl Into<Error> + Debug>) -> Self {
        match result {
            Ok(data) => Self(Ok(data)),
            Err(err) => Self(Err(err.into())),
        }
    }

    pub fn result_page(result: Result<T, impl Into<Error> + Debug>, total: u64) -> BaseResponse<T> {
        match result {
            Ok(data) => ok_result_page(data, total),
            Err(err) => err_result_page::<T>(err),
        }
    }
}

impl<T> IntoResponse for Response<T>
where
    T: Serialize + Debug,
{
    fn into_response(self) -> axum::response::Response {
        BaseResponse::from(self.0).into_response()
    }
}

fn ok_result_page<T: Serialize + Debug>(data: T, total: u64) -> BaseResponse<T> {
    BaseResponse {
        msg: Some("Success".to_string()),
        code: 0,
        data: Some(data),
        total: Some(total),
    }
}

fn err_result_page<T: Serialize + Debug>(err: impl Into<Error>) -> BaseResponse<T> {
    let err = err.into();
    tracing::warn!("error: {:?}", err);
    let (code, msg) = err.to_msg();
    BaseResponse {
        msg: Some(msg),
        code,
        data: None,
        total: None,
    }
}
