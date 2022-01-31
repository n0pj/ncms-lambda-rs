CREATE TABLE IF NOT EXISTS `database`.`post_user` (
  `uuid` VARCHAR(45) NOT NULL,
  `created_at` VARCHAR(45) NOT NULL,
  `updated_at` VARCHAR(45) NOT NULL,
  `user_uuid` VARCHAR(45) NOT NULL,
  `post_uuid` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`uuid`),
  UNIQUE INDEX `id_UNIQUE` (`uuid` ASC),
  INDEX `fk_post_user_user_idx` (`user_uuid` ASC),
  INDEX `fk_post_user_post1_idx` (`post_uuid` ASC),
  CONSTRAINT `fk_post_user_user`
    FOREIGN KEY (`user_uuid`)
    REFERENCES `database`.`user` (`uuid`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_post_user_post1`
    FOREIGN KEY (`post_uuid`)
    REFERENCES `database`.`post` (`uuid`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB
