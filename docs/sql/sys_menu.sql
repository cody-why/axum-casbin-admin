DROP TABLE IF EXISTS sys_menu;

create table sys_menu
(
    id          int auto_increment primary key,
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
    constraint menu_name_uindex unique (menu_name)
) AUTO_INCREMENT = 1000 comment '菜单信息';

-- 权限管理
insert into sys_menu(id,menu_name,menu_type,status,sort,parent_id,menu_url,api_url,icon,remark) values 
('1','首页','1','1','0','0','/home','','HomeOutlined','首页'),
('2','权限管理','1','1','2','0','/permission','','SettingOutlined','目录'),
-- ('3','基础管理','1','1','1','0','/base','','SettingOutlined','目录'),
('4','日志管理','1','1','4','0','/log','','SnippetsOutlined','目录'),


-- 用户管理
('10','用户管理','2','1','1','2','/user','','UserOutlined','菜单'),
('11','查询用户','3','1','1','10','','POST /admin/user/list','','接口'),
('12','保存用户','3','1','1','10','','POST /admin/user','','接口'),
('13','删除用户','3','1','1','10','','DELETE /admin/user','','接口'),
('14','更新用户','3','1','1','10','','PUT /admin/user','','接口'),
('15','重置密码','3','1','1','10','','PUT /admin/user/reset_password','','接口'),
('16','更新用户密码','3','1','1','10','','PUT /admin/user/update_password','','接口'),
('17','保存用户角色','3','1','1','10','','PUT /admin/user/role','','接口'),
('18','用户关联的角色','3','1','1','10','','POST /admin/user/role','','接口'),

-- 角色管理
('30','角色管理','2','1','1','2','/role','','AuditOutlined','菜单'),
('31','查询角色','3','1','1','30','','POST /admin/role/list','','接口'),
('32','保存角色','3','1','1','30','','POST /admin/role','','接口'),
('33','删除角色','3','1','1','30','','DELETE /admin/role','','接口'),
('34','更新角色','3','1','1','30','','PUT /admin/role','','接口'),
('35','查询角色菜单','3','1','1','30','','POST /admin/role/menu','','接口'),
('36','保存角色菜单','3','1','1','30','','PUT /admin/role/menu','','接口'),

-- 菜单管理
('50','菜单管理','2','1','3','2','/menu','','MenuOutlined','菜单'),
('51','查询菜单','3','1','1','50','','POST /admin/menu/list','','接口'),
('52','保存菜单','3','1','1','50','','POST /admin/menu','','接口'),
('53','更新菜单','3','1','1','50','','PUT /admin/menu','','接口'),
('54','删除菜单','3','1','1','50','','DELETE /admin/menu','','接口'),


('64','登录日志','2','1','1','4','/user','','','菜单'),
('65','常用图表','1','1','1','0','/line1','','DashboardOutlined','目录'),
('66','饼图','2','1','1','65','/pie','','','菜单'),
('67','线图','2','1','1','65','/line','','','菜单'),
('68','柱状图','2','1','1','65','/bar','','','菜单'),
('69','个人中心','1','1','1','0','/center1','','UserOutlined','目录'),
('72','个人信息','2','1','1','69','/center','','','菜单'),
('73','个人设置','2','1','1','69','/setting','','','菜单');