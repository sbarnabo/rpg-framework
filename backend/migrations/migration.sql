-- migration.sql

CREATE TABLE inventory (
    id SERIAL PRIMARY KEY,
    player_id INT REFERENCES players(id),
    item_id INT REFERENCES items(id),
    quantity INT DEFAULT 1,
    durability INT DEFAULT 100, -- Default durability for items that have it
    is_cursed BOOLEAN DEFAULT FALSE  -- Cursed status for the player's item
);
