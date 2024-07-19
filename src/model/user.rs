/*
 * @Date: 2024-06-28 15:21:48
 * @LastEditTime: 2024-07-18 16:45:14
 */

use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::db::ExecResult;
use rbatis::{impl_select, impl_select_page, py_sql, RBatis};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysUser {
    pub id: Option<u64>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub status: i32,
    // pub sort: i32,
    pub mobile: String,
    pub user_name: String,
    pub remark: Option<String>,
    pub password: String,
}

rbatis::crud!(SysUser {});

impl_select_page!(SysUser{select_page() =>"
     if do_count == false:
       order by create_time desc"});

impl_select_page!(SysUser{select_page_by_name(mobile: &str, status: Option<i32>) =>"
      where 1=1
     if mobile != null && mobile != '' :
       ` and mobile = #{mobile} `
     if status != null :
       ` and status = #{status} `
     if do_count == false:
        ` order by create_time desc `"});

// impl_select!(SysUser{select_by_id_db(id:u64) -> Option => "`where id = #{id} limit 1`"});

impl_select!(SysUser{select_by_mobile(mobile:&str) -> Option => "`where mobile = #{mobile} limit 1`"});

impl SysUser {
    #[py_sql("update sys_user set password = #{password} where id = #{id}")]
    pub async fn update_password(rb: &RBatis, id: u64, password: &str) -> Result<ExecResult, rbatis::Error> {}
}
