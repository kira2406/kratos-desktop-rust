-- Your SQL goes here
CREATE TABLE games (
    game_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    price NUMERIC NOT NULL,
    discount FLOAT8,
    release_date DATE,
    publisher_id UUID,
    rating FLOAT8,
    reviews TEXT,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ,
    FOREIGN KEY (publisher_id) REFERENCES publishers(publisher_id) ON DELETE SET NULL
);
