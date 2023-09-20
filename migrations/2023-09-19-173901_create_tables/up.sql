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

CREATE TABLE `types` (
                         `id` INT unsigned NOT NULL AUTO_INCREMENT,
                         `group_id` INT unsigned NOT NULL DEFAULT 0,
                         `name` varchar(255) NOT NULL DEFAULT '',
                         PRIMARY KEY (`id`),
                         UNIQUE KEY `types_name_UN` (`name`),
                         KEY `types_groups_FK` (`group_id`),
                         CONSTRAINT `types_groups_FK` FOREIGN KEY (`group_id`) REFERENCES `groups` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `categories` (
                              `id` INT unsigned NOT NULL AUTO_INCREMENT,
                              `type_id` INT unsigned NOT NULL DEFAULT 0,
                              `name` varchar(255) NOT NULL DEFAULT '',
                              PRIMARY KEY (`id`),
                              KEY `categories_types_FK` (`type_id`),
                              CONSTRAINT `categories_types_FK` FOREIGN KEY (`type_id`) REFERENCES `types` (`id`)
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