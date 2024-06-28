DROP TABLE IF EXISTS sys_user_role;
create table sys_user_role
(
    id          int auto_increment comment '主键'
        primary key,
    user_id     bigint UNSIGNED                    not null comment '用户ID',
    role_id     int                             not null comment '角色ID',
    status   tinyint  default 1                 not null comment '状态(1:正常，0:禁用)',
    create_time datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间',
    -- 关联user表
    foreign key (user_id) references sys_user(id) on delete cascade,
    -- 关联role表
    foreign key (role_id) references sys_role(id) on delete cascade
)
    comment '角色用户关联表';

INSERT INTO sys_user_role (id, user_id, role_id, status) VALUES (1, 2, 3, 1);
INSERT INTO sys_user_role (id, user_id, role_id, status) VALUES (2, 12, 3, 1);
INSERT INTO sys_user_role (id, user_id, role_id, status) VALUES (4, 3, 3, 1);
INSERT INTO sys_user_role (id, user_id, role_id, status) VALUES (5, 1, 1, 1);
