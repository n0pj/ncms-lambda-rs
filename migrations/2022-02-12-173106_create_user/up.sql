CREATE TABLE IF NOT EXISTS `user` (
  `uuid` VARCHAR(36) NOT NULL,
  `name` VARCHAR(45) NULL,
  `email` VARCHAR(100) NULL,
  `display_name` VARCHAR(45) NULL,
  `password` VARCHAR(255) NULL,
  `google_authenticator_secret` VARCHAR(255) NULL,
  `created_at` VARCHAR(45) NOT NULL,
  `updated_at` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`uuid`),
  UNIQUE INDEX `email_UNIQUE` (`email` ASC))
ENGINE = InnoDB
