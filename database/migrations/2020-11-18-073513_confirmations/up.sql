-- Your SQL goes here
CREATE TABLE confirmations (
    id UUID NOT NULL PRIMARY KEY,
    username VARCHAR(32) NOT NULL,
    email VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP NOT NULL,
)