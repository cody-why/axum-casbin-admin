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
pub struct Response<T>
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

impl<T> IntoResponse for Response<T>
where
    T: Serialize + Debug,
{
    fn into_response(self) -> axum::response::Response {
        tracing::info!("{:?}", self);
        Json(self).into_response()
    }
}

impl<T> From<Result<T, Error>> for Response<T>
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

impl<T> From<Error> for Response<T>
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

impl<T> From<T> for Response<T>
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

// pub struct Response<T>(pub Result<T, Error>);

impl<T> Response<T>
where
    T: Serialize + Debug,
{
    pub fn ok(data: T) -> Self {
        Self::from(data)
    }

    pub fn err(err: impl Into<Error>) -> Self {
        Self::from(err.into())
    }

    pub fn result(result: Result<T, impl Into<Error> + Debug>) -> Self {
        match result {
            Ok(data) => Self::from(data),
            Err(err) => Self::from(err.into()),
        }
    }

    pub fn page(result: Result<T, impl Into<Error> + Debug>, total: u64) -> Response<T> {
        match result {
            Ok(data) => ok_page(data, total),
            Err(err) => Self::from(err.into()),
        }
    }
}

fn ok_page<T: Serialize + Debug>(data: T, total: u64) -> Response<T> {
    Response {
        msg: Some("Success".to_string()),
        code: 0,
        data: Some(data),
        total: Some(total),
    }
}
