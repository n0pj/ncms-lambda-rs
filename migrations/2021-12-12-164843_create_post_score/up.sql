CREATE TABLE IF NOT EXISTS `database`.`post_score` (
  `uuid` VARCHAR(45) NOT NULL,
  `count` BIGINT NOT NULL,
  `created_at` VARCHAR(45) NOT NULL,
  `updated_at` VARCHAR(45) NOT NULL,
  `post_score_kind_id` VARCHAR(45) NOT NULL,
  `post_status_uuid` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`uuid`),
  UNIQUE INDEX `id_UNIQUE` (`uuid` ASC),
  INDEX `fk_post_score_post_score_kind1_idx` (`post_score_kind_id` ASC),
  INDEX `fk_post_score_post_status1_idx` (`post_status_uuid` ASC),
  CONSTRAINT `fk_post_score_post_score_kind1`
    FOREIGN KEY (`post_score_kind_id`)
    REFERENCES `database`.`post_score_kind` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_post_score_post_status1`
    FOREIGN KEY (`post_status_uuid`)
    REFERENCES `database`.`post_status` (`uuid`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB
