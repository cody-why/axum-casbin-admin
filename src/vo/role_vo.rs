use crate::model::menu::SysMenu;
use crate::model::role::SysRole;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RoleListReq {
    #[serde(rename = "pageNo")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    pub role_name: Option<String>,
    pub status: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RoleListData {
    pub id: i32,
    pub status: i32,
    pub role_name: String,
    pub remark: String,
    pub create_time: String,
    pub update_time: String,
}

impl From<SysRole> for RoleListData {
    fn from(role: SysRole) -> Self {
        Self {
            id: role.id.unwrap(),
            status: role.status,
            role_name: role.role_name,
            remark: role.remark.unwrap_or_default(),
            create_time: role.create_time.unwrap().to_string(),
            update_time: role.update_time.unwrap().to_string(),
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RoleSaveReq {
    pub role_name: String,
    pub status: i32,
    pub remark: Option<String>,
}

impl From<RoleSaveReq> for SysRole {
    fn from(role_req: RoleSaveReq) -> Self {
        let now = Some(DateTime::now());
        SysRole {
            id: None,
            status: role_req.status,
            role_name: role_req.role_name,
            remark: role_req.remark,
            create_time: now.clone(),
            update_time: now,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RoleUpdateReq {
    pub id: i32,
    pub status: i32,
    pub role_name: String,
    pub remark: Option<String>,
}

rbatis::impl_update!(RoleUpdateReq {}, "sys_role");

impl From<RoleUpdateReq> for SysRole {
    fn from(role_req: RoleUpdateReq) -> Self {
        let now = Some(DateTime::now());
        SysRole {
            id: Some(role_req.id),
            status: role_req.status,
            role_name: role_req.role_name,
            remark: role_req.remark,
            create_time: None,
            update_time: now,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RoleDeleteReq {
    pub ids: Vec<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct QueryRoleMenuReq {
    pub role_id: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct QueryRoleMenuData {
    pub role_menus: Vec<i32>,
    pub menu_list: Vec<MenuDataList>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MenuDataList {
    pub id: i32,
    pub parent_id: i32,
    pub title: String,
    pub key: String,
    // pub label: String,
    #[serde(rename = "isPenultimate")]
    pub is_penultimate: bool,
}

impl From<SysMenu> for MenuDataList {
    fn from(role: SysMenu) -> Self {
        Self {
            id: role.id.unwrap(),
            parent_id: role.parent_id,
            title: role.menu_name,
            key: role.id.unwrap().to_string(),
            is_penultimate: role.parent_id == 2,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateRoleMenuReq {
    pub menu_ids: Vec<i32>,
    pub role_id: i32,
}
