use crate::model::menu::SysMenu;
use crate::model::role::SysRole;
use crate::model::role_menu::SysRoleMenu;
use crate::model::user_role::SysUserRole;
use crate::vo::role_vo::*;
use crate::{pool, Error, Result};
use rbatis::plugin::page::PageRequest;
use rbatis::Page;

// 查询角色列表
pub async fn role_list(item: RoleListReq) -> Result<Page<RoleListData>> {
    let rb = pool!();

    let role_name = item.role_name.as_deref().unwrap_or_default();
    let status = item.status.as_deref().unwrap_or_default();

    let page_req = PageRequest::new(item.page_no, item.page_size);
    let result = SysRole::select_page_by_name(rb, &page_req, role_name, status).await?;

    let page = Page::<RoleListData>::from(result);
    Ok(page)
}

// 添加角色信息
pub async fn role_save(item: RoleSaveReq) -> Result<u64> {
    let rb = pool!();

    let sys_role = SysRole::from(item);
    let result = SysRole::insert(rb, &sys_role).await?;
    Ok(result.rows_affected)
}

// 更新角色信息
pub async fn role_update(item: RoleUpdateReq) -> Result<u64> {
    let rb = pool!();

    // let sys_role = SysRole::from(item);
    let result = RoleUpdateReq::update_by_column(rb, &item, "id").await?;
    Ok(result.rows_affected)
}

// 删除角色信息
pub async fn role_delete(item: RoleDeleteReq) -> Result<u64> {
    let rb = pool!();

    let ids = item.ids;
    let user_role = SysUserRole::select_all_cache(rb).await?;
    let have = user_role.iter().any(|x| ids.contains(&x.role_id));
    if have {
        return Error::err("角色已被使用,不能删除");
    }

    let mut tx = rb.acquire_begin().await?;
    let result = SysRole::delete_in_column(&tx, "id", &ids).await?;
    SysRoleMenu::delete_in_column(&tx, "role_id", &ids).await?;
    tx.commit().await?;

    Ok(result.rows_affected)
}

pub async fn get_role_menu_ids(role_id: &[i32]) -> Result<Vec<i32>> {
    let rb = pool!();
    let role_menu = SysRoleMenu::select_all_cache(rb).await?;
    let role_menu_ids: Vec<i32> = role_menu
        .iter()
        .filter(|x| role_id.contains(&x.role_id))
        .map(|x| x.menu_id)
        .collect();
    Ok(role_menu_ids)
}

// 查询角色关联的菜单
pub async fn query_role_menu(item: QueryRoleMenuReq) -> Result<QueryRoleMenuData> {
    let rb = pool!();

    // 查询所有菜单
    let menu_list = SysMenu::select_all_cache(rb).await?;

    let mut menu_data_list: Vec<MenuDataList> = Vec::new();

    for y in menu_list {
        menu_data_list.push(y.into());
    }
    // 超级管理员默认拥有所有菜单权限
    let role_menu_ids = if item.role_id == 1 {
        menu_data_list.iter().map(|x| x.id).collect()
    } else {
        // let role_menu = SysRoleMenu::select_all_cache(rb).await?;
        // role_menu.iter().filter(|x| x.role_id == item.role_id)
        //     .map(|x| x.menu_id).collect()
        // CasbinService::get_role_menu_ids(item.role_id).await
        get_role_menu_ids(&[item.role_id]).await?
    };

    let result = QueryRoleMenuData {
        role_menus: role_menu_ids,
        menu_list: menu_data_list,
    };
    Ok(result)
}

// 更新角色关联的菜单
pub async fn update_role_menu(item: UpdateRoleMenuReq) -> Result<u64> {
    if item.role_id == 1 {
        return Error::err("不能修改超级管理员");
    }
    let role_id = item.role_id;

    let rb = pool!();
    let roles = SysRole::select_by_id(rb, item.role_id as u64).await?;
    if roles.is_empty() {
        return Error::err("角色不存在");
    }

    let _ = SysRoleMenu::delete_by_column(rb, "role_id", &role_id).await?;

    let len = item.menu_ids.len();
    let role_menu: Vec<SysRoleMenu> = item.menu_ids.iter().map(|x| SysRoleMenu::new(role_id, *x)).collect();

    let result = SysRoleMenu::insert_batch(rb, &role_menu, len as u64).await?;
    SysRoleMenu::remove_cached();

    Ok(result.rows_affected)
}
