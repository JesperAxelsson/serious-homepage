use serde_derive::{Deserialize, Serialize};

// The query parameters for list_todos.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListOptions {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}
