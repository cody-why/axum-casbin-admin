use rbatis::rbdc::DateTime;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysTrash {
    pub id: Option<String>,
    pub table_name: Option<String>,
    pub data: Option<String>,
    pub create_date: Option<DateTime>,
}

rbatis::crud!(SysTrash {});
rbatis::impl_delete!(SysTrash{ delete_by_day_before(before:DateTime) => "` where create_date < #{before}`"});
