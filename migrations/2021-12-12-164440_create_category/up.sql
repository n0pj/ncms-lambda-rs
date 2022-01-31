CREATE TABLE IF NOT EXISTS `database`.`category` (
  `uuid` VARCHAR(45) NOT NULL,
  `name` VARCHAR(45) NOT NULL,
  `slug` VARCHAR(45) NOT NULL,
  `created_at` VARCHAR(45) NOT NULL,
  `updated_at` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`uuid`),
  UNIQUE INDEX `name_UNIQUE` (`name` ASC),
  UNIQUE INDEX `slug_UNIQUE` (`slug` ASC),
  UNIQUE INDEX `uuid_UNIQUE` (`uuid` ASC))
ENGINE = InnoDB
