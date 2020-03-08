use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: i64,
    pub text: String,
    pub completed: bool,
}