use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Album {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub image_url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateAlbum {
    pub title: String,
    pub description: String,
    pub image_url: String,
}
