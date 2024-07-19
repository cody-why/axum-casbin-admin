/*
 * @Date: 2024-06-28 15:21:48
 * @LastEditTime: 2024-07-18 16:56:32
 */

use rbatis::{impl_select_page, rbdc::datetime::DateTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysRole {
    pub id: Option<i32>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub status: i32,
    pub role_name: String,
    pub remark: Option<String>,
}

rbatis::crud!(SysRole {});
impl_select_page!(SysRole{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"});

impl_select_page!(SysRole{select_page_by_name(role_name: &str, status: Option<i32>) =>"
      where 1=1
     if role_name != null && role_name != '':
       ` and role_name = #{role_name} `
     if status != null :
       ` and status = #{status} `
     if do_count == false:
        ` order by create_time desc `"});
