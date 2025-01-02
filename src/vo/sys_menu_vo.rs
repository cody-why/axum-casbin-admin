use crate::model::sys_menu::SysMenu;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct MenuListReq {
    pub menu_name: Option<String>,
    pub status: Option<i32>,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct MenuListData {
    pub id: Option<i32>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
    pub parent_id: Option<i32>,
    pub menu_name: Option<String>,
    pub menu_url: Option<String>,
    pub icon: Option<String>,
    pub api_url: Option<String>,
    pub remark: Option<String>,
    pub menu_type: Option<i32>,
    // pub create_time: Option<String>,
    // pub update_time: Option<String>,
}

impl From<SysMenu> for MenuListData {
    fn from(menu: SysMenu) -> Self {
        Self {
            id: menu.id,
            sort: menu.sort,
            status: menu.status,
            parent_id: menu.parent_id,
            menu_name: menu.menu_name,
            menu_url: menu.menu_url,
            icon: menu.icon,
            api_url: menu.api_url,
            remark: menu.remark,
            menu_type: menu.menu_type,
            // create_time: format_date_option(menu.create_time),
            // update_time: format_date_option(menu.update_time),
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct MenuSaveReq {
    pub sort: Option<i32>,
    pub status: Option<i32>,
    pub parent_id: Option<i32>,
    pub menu_name: Option<String>,
    pub menu_url: Option<String>,
    pub icon: Option<String>,
    pub api_url: Option<String>,
    pub remark: Option<String>,
    pub menu_type: Option<i32>,
}

impl From<MenuSaveReq> for SysMenu {
    fn from(req: MenuSaveReq) -> Self {
        let now = Some(DateTime::now());
        Self {
            id: None,
            sort: req.sort,
            status: req.status,
            parent_id: req.parent_id,
            menu_name: req.menu_name,
            menu_url: req.menu_url,
            icon: req.icon,
            api_url: req.api_url,
            remark: req.remark,
            menu_type: req.menu_type,
            create_time: now.clone(),
            update_time: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MenuUpdateReq {
    pub id: i32,
    pub sort: Option<i32>,
    pub status: Option<i32>,
    pub parent_id: Option<i32>,
    pub menu_name: Option<String>,
    pub menu_url: Option<String>,
    // #[serde(rename(serialize = "menu_icon"))]
    pub icon: Option<String>,
    pub api_url: Option<String>,
    pub remark: Option<String>,
    pub menu_type: Option<i32>,
}

rbatis::impl_update!(MenuUpdateReq {}, "sys_menu");

impl From<MenuUpdateReq> for SysMenu {
    fn from(req: MenuUpdateReq) -> Self {
        let now = Some(DateTime::now());
        Self {
            id: Some(req.id),
            sort: req.sort,
            status: req.status,
            parent_id: req.parent_id,
            menu_name: req.menu_name,
            menu_url: req.menu_url,
            icon: req.icon,
            api_url: req.api_url,
            remark: req.remark,
            menu_type: req.menu_type,
            create_time: None,
            update_time: now,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct MenuDeleteReq {
    pub ids: Vec<i32>,
}
