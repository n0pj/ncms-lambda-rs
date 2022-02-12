CREATE TABLE IF NOT EXISTS `post_comment` (
  `uuid` VARCHAR(36) NOT NULL,
  `content` LONGTEXT NOT NULL,
  `created_at` VARCHAR(45) NOT NULL,
  `updated_at` VARCHAR(45) NOT NULL,
  `post_uuid` VARCHAR(36) NOT NULL,
  `user_uuid` VARCHAR(36) NOT NULL,
  PRIMARY KEY (`uuid`),
  INDEX `fk_post_comment_post_idx` (`post_uuid` ASC),
  INDEX `fk_post_comment_user1_idx` (`user_uuid` ASC),
  CONSTRAINT `fk_post_comment_post`
    FOREIGN KEY (`post_uuid`)
    REFERENCES `post` (`uuid`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_post_comment_user1`
    FOREIGN KEY (`user_uuid`)
    REFERENCES `user` (`uuid`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB
