use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Recipe {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RecipeListItem {
    pub id: i64,
    pub title: String,
    pub description: String,
}