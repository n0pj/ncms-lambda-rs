-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `database`.`post_score_kind` (
  `id` VARCHAR(45) NOT NULL,
  `created_at` VARCHAR(45) NOT NULL,
  `updated_at` VARCHAR(45) NOT NULL,
  `kind_uuid` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `id_UNIQUE` (`id` ASC),
  INDEX `fk_post_score_kind_kind1_idx` (`kind_uuid` ASC),
  CONSTRAINT `fk_post_score_kind_kind1`
    FOREIGN KEY (`kind_uuid`)
    REFERENCES `database`.`kind` (`uuid`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB
