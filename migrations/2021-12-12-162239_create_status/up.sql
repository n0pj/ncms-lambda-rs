CREATE TABLE IF NOT EXISTS `database`.`status` (
  `uuid` VARCHAR(45) NOT NULL,
  `name` VARCHAR(45) NOT NULL,
  `created_at` VARCHAR(45) NOT NULL,
  `updated_at` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`uuid`),
  UNIQUE INDEX `id_UNIQUE` (`uuid` ASC))
ENGINE = InnoDB
