use rbatis::{impl_select, impl_select_page, rbdc::datetime::DateTime, RBatis};
use serde::{Deserialize, Serialize};

use rbatis::rbatis_codegen::IntoSql;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SysMenu {
    pub id: Option<i32>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub status: i32,
    pub sort: i32,
    pub parent_id: i32,
    pub menu_name: String,
    pub menu_url: Option<String>,
    pub api_url: Option<String>,
    pub icon: Option<String>,
    pub remark: Option<String>,
    pub menu_type: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SysMenuUrl {
    pub api_url: Option<String>,
}

rbatis::crud!(SysMenu {});

impl_select_page!(SysMenu{select_page() =>"
     if do_count == false:
       order by create_time asc"});

// impl_select!(SysMenu{select_by_id(id:i32) -> Option => "`where id = #{id} limit 1`"});

impl_select!(SysMenu{select_by_ids(ids:&[i32]) -> Vec => "`where id in ${ids.sql()}  and status = 1 order by sort asc`"});

impl SysMenu {
    #[rbatis::sql(
        "select m.* from sys_user_role ur
               left join sys_role r on ur.role_id = r.id
               left join sys_role_menu rm on r.id = rm.role_id
               left join sys_menu m on rm.menu_id = m.id where ur.user_id = ?"
    )]
    pub async fn select_by_user_id(rb: &RBatis, user_id: u64) -> Result<Vec<SysMenu>, rbatis::Error> {}
}

#[cfg(test)]
mod tests {
    use crate::pool;

    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_select() {
        let rb = pool!();
        let vec = SysMenu::select_by_user_id(rb, 2).await.unwrap();

        println!("{:?}", vec);
    }
}
