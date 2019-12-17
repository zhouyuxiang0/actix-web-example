CREATE TABLE user(
    id INT NOT NULL AUTO_INCREMENT  COMMENT '主键' ,
    username VARCHAR(32)    COMMENT '用户名' ,
    password VARCHAR(32)  NOT NULL  COMMENT '用户密码' ,
    salt VARCHAR(32) NOT NULL   COMMENT '密码盐' ,
    realname VARCHAR(32)    COMMENT '真实姓名' ,
    cellphone VARCHAR(32)  NOT NULL  COMMENT '手机号' ,
    enable TINYINT(1)   DEFAULT 1 COMMENT '状态 默认 1 启用, 0 禁用' ,
    login_time DATETIME DEFAULT CURRENT_TIMESTAMP COMMENT '登陆时间',
    create_time DATETIME DEFAULT CURRENT_TIMESTAMP  COMMENT '创建时间' ,
    update_time DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间' ,
    PRIMARY KEY (id)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT = '用户表';
