CREATE TABLE IF NOT EXISTS `database`.`kind` (
  `uuid` VARCHAR(45) NOT NULL,
  `name` VARCHAR(45) NOT NULL,
  `created_at` VARCHAR(45) NOT NULL,
  `updated_at` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`uuid`),
  UNIQUE INDEX `name_UNIQUE` (`name` ASC),
  UNIQUE INDEX `id_UNIQUE` (`uuid` ASC))
ENGINE = InnoDB
