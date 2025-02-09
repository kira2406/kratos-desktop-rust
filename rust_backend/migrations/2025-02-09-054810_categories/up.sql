-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Create the categories table
CREATE TABLE categories (
    category_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT
);