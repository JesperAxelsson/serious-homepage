-- Add migration script here
CREATE TABLE album2image (
    album_id BIGINT NOT NULL REFERENCES album(id),
    image_id BIGINT NOT NULL REFERENCES image(id),
    PRIMARY KEY (album_id,image_id)
);