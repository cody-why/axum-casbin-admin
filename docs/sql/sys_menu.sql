DROP TABLE IF EXISTS sys_menu;
create table sys_menu
(
    id          int auto_increment comment '主键'
        primary key,
    menu_name   varchar(50)                            not null comment '菜单名称',
    menu_type   tinyint      default 1                 not null comment '菜单类型(1：目录   2：菜单   3：按钮)',
    status      tinyint      default 1                 not null comment '状态(1:正常，0:禁用)',
    sort        int          default 1                 not null comment '排序',
    parent_id   int                                 not null comment '父ID',
    menu_url    varchar(255) default ''                null comment '路由路径',
    api_url     varchar(255) default ''                null comment '接口URL',
    icon        varchar(255)                           null comment '菜单图标',
    remark      varchar(255)                           null comment '备注',
    create_time datetime     default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime     default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间',
    constraint menu_name_uindex
        unique (menu_name)
)
    comment '菜单信息';


insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('1','首页','1','1','0','0','/home','','HomeOutlined','首页');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('2','权限管理','1','1','1','0','/permission','','SettingOutlined','权限管理');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('3','用户管理','2','1','3','2','/user','','UserOutlined','用户管理');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('4','角色管理','2','1','2','2','/role','','AuditOutlined','角色管理');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('5','菜单管理','2','1','1','2','/menu','','MenuOutlined','菜单管理');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('6','查询用户','3','1','1','3','','/admin/user_list','','查询用户接口');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('7','更新用户状态','3','1','1','3','','/admin/update_user_status','','更新用户状态接口');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('8','保存用户弹窗','3','1','1','3','','/admin/user_save_view','','保存用户弹窗');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('9','保存用户','3','1','1','3','','/admin/user_save','','保存用户接口');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('10','删除用户','3','1','1','3','','/admin/user_delete','','删除用户接口');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('11','更新用户弹窗','3','1','1','3','','/admin/user_update_view','','更新用户弹窗');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('12','更新用户','3','1','1','3','','/admin/user_update','','更新用户接口');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('13','更新用户密码弹窗','3','1','1','3','','/admin/update_user_password_view','','更新用户密码弹窗');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('14','更新用户密码','3','1','1','3','','/admin/update_user_password','','更新用户密码接口');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('15','设置角色弹窗','3','1','1','3','','/admin/update_user_role_view','','设置角色弹窗');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('16','保存用户角色','3','1','1','3','','/admin/update_user_role','','保存用户角色接口');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('17','用户关联的角色','3','1','1','3','','/admin/query_user_role','','获取用户关联的角色');
-- 角色管理
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('18','查询角色','3','1','1','4','','/admin/role_list','','查询角色接口');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('19','更新角色状态','3','1','1','4','','/admin/update_role_status','','更新角色状态接口');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('20','保存角色弹窗','3','1','1','4','','/admin/role_save_view','','保存角色弹窗');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('21','保存角色','3','1','1','4','','/admin/role_save','','保存角色接口');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('22','删除角色','3','1','1','4','','/admin/role_delete','','删除角色接口');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('23','修改角色弹窗','3','1','1','4','','/admin/role_update_view','','修改角色弹窗');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('24','更新角色','3','1','1','4','','/admin/role_update','','更新角色接口');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('25','设置权限弹窗','3','0','1','4','','/admin/query_role_menu_view','','设置权限弹窗');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('26','菜单角色关联','3','1','1','4','','/admin/query_role_menu','','菜单角色关联');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('27','保存角色菜单关联','3','1','1','4','','/admin/update_role_menu','','角色菜单关联接口');
-- 菜单管理
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('28','查询菜单','3','1','1','5','','/admin/menu_list','','查询菜单接口');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('29','更新菜单状态','3','1','1','5','','/admin/update_menu_status','','更新菜单状态接口');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('30','保存菜单弹窗','3','1','1','5','','/admin/menu_save_view','','保存菜单弹窗');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('31','保存菜单','3','1','1','5','','/admin/menu_save','','保存菜单接口');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('32','删除菜单','3','1','1','5','','/admin/menu_delete','','删除菜单接口');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('33','修改菜单弹窗','3','1','1','5','','/admin/menu_update_view','','修改菜单弹窗');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('34','更新菜单','3','1','1','5','','/admin/menu_update','','更新菜单接口');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('63','日志管理','1','1','1','0','/log1','','SnippetsOutlined','');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('64','登录日志','2','1','1','63','/log','','','');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('65','常用图表','1','1','1','0','/line1','','DashboardOutlined','');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('66','饼图','2','1','1','65','/pie','','','');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('67','线图','2','1','1','65','/line','','','');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('68','柱状图','2','1','1','65','/bar','','','');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('69','个人中心','1','1','1','0','/center1','','UserOutlined','');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('72','个人信息','2','1','1','69','/center','','','');
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values('73','个人设置','2','1','1','69','/setting','','','');
