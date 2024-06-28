use crate::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use std::fmt::Debug;

use crate::error::Error;

pub mod menu_vo;
pub mod role_vo;
pub mod user_vo;

/// 统一返回vo
#[derive(Serialize, Debug, Clone)]
pub struct BaseResponse<T>
where
    T: Serialize + Debug,
{
    pub code: i32,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> BaseResponse<T>
where
    T: Serialize + Debug,
{
    pub fn err(code: i32, msg: impl ToString) -> Self {
        Self {
            code,
            msg: Some(msg.to_string()),
            data: None,
        }
    }
}

impl<T> IntoResponse for BaseResponse<T>
where
    T: Serialize + Debug,
{
    fn into_response(self) -> axum::response::Response {
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
        match e {
            Error::E(msg) => Self {
                code: 1,
                msg: Some(msg),
                data: None,
            },
            Error::Code(_code, msg) => Self {
                code: _code,
                msg: Some(msg),
                data: None,
            },
            _ => Self {
                code: 2,
                msg: Some("操作失败".to_string()),
                data: None,
            },
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
            msg: Some("操作成功".to_string()),
            data: Some(data),
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

    pub fn result(result: core::result::Result<T, impl Into<Error>>) -> Self {
        match result {
            Ok(data) => Self(Ok(data)),
            Err(err) => Self(Err(err.into())),
        }
    }

    pub fn result_page(result: core::result::Result<T, impl Into<Error>>, total: u64) -> Json<ResponsePage<T>> {
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

/// 统一返回分页
#[derive(Serialize, Debug, Clone)]
pub struct ResponsePage<T>
where
    T: Serialize + Debug,
{
    pub code: i32,
    pub msg: String,
    pub total: u64,
    pub data: Option<T>,
}

fn ok_result_page<T: Serialize + Debug>(data: T, total: u64) -> Json<ResponsePage<T>> {
    Json(ResponsePage {
        msg: "操作成功".to_string(),
        code: 0,
        data: Some(data),
        total,
    })
}

fn err_result_page<T: Serialize + Debug>(err: impl Into<Error>) -> Json<ResponsePage<T>> {
    let msg = err.into().msg();
    Json(ResponsePage {
        msg,
        code: 1,
        data: None,
        total: 0,
    })
}
