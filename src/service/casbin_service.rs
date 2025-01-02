use casbin::{CoreApi, MgmtApi, RbacApi};
use casbin_rb_adapter::vec_string;

use crate::casbin_read;
use crate::casbin_write;
use crate::model::sys_menu::SysMenu;
use crate::Result;

pub struct CasbinService {}

impl CasbinService {
    /// 权限校验
    pub async fn enforce(user_id: impl ToString, path: &str, method: &str) -> bool {
        casbin_read!()
            .enforce((&user_id.to_string(), path, method))
            .unwrap_or_else(|e| {
                tracing::error!("casbin enforce error: {}", e);
                false
            })
    }

    /// 获取角色的用户
    pub async fn get_users_for_role(role_id: impl ToString) -> Vec<String> {
        casbin_read!().get_users_for_role(&role_id.to_string(), None)
    }
    /// 获取用户的角色
    pub async fn get_roles_for_user(user_id: impl ToString) -> Vec<i32> {
        casbin_write!()
            .get_roles_for_user(&user_id.to_string(), None)
            .iter()
            .map(|x| x.trim_start_matches('r').parse().unwrap_or(0))
            .collect()
    }

    /// 获取用户的权限菜单ids
    pub async fn get_user_menu_ids(user_id: impl ToString) -> Vec<u64> {
        let ps = casbin_write!().get_implicit_permissions_for_user(&user_id.to_string(), None);

        // [["1", "*", "*", "id"]]
        ps.iter()
            .filter(|x| x[1] != "*")
            .map(|x| x[3].parse().unwrap_or_default())
            .collect()
    }
    /// 获取角色的权限菜单ids
    pub async fn get_role_menu_ids(role_id: impl std::fmt::Display) -> Vec<u64> {
        let ps = casbin_write!().get_implicit_permissions_for_user(&format!("r{role_id}"), None);
        // [["1", "*", "*", "id"]]
        ps.iter().filter(|x| x[1] != "*").map(|x| x[3].parse().unwrap_or(0)).collect()
    }

    /// 删除角色的权限
    pub async fn delete_roles_policy(role_ids: &[i32]) -> Result<bool> {
        for id in role_ids {
            let role_id = format!("r{id}");
            casbin_write!().remove_filtered_policy(0, vec![role_id]).await?;
        }

        Ok(true)
    }

    /// 更新角色的权限
    pub async fn update_role_policy(role_id: u64, menus: &[SysMenu]) -> Result<bool> {
        let role_id = format!("r{role_id}");

        use std::collections::HashSet;

        // 获取当前策略
        let current_policies = {
            let policies = casbin_read!().get_filtered_policy(0, vec![role_id.clone()]);
            policies.into_iter().collect::<HashSet<_>>()
        };

        let mut new_policies = HashSet::with_capacity(menus.len());

        // 优化策略构建
        // [["1", "/admin/user", "*", "id"]]
        for menu in menus
            .iter()
            .filter(|x| x.menu_type.is_some_and(|x| x == 3) && !crate::is_empty!(x.api_url))
        {
            let api_url = crate::empty_or!(menu.api_url, "/");
            let policy = if let Some((method, path)) = api_url.split_once(' ') {
                vec_string![&role_id, path, method, menu.id.unwrap()]
            } else {
                vec_string![&role_id, api_url, "*", menu.id.unwrap()]
            };
            new_policies.insert(policy);
        }

        // 找出差异
        let policies_to_remove: Vec<_> =
            current_policies.difference(&new_policies).cloned().collect();

        let policies_to_add: Vec<_> = new_policies.difference(&current_policies).cloned().collect();

        if !policies_to_remove.is_empty() {
            casbin_write!().remove_policies(policies_to_remove).await?;
        }
        if !policies_to_add.is_empty() {
            casbin_write!().add_policies(policies_to_add).await?;
        }

        Ok(true)
    }

    /// 更新用户的角色
    pub async fn update_user_roles(user_id: impl ToString, role_ids: &[u64]) -> Result<bool> {
        // 获取当前用户已有的角色
        let mut e = casbin_write!();
        let current_roles = e.get_roles_for_user(&user_id.to_string(), None);
        drop(e);
        // 将新的角色ID转换为预期的格式
        let new_roles: Vec<String> = role_ids.iter().map(|x| format!("r{x}")).collect();

        // 找出需要删除的角色
        let roles_to_remove: Vec<String> =
            current_roles.iter().filter(|r| !new_roles.contains(r)).cloned().collect();

        // 找出需要新增的角色
        let roles_to_add: Vec<Vec<String>> = new_roles
            .iter()
            .filter(|r| !current_roles.contains(r))
            .map(|r| vec_string![user_id, r])
            .collect();

        // 如果有需要删除的角色,执行删除操作
        if !roles_to_remove.is_empty() {
            for role in roles_to_remove {
                casbin_write!().remove_grouping_policy(vec_string![user_id, role]).await?;
            }
        }

        // 如果有需要新增的角色,执行新增操作
        if !roles_to_add.is_empty() {
            casbin_write!().add_grouping_policies(roles_to_add).await?;
        }

        Ok(true)
    }

    /// 删除用户的角色
    pub async fn delete_user_role(user_id: impl ToString) -> Result<bool> {
        casbin_write!().remove_filtered_grouping_policy(0, vec_string![user_id]).await?;
        Ok(true)
    }
}
