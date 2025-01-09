DROP TABLE IF EXISTS sys_user_role;
create table sys_user_role
(
    id          int auto_increment comment '主键'
        primary key,
    user_id     bigint                          not null comment '用户ID',
    role_id     int                             not null comment '角色ID',
    status   tinyint  default 1                 not null comment '状态(1:正常，0:禁用)',
    create_time datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间'
)
    comment '角色用户关联';

INSERT INTO sys_user_role (id, user_id, role_id, status) VALUES (1, 1, 1, 1);

