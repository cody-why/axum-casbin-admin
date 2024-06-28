#![allow(clippy::needless_question_mark)]

use crate::*;
pub mod menu;
pub mod role;
pub mod role_menu;
pub mod trash;
pub mod user;
pub mod user_role;

const EXPIRE: u64 = 60 * 60;
impl_cache!(role::SysRole, EXPIRE);
impl_cache!(menu::SysMenu, EXPIRE);
impl_cache!(role_menu::SysRoleMenu, EXPIRE);
impl_cache!(user::SysUser, EXPIRE);
impl_cache!(user_role::SysUserRole, EXPIRE);

impl_cache_db!(menu::SysMenu);
impl_cache_db!(role::SysRole);
impl_cache_db!(user::SysUser);
impl_cache_db!(role_menu::SysRoleMenu);
impl_cache_db!(user_role::SysUserRole);
