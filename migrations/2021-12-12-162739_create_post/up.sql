CREATE TABLE IF NOT EXISTS `database`.`post` (
  `uuid` VARCHAR(45) NOT NULL,
  `title` VARCHAR(120) NULL,
  `password` VARCHAR(255) NULL,
  `content` LONGTEXT NOT NULL,
  `slug` VARCHAR(45) NOT NULL,
  `updated_at` VARCHAR(45) NOT NULL,
  `created_at` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`uuid`),
  UNIQUE INDEX `id_UNIQUE` (`uuid` ASC))
ENGINE = InnoDB
