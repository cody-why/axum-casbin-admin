#![allow(clippy::needless_question_mark)]

use crate::*;
pub mod sys_menu;
pub mod role;
pub mod sys_user;
// pub mod user_role;
pub mod sys_role_menu;
pub mod sys_trash;

const EXPIRE: u64 = 60 * 60;
impl_cache!(role::SysRole, EXPIRE);
impl_cache!(sys_menu::SysMenu, EXPIRE);
impl_cache!(sys_role_menu::SysRoleMenu, EXPIRE);
impl_cache!(sys_user::SysUser, EXPIRE);
// impl_cache!(user_role::SysUserRole, EXPIRE);

impl_cache_db!(sys_menu::SysMenu);
impl_cache_db!(role::SysRole);
impl_cache_db!(sys_user::SysUser);
impl_cache_db!(sys_role_menu::SysRoleMenu);
// impl_cache_db!(user_role::SysUserRole);
