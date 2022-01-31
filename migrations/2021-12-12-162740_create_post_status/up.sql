CREATE TABLE IF NOT EXISTS `database`.`post_status` (
  `uuid` VARCHAR(45) NOT NULL,
  `created_at` VARCHAR(45) NOT NULL,
  `updated_at` VARCHAR(45) NOT NULL,
  `status_uuid` VARCHAR(45) NOT NULL,
  `post_uuid` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`uuid`),
  UNIQUE INDEX `id_UNIQUE` (`uuid` ASC),
  INDEX `fk_post_status_status1_idx` (`status_uuid` ASC),
  INDEX `fk_post_status_post1_idx` (`post_uuid` ASC),
  CONSTRAINT `fk_post_status_status1`
    FOREIGN KEY (`status_uuid`)
    REFERENCES `database`.`status` (`uuid`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_post_status_post1`
    FOREIGN KEY (`post_uuid`)
    REFERENCES `database`.`post` (`uuid`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB
