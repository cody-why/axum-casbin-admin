use crate::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use std::{any::TypeId, fmt::Debug};
use utoipa::ToSchema;

use crate::error::Error;
use rbatis::Page as RbatisPage;

use super::Page;

pub type PageResponse<T, C = ()> = Response<Page<T, C>>;

/// Response
#[derive(Serialize, Debug, Clone, ToSchema)]
pub struct Response<T>
where
    T: Serialize + Debug,
{
    /// 状态码
    #[schema(example = 0)]
    pub code: i32,
    /// 消息
    #[schema(example = "Success")]
    pub msg: Option<String>,
    /// 数据
    pub data: Option<T>,
}

impl<T> Response<T>
where
    T: Serialize + Debug,
{
    pub fn new(code: i32, msg: Option<String>, data: Option<T>) -> Self {
        Self { code, msg, data }
    }
}

impl<T> IntoResponse for Response<T>
where
    T: Serialize + Debug + 'static,
{
    fn into_response(self) -> axum::response::Response {
        // 数据量小才打印日志
        if TypeId::of::<T>() != TypeId::of::<crate::vo::sys_user_vo::QueryUserMenuData>() {
            tracing::info!("{:?}", self);
        }

        // if !std::any::type_name::<T>().contains("QueryUserMenuData") {
        //     tracing::info!("{:?}", self);
        // }

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
            msg: Some("操作成功".to_string()),
            data: Some(data),
        }
    }
}

/// 分页结构, RbatisPage<T>, T 为数据
impl<T> From<Result<RbatisPage<T>, Error>> for Response<Page<T>>
where
    T: Serialize + Debug + Send + Sync,
{
    fn from(value: Result<RbatisPage<T>, Error>) -> Self {
        let data = value.map(|page| Page {
            items: page.records,
            total: page.total,
            page_no: page.page_no,
            count: None,
        });
        match data {
            Ok(page) => Response::from(page),
            Err(err) => Response::from(err),
        }
    }
}

/// 分页结构, (RbatisPage<T>, C), T 为数据, C 为统计数据
impl<T, C> From<Result<(RbatisPage<T>, C), Error>> for Response<Page<T, C>>
where
    T: Serialize + Debug + Send + Sync,
    C: Serialize + Debug,
{
    fn from(value: Result<(RbatisPage<T>, C), Error>) -> Self {
        let data = value.map(|(page, count)| Page {
            items: page.records,
            total: page.total,
            page_no: page.page_no,
            count: Some(count),
        });
        match data {
            Ok(page) => Response::from(page),
            Err(err) => Response::from(err),
        }
    }
}
