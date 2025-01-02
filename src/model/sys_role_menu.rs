use std::collections::HashMap;

use rbatis::rbdc::datetime::DateTime;
use rbatis::sql;
use rbatis::{impl_select_page, rbatis::RBatis};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysRoleMenu {
    pub id: Option<i32>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub status: i32,
    pub menu_id: i32,
    pub role_id: i32,
}

impl SysRoleMenu {
    pub fn new(role_id: i32, menu_id: i32) -> Self {
        let now = Some(DateTime::now());
        Self {
            id: None,
            create_time: now.clone(),
            update_time: now,
            status: 1,
            menu_id,
            role_id,
        }
    }
}

rbatis::crud!(SysRoleMenu {});
impl_select_page!(SysRoleMenu{select_page() =>"
     if do_count == false:
       order by create_time desc"});

#[sql("select menu_id from sys_role_menu where role_id = ?")]
pub async fn query_menu_by_role(rb: &RBatis, role_id: i32) -> rbatis::Result<Vec<HashMap<String, i32>>> {
    impled!()
}
