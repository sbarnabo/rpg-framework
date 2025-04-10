-- Vehicles
CREATE TABLE vehicles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    vehicle_type VARCHAR(50),  -- e.g., magical, mechanical
    world_restriction VARCHAR(255), -- worlds the vehicle can be used in
    damage INT,  -- how much damage the vehicle can inflict
    special_abilities TEXT[],  -- array of abilities (e.g., weaponry)
    owner_id INT,  -- Owner of the vehicle (foreign key to the player)
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
