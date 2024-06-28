DROP TABLE IF EXISTS `sys_trash`;

CREATE TABLE `sys_trash` (
  `id` varchar(50) PRIMARY KEY,
  `table_name` varchar(50) NOT NULL,
  `data` text NOT NULL,
  `create_date` datetime DEFAULT CURRENT_TIMESTAMP
) ;

