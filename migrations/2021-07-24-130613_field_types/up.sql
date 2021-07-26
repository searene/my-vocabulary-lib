-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `field_types` (
    `id` integer not null primary key autoincrement,
    `name` text,
    `category` text,
    `card_type_id` integer
);