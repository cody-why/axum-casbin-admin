use crate::middleware::context::UserContext;
use crate::model::menu::SysMenu;
use crate::model::role::SysRole;
use crate::model::user::SysUser;
use crate::service::login_service;
use crate::utils::jwt_util::JWTToken;
use crate::utils::password::Password;
use crate::vo::user_vo::*;
use crate::Result;
use crate::{context, pool, Error};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rbatis::plugin::page::PageRequest;
use rbatis::Page;
use std::collections::HashSet;

use super::casbin_service::CasbinService;
use super::role_service::get_role_menu_ids;

// 用户登录
#[tracing::instrument(ret, err)]
pub async fn login(item: UserLoginReq) -> Result<String> {
    let try_num = login_service::is_need_wait_login_ex(&item.mobile).await?;

    let rb = pool!();

    let user_result = SysUser::select_by_mobile(rb, &item.mobile).await?;
    // info!("select_by_mobile: {:?}", user_result);

    let user = match user_result {
        Some(user) => user,
        None => return Error::err("用户不存在"),
    };
    if !Password::verify(&item.password, &user.password) {
        login_service::add_retry_login_limit_num(&item.mobile).await?;

        return Error::err("密码不正确");
    }
    if try_num > 0 {
        login_service::remove_retry_login_limit_num(&item.mobile).await?;
    }
    if user.status != 1 {
        return Error::err("用户被禁用");
    }

    let id = user.id.unwrap();
    let username = user.user_name;

    // let btn_menu = query_user_btn_menu(id).await;

    let token = JWTToken::new(id, &username, vec![]).create_token()?;
    Ok(token)
}


pub async fn query_user_role(item: QueryUserRoleReq) -> Result<QueryUserRoleData> {
    let rb = pool!();

    // let user_role = SysUserRole::select_all_cache(rb).await?;
    // let user_role_ids: Vec<i32> = user_role.iter().filter(|x| x.user_id ==
    // item.user_id)     .map(|x| x.role_id).collect();

    let user_role_ids = CasbinService::get_roles_for_user(item.user_id).await;

    let sys_role = SysRole::select_all_cache(rb).await?;
    let sys_role_list: Vec<UserRoleList> = sys_role.into_iter().map(|x| x.into()).collect();

    let result = QueryUserRoleData {
        sys_role_list,
        user_role_ids,
    };
    Ok(result)
}

pub async fn update_user_role(item: UpdateUserRoleReq) -> Result<bool> {
    let user_id = item.user_id;
    if user_id == 1 {
        return Error::err("不能修改超级管理员");
    }

    let role_ids = item.role_ids;
    // let len = role_ids.len();
    // let rb = pool!();
    // let _ = SysUserRole::delete_by_column(rb, "user_id", user_id).await?;
    // let time = Some(DateTime::now());
    // let mut sys_role_user_list: Vec<SysUserRole> = Vec::new();
    // for role_id in role_ids {
    //     sys_role_user_list.push(SysUserRole {
    //         id: None,
    //         create_time: time.clone(),
    //         update_time: time.clone(),
    //         status: 1,
    //         role_id,
    //         user_id,
    //     })
    // }
    // let result = SysUserRole::insert_batch(rb, &sys_role_user_list, len as u64).await?;
    // SysUserRole::remove_cached();

    let _ = CasbinService::update_user_roles(user_id, &role_ids).await?;

    Ok(true)
}

pub async fn query_user_menu(content: UserContext) -> Result<QueryUserMenuData> {
    let rb = pool!();
    let result = SysUser::select_by_id(rb, content.id).await?;

    let user = match result.first() {
        None => return Error::err("用户不存在"),
        Some(user) => user,
    };

    // role_id为1是超级管理员
    let roles = CasbinService::get_roles_for_user(content.id).await;
    let is_admin = roles.contains(&1);
    let sys_menu_list: Vec<SysMenu> = if is_admin {
        SysMenu::select_all_cache(rb).await?
    } else {
        // let sys_menu_list = SysMenu::select_by_user_id(rb, content.id).await?;
        // let menu_ids = CasbinService::get_user_menu_ids(content.id).await;
        let menu_ids = get_role_menu_ids(&roles).await?;
        // tracing::debug!("menu_ids: {:?}", menu_ids);
        let sys_menu_list = SysMenu::select_all_cache(rb).await?;
        sys_menu_list
            .into_iter()
            .filter(|x| {
                if let Some(id) = x.id {
                    return menu_ids.contains(&id);
                }
                false
            })
            .collect()
    };
    let mut btn_menu: Vec<String> = Vec::new();
    let mut sys_menu_ids: HashSet<i32> = HashSet::new();

    for x in &sys_menu_list {
        // 菜单类型为3的为按钮
        if x.menu_type != 3 {
            if let Some(id) = x.id {
                sys_menu_ids.insert(id);
            }
            if x.parent_id != 0 {
                sys_menu_ids.insert(x.parent_id);
            }
        }
        if let Some(api_url) = &x.api_url {
            if !api_url.is_empty() {
                btn_menu.push(api_url.to_owned());
            }
        }
    }
    let sys_menu: Vec<MenuUserList> = sys_menu_list
        .into_iter()
        .filter(|x| sys_menu_ids.contains(&x.id.unwrap_or_default()))
        .map(|x| x.into())
        .collect();

    let resp = QueryUserMenuData {
        sys_menu,
        btn_menu,
        avatar: "".to_string(),
        name: user.user_name.clone(),
    };
    Ok(resp)
}

// 查询用户列表
pub async fn user_list(item: UserListReq) -> Result<Page<UserListData>> {
    let rb = pool!();

    let mobile = item.mobile.as_deref().unwrap_or_default();
    let status = item.status.as_deref().unwrap_or_default();
    let page_req = PageRequest::new(item.page_no, item.page_size);
    let result = SysUser::select_page_by_name(rb, &page_req, mobile, status).await?;
    let page = Page::<UserListData>::from(result);
    Ok(page)
}

// 添加用户信息
pub async fn user_save(item: UserSaveReq) -> Result<u64> {
    let rb = pool!();
    let mut sys_user = SysUser::from(item);
    let password = context().config.default_password.as_str();
    sys_user.password = Password::md5_and_hash(password);

    let result = SysUser::insert(rb, &sys_user).await?;

    Ok(result.rows_affected)
}

// 更新用户信息
pub async fn user_update(item: UserUpdateReq) -> Result<u64> {
    if item.id == 1 {
        return Error::err("不能修改超级管理员");
    }
    let rb = pool!();
    let result = SysUser::select_by_id(rb, item.id).await?;

    match result.first() {
        None => Error::err("用户不存在"),
        Some(_user) => {
            let result = UserUpdateReq::update_by_column(rb, &item, "id").await?;
            SysUser::remove_cache_by(format!("id_{}", item.id));
            Ok(result.rows_affected)
        },
    }
}

// 删除用户信息
pub async fn user_delete(item: UserDeleteReq) -> Result<u64> {
    let rb = pool!();
    //id为1的用户为系统预留用户,不能删除
    let ids: Vec<u64> = item.ids.par_iter().filter(|x| **x != 1).cloned().collect();
    if ids.is_empty() {
        return Ok(0);
    }

    for id in &ids {
        SysUser::remove_cache_by(format!("id_{}", id));
    }
    let result = SysUser::delete_in_column(rb, "id", &ids).await?;

    Ok(result.rows_affected)
}

// 更新用户密码
pub async fn update_user_password(item: UpdateUserPwdReq) -> Result<u64> {
    let rb = pool!();

    let result = SysUser::select_by_id(rb, item.id).await?;

    match result.first() {
        None => Error::err("用户不存在"),
        Some(user) => {
            if !Password::verify(&item.password, &user.password) {
                return Error::err("密码不正确");
            }
            let id = user.id.unwrap();
            let password = Password::hash(&item.new_password);
            let result = SysUser::update_password(rb, id, &password).await?;
            SysUser::remove_cache_by(format!("id_{}", id));
            if result.rows_affected == 1 {
                Ok(1)
            } else {
                Error::err("修改失败")
            }
        },
    }
}
