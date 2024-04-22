-- Your SQL goes here
CREATE TABLE users(
    id BIGSERIAL PRIMARY KEY,
    --------
    name VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    token BIGINT NOT NULL,
    is_admin BOOLEAN NOT NULL
);