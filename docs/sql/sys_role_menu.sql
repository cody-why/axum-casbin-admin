DROP TABLE IF EXISTS sys_role_menu;

create table sys_role_menu
(
    id          int auto_increment primary key,
    role_id     int                             not null comment '角色ID',
    menu_id     int                             not null comment '菜单ID',
    status   tinyint  default 1                 not null comment '状态(1:正常，0:禁用)',
    create_time datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间',
    -- 联合唯一索引
    unique key idx_role_menu (role_id, menu_id)
) comment '菜单角色关联';



