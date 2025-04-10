-- 20230415125000_create_minions_table.sql

CREATE TABLE minions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    owner_id INT REFERENCES players(id),
    minion_type VARCHAR(50),
    health INT,
    power INT,
    abilities TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
