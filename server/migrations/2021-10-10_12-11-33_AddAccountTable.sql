-- Add migration script here 
CREATE TABLE IF NOT EXISTS account (
    id          BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    password    TEXT NOT NULL,
    salt        TEXT NOT NULL
);
