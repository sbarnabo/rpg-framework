-- 20230415123500_create_items_table.sql

CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    durability INT DEFAULT 100,
    is_magical BOOLEAN DEFAULT FALSE,
    is_cursed BOOLEAN DEFAULT FALSE,
    item_type VARCHAR(50),
    power INT,
    value INT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
