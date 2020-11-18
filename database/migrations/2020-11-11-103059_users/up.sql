-- Your SQL goes here
CREATE TABLE users (
    id UUID NOT NULL PRIMARY KEY,
    username VARCHAR(32) NOT NULL,
    email VARCHAR(255) NOT NULL,
    pass_hash VARCHAR(255) NOT NULL,
    avatar TEXT,
    bio VARCHAR(255),
    big_bio VARCHAR(8000)
    created_at TIMESTAMP NOT NULL
)