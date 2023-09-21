DROP TABLE IF EXISTS `items`;
DROP TABLE IF EXISTS `categories`;
DROP TABLE IF EXISTS `types`;
DROP TABLE IF EXISTS `kinds`;
DROP TABLE IF EXISTS `containers`;
DROP TABLE IF EXISTS `locations`;
DROP TABLE IF EXISTS `groups`;

CREATE TABLE `groups` (
                          `id` INT unsigned NOT NULL AUTO_INCREMENT,
                          `name` varchar(255) NOT NULL DEFAULT '',
                          PRIMARY KEY (`id`),
                          UNIQUE KEY `groups_name_UN` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `locations` (
                             `id` INT unsigned NOT NULL AUTO_INCREMENT,
                             `name` varchar(255) NOT NULL DEFAULT '',
                             `description` text NOT NULL DEFAULT '',
                             PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `containers` (
                         `id` INT unsigned NOT NULL AUTO_INCREMENT,
                         `location_id` INT unsigned NOT NULL DEFAULT 0,
                         `description` text NOT NULL DEFAULT '',
                         PRIMARY KEY (`id`),
                         KEY `containers_locations_FK` (`location_id`),
                         CONSTRAINT `containers_locations_FK` FOREIGN KEY (`location_id`) REFERENCES `locations` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `kinds` (
                         `id` INT unsigned NOT NULL AUTO_INCREMENT,
                         `group_id` INT unsigned NOT NULL DEFAULT 0,
                         `name` varchar(255) NOT NULL DEFAULT '',
                         PRIMARY KEY (`id`),
                         UNIQUE KEY `kinds_name_UN` (`name`),
                         KEY `kinds_groups_FK` (`group_id`),
                         CONSTRAINT `kinds_groups_FK` FOREIGN KEY (`group_id`) REFERENCES `groups` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `categories` (
                              `id` INT unsigned NOT NULL AUTO_INCREMENT,
                              `kind_id` INT unsigned NOT NULL DEFAULT 0,
                              `name` varchar(255) NOT NULL DEFAULT '',
                              PRIMARY KEY (`id`),
                              KEY `categories_kinds_FK` (`kind_id`),
                              CONSTRAINT `categories_kinds_FK` FOREIGN KEY (`kind_id`) REFERENCES `kinds` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `items` (
                         `id` INT unsigned NOT NULL AUTO_INCREMENT,
                         `category_id` INT unsigned NOT NULL DEFAULT 0,
                         `name` varchar(255) NOT NULL DEFAULT '',
                         `date` date NOT NULL DEFAULT 0,
                         `description` text NOT NULL DEFAULT '',
                         `sealed` BOOL NOT NULL DEFAULT 0,
                         `rate` TINYINT unsigned NOT NULL DEFAULT 0,
                         `container_id` INT unsigned NOT NULL DEFAULT 0,
                         PRIMARY KEY (`id`),
                         KEY `items_boxes_FK` (`container_id`),
                         KEY `items_categories_FK` (`category_id`),
                         CONSTRAINT `items_boxes_FK` FOREIGN KEY (`container_id`) REFERENCES `containers` (`id`),
                         CONSTRAINT `items_categories_FK` FOREIGN KEY (`category_id`) REFERENCES `categories` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

insert into funil.locations
select * from equipefunil.locations;

insert into funil.containers
select * from equipefunil.boxes;

insert into funil.groups
select * from equipefunil.groups;

insert into funil.kinds
select * from equipefunil.types;

insert into funil.categories
select * from equipefunil.categories;

insert into funil.items
select * from equipefunil.items;