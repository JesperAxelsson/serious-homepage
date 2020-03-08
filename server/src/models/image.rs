
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Image {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub preview_url: String,
}
