CREATE TABLE IF NOT EXISTS `database`.`post_kind` (
  `uuid` VARCHAR(45) NOT NULL,
  `created_at` VARCHAR(45) NOT NULL,
  `updated_at` VARCHAR(45) NOT NULL,
  `post_uuid` VARCHAR(45) NOT NULL,
  `kind_uuid` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`uuid`),
  UNIQUE INDEX `id_UNIQUE` (`uuid` ASC),
  INDEX `fk_post_kind_post1_idx` (`post_uuid` ASC),
  INDEX `fk_post_kind_kind1_idx` (`kind_uuid` ASC),
  CONSTRAINT `fk_post_kind_post1`
    FOREIGN KEY (`post_uuid`)
    REFERENCES `database`.`post` (`uuid`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_post_kind_kind1`
    FOREIGN KEY (`kind_uuid`)
    REFERENCES `database`.`kind` (`uuid`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB
