-- Add migration script here

CREATE TABLE image (
    id INTEGER PRIMARY KEY,
    title varchar(50) NOT NULL,
    description varchar(150) NOT NULL,
    image_url varchar(512) NOT NULL,
    preview_url varchar(512) NOT NULL
);