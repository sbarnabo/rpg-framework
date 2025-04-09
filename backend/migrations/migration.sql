-- migration.sql

CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    item_type VARCHAR(255) NOT NULL,
    value INT,
    durability INT DEFAULT NULL,   -- Nullable column for durability (only for items like weapons and armor)
    is_magical BOOLEAN DEFAULT FALSE, -- Boolean for magical items
    is_cursed BOOLEAN DEFAULT FALSE  -- Boolean for cursed items
);
