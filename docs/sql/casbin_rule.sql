CREATE TABLE IF NOT EXISTS casbin_rule (
  id INT NOT NULL AUTO_INCREMENT,
  ptype VARCHAR(12) NOT NULL,
  v0 VARCHAR(128) NOT NULL,
  v1 VARCHAR(128) NOT NULL,
  v2 VARCHAR(128) NOT NULL,
  v3 VARCHAR(128) NOT NULL,
  v4 VARCHAR(128) NOT NULL,
  v5 VARCHAR(128) NOT NULL,
  PRIMARY KEY(id),
  CONSTRAINT unique_key_casbin_adapter UNIQUE(ptype, v0, v1, v2, v3, v4, v5)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;

insert into casbin_rule (ptype, v0, v1, v2, v3, v4, v5) values ('p', 'r1', '*', '*', '0', '', '');
insert into casbin_rule (ptype, v0, v1, v2, v3, v4, v5) values ('g', '1', 'r1', '', '', '', '');