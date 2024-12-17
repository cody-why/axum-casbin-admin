use crate::model::menu::SysMenu;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::model::user::SysUser;

use super::role_vo::RoleListData;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserLoginReq {
    #[schema(example = "18500000000")]
    pub mobile: String,
    #[schema(example = "e10adc3949ba59abbe56e057f20f883e")]
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct QueryUserRoleReq {
    pub user_id: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct QueryUserRoleData {
    pub sys_role_list: Vec<RoleListData>,
    pub user_role_ids: Vec<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUserRoleReq {
    pub user_id: u64,
    pub role_ids: Vec<u64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct QueryUserMenuData {
    pub sys_menu: Vec<MenuUserList>,
    pub btn_menu: Vec<String>,
    pub avatar: String,
    pub name: String,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct MenuUserList {
    pub id: i32,
    pub parent_id: i32,
    pub name: String,
    pub path: String,
    pub api_url: String,
    pub menu_type: i32,
    pub icon: String,
}

impl From<SysMenu> for MenuUserList {
    fn from(menu: SysMenu) -> Self {
        Self {
            id: menu.id.unwrap(),
            parent_id: menu.parent_id,
            name: menu.menu_name,
            icon: menu.icon.unwrap_or_default(),
            api_url: menu.api_url.unwrap_or_default(),
            menu_type: menu.menu_type,
            path: menu.menu_url.unwrap_or_default(),
        }
    }
}
#[derive(Debug, Deserialize, ToSchema)]
pub struct UserListReq {
    #[serde(rename = "pageNo")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    pub mobile: Option<String>,
    pub status: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserListData {
    pub id: u64,
    pub status: i32,
    pub mobile: String,
    pub user_name: String,
    pub remark: String,
    pub create_time: String,
    pub update_time: String,
}

impl From<SysUser> for UserListData {
    fn from(user: SysUser) -> Self {
        Self {
            id: user.id.unwrap(),
            status: user.status,
            mobile: user.mobile,
            user_name: user.user_name,
            remark: user.remark.unwrap_or_default(),
            create_time: user.create_time.unwrap().to_string(),
            update_time: user.update_time.unwrap().to_string(),
        }
    }
}
#[derive(Debug, Deserialize, ToSchema)]
pub struct UserSaveReq {
    pub mobile: String,
    pub user_name: String,
    pub status: i32,
    pub remark: Option<String>,
}

impl From<UserSaveReq> for SysUser {
    fn from(item: UserSaveReq) -> Self {
        let now = Some(DateTime::now());
        Self {
            id: None,
            create_time: now.clone(),
            update_time: now,
            status: item.status,
            mobile: item.mobile,
            user_name: item.user_name,
            remark: item.remark,
            password: "123456".to_string(), //默认密码为123456
        }
    }
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserUpdateReq {
    pub id: u64,
    pub status: i32,
    pub mobile: String,
    // deserialize rename, see https://serde.rs/field-attrs.html#rename
    // #[serde(rename(deserialize = "real_name"))]
    pub user_name: String,
    pub remark: Option<String>,
}

rbatis::impl_update!(UserUpdateReq {}, "sys_user");

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserDeleteReq {
    pub ids: Vec<u64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUserPwdReq {
    pub id: u64,
    pub password: String,
    pub new_password: String,
}
