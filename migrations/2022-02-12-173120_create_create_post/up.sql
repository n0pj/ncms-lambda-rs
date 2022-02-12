CREATE TABLE IF NOT EXISTS `post` (
  `uuid` VARCHAR(36) NOT NULL,
  `title` VARCHAR(120) NULL,
  `password` VARCHAR(255) NULL,
  `content` LONGTEXT NOT NULL,
  `slug` VARCHAR(45) NULL,
  `created_at` VARCHAR(45) NOT NULL,
  `updated_at` VARCHAR(45) NOT NULL,
  `status_uuid` VARCHAR(36) NOT NULL,
  `user_uuid` VARCHAR(36) NOT NULL,
  PRIMARY KEY (`uuid`),
  INDEX `fk_post_status1_idx` (`status_uuid` ASC),
  INDEX `fk_post_user1_idx` (`user_uuid` ASC),
  CONSTRAINT `fk_post_status1`
    FOREIGN KEY (`status_uuid`)
    REFERENCES `status` (`uuid`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_post_user1`
    FOREIGN KEY (`user_uuid`)
    REFERENCES `user` (`uuid`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB
