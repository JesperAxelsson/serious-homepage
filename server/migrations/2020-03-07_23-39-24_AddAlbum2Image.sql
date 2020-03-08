-- Add migration script here
CREATE TABLE album2image (
    album_id INTEGER NOT NULL REFERENCES album(id),
    image_id INTEGER NOT NULL REFERENCES image(id),
    PRIMARY KEY (album_id,image_id)
);