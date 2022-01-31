CREATE TABLE IF NOT EXISTS `database`.`post_comment` (
  `uuid` VARCHAR(45) NOT NULL,
  `content` LONGTEXT NOT NULL,
  `author` VARCHAR(45) NOT NULL,
  `created_at` VARCHAR(45) NOT NULL,
  `updated_at` VARCHAR(45) NOT NULL,
  `post_uuid` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`uuid`),
  UNIQUE INDEX `id_UNIQUE` (`uuid` ASC),
  INDEX `fk_post_comment_post1_idx` (`post_uuid` ASC),
  CONSTRAINT `fk_post_comment_post1`
    FOREIGN KEY (`post_uuid`)
    REFERENCES `database`.`post` (`uuid`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB
