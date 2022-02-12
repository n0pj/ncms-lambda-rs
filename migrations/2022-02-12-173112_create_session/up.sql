CREATE TABLE IF NOT EXISTS `session` (
  `uuid` VARCHAR(36) NOT NULL,
  `bearer_token` VARCHAR(255) NOT NULL,
  `expired_at` VARCHAR(45) NOT NULL,
  `created_at` VARCHAR(45) NOT NULL,
  `updated_at` VARCHAR(45) NOT NULL,
  `user_uuid` VARCHAR(36) NOT NULL,
  PRIMARY KEY (`uuid`),
  UNIQUE INDEX `bearer_token_UNIQUE` (`bearer_token` ASC),
  INDEX `fk_session_user1_idx` (`user_uuid` ASC),
  CONSTRAINT `fk_session_user1`
    FOREIGN KEY (`user_uuid`)
    REFERENCES `user` (`uuid`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB
