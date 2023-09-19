CREATE TABLE `groups` (
                          `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
                          `name` varchar(255) DEFAULT NULL,
                          PRIMARY KEY (`id`),
                          UNIQUE KEY `groups_name_UN` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `locations` (
                             `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
                             `name` varchar(255) DEFAULT NULL,
                             `description` text DEFAULT NULL,
                             PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `boxes` (
                         `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
                         `location_id` int(11) unsigned DEFAULT NULL,
                         `description` text DEFAULT NULL,
                         PRIMARY KEY (`id`),
                         KEY `boxes_locations_FK` (`location_id`),
                         CONSTRAINT `boxes_locations_FK` FOREIGN KEY (`location_id`) REFERENCES `locations` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `types` (
                         `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
                         `group_id` int(11) unsigned DEFAULT NULL,
                         `name` varchar(255) DEFAULT NULL,
                         PRIMARY KEY (`id`),
                         UNIQUE KEY `types_name_UN` (`name`),
                         KEY `types_groups_FK` (`group_id`),
                         CONSTRAINT `types_groups_FK` FOREIGN KEY (`group_id`) REFERENCES `groups` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `categories` (
                              `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
                              `type_id` int(11) unsigned DEFAULT NULL,
                              `name` varchar(255) DEFAULT NULL,
                              PRIMARY KEY (`id`),
                              KEY `categories_types_FK` (`type_id`),
                              CONSTRAINT `categories_types_FK` FOREIGN KEY (`type_id`) REFERENCES `types` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `items` (
                         `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
                         `category_id` int(11) unsigned DEFAULT NULL,
                         `name` varchar(255) DEFAULT NULL,
                         `date` date DEFAULT NULL,
                         `description` text DEFAULT NULL,
                         `sealed` tinyint(1) NOT NULL,
                         `rate` smallint(5) unsigned NOT NULL DEFAULT 0,
                         `box_id` int(11) unsigned DEFAULT 0,
                         PRIMARY KEY (`id`),
                         KEY `items_boxes_FK` (`box_id`),
                         KEY `items_categories_FK` (`category_id`),
                         CONSTRAINT `items_boxes_FK` FOREIGN KEY (`box_id`) REFERENCES `boxes` (`id`),
                         CONSTRAINT `items_categories_FK` FOREIGN KEY (`category_id`) REFERENCES `categories` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
