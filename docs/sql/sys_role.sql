Drop table if exists sys_role;
create table sys_role
(
    id          int auto_increment comment '主键'
        primary key,
    role_name   varchar(50)                        not null comment '名称',
    status   tinyint  default 1                 not null comment '状态(1:正常，0:禁用)',
    remark      varchar(255)                       not null comment '备注',
    create_time datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间',
    constraint role_name
        unique (role_name)
)
    comment '角色信息';

create index name_status_index
    on sys_role (role_name, status);

INSERT INTO sys_role (id, role_name, status, remark) VALUES (1, '超级管理员', 1, '全部权限');
INSERT INTO sys_role (id, role_name, status, remark) VALUES (3, '演示角色', 1, '仅有查看功能');
