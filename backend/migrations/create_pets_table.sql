-- 20230415124500_create_pets_table.sql

CREATE TABLE pets (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    owner_id INT REFERENCES players(id),
    pet_type VARCHAR(50),
    health INT,
    power INT,
    abilities TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
