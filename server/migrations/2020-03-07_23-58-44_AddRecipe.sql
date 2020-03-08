-- Add migration script here

CREATE TABLE recipe (
    id INTEGER PRIMARY KEY,
    title varchar(50) NOT NULL,
    description varchar(150) NOT NULL,
    content text NOT NULL
);
