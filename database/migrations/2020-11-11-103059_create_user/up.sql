-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(32) NOT NULL,
    pass_hash VARCHAR(255) NOT NULL,
    avatar TEXT,
    bio VARCHAR(255),
    big_bio TEXT
)