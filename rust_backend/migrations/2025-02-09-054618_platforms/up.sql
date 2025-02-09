-- Your SQL goes here
CREATE TABLE platforms (
    platform_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL
);
