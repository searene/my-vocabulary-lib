-- Your SQL goes here
CREATE TABLE `field_types` (
    `id` integer not null primary key autoincrement,
    `name` text,
    `category` text,
    `card_type_id` integer
);