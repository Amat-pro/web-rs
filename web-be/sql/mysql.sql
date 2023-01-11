-- --------------------------------------WEB-RS-----------------------------------------   

-- Note:
-- Default characterset: utf8mb4

-- user    
CREATE TABLE `user` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `nick_name` VARCHAR(255) NOT NULL DEFAULT '',
  `email` VARCHAR(255) NOT NULL DEFAULT '',
  `password` VARCHAR(1000) NOT NULL DEFAULT '',
  `is_delete` BOOLEAN NOT NULL DEFAULT false,
  `create_time` BIGINT NOT NULL DEFAULT 0,
  `update_time` BIGINT NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `nick_name_UNIQUE` (`nick_name` ASC) VISIBLE,
  UNIQUE INDEX `email_UNIQUE` (`email` ASC) VISIBLE)
ENGINE = InnoDB;

CREATE TABLE `user_article` (
  `id` BIGINT NOT NULL AUTO_INCREMENT,
  `article_id` BIGINT NOT NULL DEFAULT 0,
  `user_id` BIGINT NOT NULL DEFAULT 0,
  `is_delete` BOOLEAN NOT NULL DEFAULT false,
  `create_time` BIGINT NOT NULL DEFAULT 0,
  `update_time` BIGINT NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  INDEX `idx_user_id` USING BTREE (`user_id`) VISIBLE)
ENGINE = InnoDB;