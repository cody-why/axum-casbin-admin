use crate::model::menu::SysMenu;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

use crate::model::role::SysRole;
use crate::model::user::SysUser;

#[derive(Debug, Deserialize)]
pub struct UserLoginReq {
    pub mobile: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct QueryUserRoleReq {
    pub user_id: u64,
}

#[derive(Debug, Serialize)]
pub struct QueryUserRoleData {
    pub sys_role_list: Vec<UserRoleList>,
    pub user_role_ids: Vec<i32>,
}

#[derive(Debug, Serialize)]
pub struct UserRoleList {
    pub id: i32,
    pub status: i32,
    pub role_name: String,
    pub remark: String,
    pub create_time: String,
    pub update_time: String,
}

impl From<SysRole> for UserRoleList {
    fn from(x: SysRole) -> Self {
        Self {
            id: x.id.unwrap(),
            status: x.status,
            role_name: x.role_name,
            remark: x.remark.unwrap_or_default(),
            create_time: x.create_time.unwrap().to_string(),
            update_time: x.update_time.unwrap().to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRoleReq {
    pub user_id: u64,
    pub role_ids: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct QueryUserMenuReq {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct QueryUserMenuResp {
    pub msg: String,
    pub code: i32,
    pub data: QueryUserMenuData,
    pub success: bool,
}

#[derive(Debug, Serialize)]
pub struct QueryUserMenuData {
    pub sys_menu: Vec<MenuUserList>,
    pub btn_menu: Vec<String>,
    pub avatar: String,
    pub name: String,
}

#[derive(Debug, Serialize, Clone)]
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
#[derive(Debug, Deserialize)]
pub struct UserListReq {
    #[serde(rename = "pageNo")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    pub mobile: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResp {
    pub msg: String,
    pub code: i32,
    pub success: bool,
    pub total: u64,
    pub data: Option<Vec<UserListData>>,
}

#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct UserDeleteReq {
    pub ids: Vec<u64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPwdReq {
    pub id: u64,
    pub password: String,
    pub new_password: String,
}
