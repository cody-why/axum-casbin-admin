use rbatis::{impl_select_page, rbdc::datetime::DateTime};
use serde::{Deserialize, Serialize};

// user_role
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysUserRole {
    pub id: Option<i32>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub status: i32,
    pub role_id: i32,
    pub user_id: i64,
}

rbatis::crud!(SysUserRole {});
impl_select_page!(SysUserRole{select_page() =>"
     if do_count == false:
       order by create_time desc"});

impl_select_page!(SysUserRole{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       where user_name != #{name}
     if name == '':
       where user_name != ''"});

// 查询是否为超级管理员(role_id=1是预设超级管理的id)
rbatis::impl_select!(SysUserRole{is_admin(user_id:u64) => "`where user_id = #{user_id} and role_id = 1`"});
