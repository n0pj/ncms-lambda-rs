CREATE TABLE IF NOT EXISTS `category` (
  `uuid` VARCHAR(36) NOT NULL,
  `name` VARCHAR(45) NOT NULL,
  `slug` VARCHAR(45) NOT NULL,
  `is_main` TINYINT NULL DEFAULT 0 COMMENT '0 = false, 1 = true, true ならメインカテゴリーとして扱い、false ならサブカテゴリーとして扱う',
  `created_at` VARCHAR(45) NOT NULL,
  `updated_at` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`uuid`),
  UNIQUE INDEX `name_UNIQUE` (`name` ASC))
ENGINE = InnoDB
