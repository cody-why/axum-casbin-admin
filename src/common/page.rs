use serde::Serialize;
use std::fmt::Debug;
use utoipa::ToSchema;

/// 分页数据结构
#[derive(Serialize, Debug, Clone, ToSchema)]
pub struct Page<T, C = ()>
where
    T: Serialize + Debug,
    C: Serialize + Debug,
{
    /// 分页数据
    pub items: Vec<T>,
    /// 分页总数
    pub total: u64,
    /// 当前页码
    pub page_no: u64,
    /// 统计数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<C>,
}
