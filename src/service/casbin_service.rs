use casbin::{CoreApi, MgmtApi, RbacApi};
// use casbin_rb_adapter::casbin;
use casbin_rb_adapter::vec_string;

use crate::e_read;
use crate::e_write;
use crate::model::menu::SysMenu;
use crate::Result;

pub struct CasbinService {}

impl CasbinService {
    /// 权限校验
    pub async fn enforce(user_id: impl ToString, path: &str, method: &str) -> bool {
        e_read!().enforce((&user_id.to_string(), path, method)).unwrap_or_else(|e| {
            tracing::error!("casbin enforce error: {}", e);
            false
        })
    }

    /// 获取角色的用户
    pub async fn get_users_for_role(role_id: impl ToString) -> Vec<String> {
        e_write!().get_users_for_role(&role_id.to_string(), None)
    }
    /// 获取用户的角色
    pub async fn get_roles_for_user(user_id: impl ToString) -> Vec<i32> {
        e_write!()
            .get_roles_for_user(&user_id.to_string(), None)
            .iter()
            .map(|x| x.trim_start_matches('r').parse().unwrap_or(0))
            .collect()
    }

    /// 获取用户的权限菜单ids
    pub async fn get_user_menu_ids(user_id: impl ToString) -> Vec<i32> {
        let ps = e_write!().get_implicit_permissions_for_user(&user_id.to_string(), None);
        // [["1", "*", "*", "id"]]
        ps.iter()
            .filter(|x| x[1] != "*")
            .map(|x| x[3].parse().unwrap_or_default())
            .collect()
    }
    /// 获取角色的权限菜单ids
    pub async fn get_role_menu_ids(role_id: impl std::fmt::Display) -> Vec<i32> {
        let ps = e_write!().get_implicit_permissions_for_user(&format!("r{role_id}"), None);
        // [["1", "*", "*", "id"]]
        ps.iter().filter(|x| x[1] != "*").map(|x| x[3].parse().unwrap_or(0)).collect()
    }

    /// 删除角色的权限
    pub async fn delete_roles_policy(role_ids: &Vec<i32>) -> Result<bool> {
        for role_id in role_ids {
            let r = e_write!().remove_filtered_policy(0, vec![format!("r{role_id}")]).await?;
            if !r {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// 更新角色的权限
    pub async fn update_role_policy(role_id: i32, menus: &[SysMenu]) -> Result<bool> {
        // delete role policy, add new policy
        // let r = e_write!().delete_permissions_for_user(&role_id).await?;
        let role_id = format!("r{role_id}");
        let _ = e_write!().remove_filtered_policy(0, vec_string![role_id]).await?;

        // [["1", "/admin/user", "*", "id"]]
        let pms = menus
            .iter()
            .filter(|x| !crate::is_empty!(x.api_url)) // 过滤掉目录
            .map(|x| {
                let api_url = crate::empty_or!(x.api_url, "/");
                vec_string![role_id, api_url, "*", x.id.unwrap()]
            })
            .collect();
        let r = e_write!().add_policies(pms).await?;
        Ok(r)
    }

    /// 更新用户的角色
    pub async fn update_user_roles(user_id: impl ToString, role_ids: &[i32]) -> Result<bool> {
        // 没有错误就是成功
        let _ = e_write!().remove_filtered_grouping_policy(0, vec_string![user_id]).await?;

        let pms = role_ids.iter().map(|x| vec_string![user_id, format!("r{x}")]).collect();
        let r = e_write!().add_grouping_policies(pms).await?;
        Ok(r)
    }
}
