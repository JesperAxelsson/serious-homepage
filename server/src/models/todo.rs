use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: i64,
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateTodo {
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateTodo {
    pub text: String,
    pub completed: bool,
}
