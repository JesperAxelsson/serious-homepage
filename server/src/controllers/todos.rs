use crate::{
    models::{CreateTodo, ListOptions, UpdateTodo},
    repo::todo_repo,
    session::SessionId,
    DatabaseConnection,
};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct UpdateTodo {
//     pub text: String,
//     pub completed: bool,
// }

pub async fn get_todo(
    _session_id: SessionId,
    Path(id): Path<i64>,
    DatabaseConnection(conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
    let res = todo_repo::get_todo(conn, id).await;

    match res {
        Ok(Some(todo)) => Ok(serde_json::to_string(&todo).expect("Failed to parse to json")),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Not found".to_owned())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

pub async fn list_todos(
    Query(opts): Query<ListOptions>,
    DatabaseConnection(conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
    let todos = todo_repo::list_todos(conn, opts.limit, opts.offset).await;

    match todos {
        Ok(todos) => Ok(serde_json::to_string(&todos).expect("Failed to parse to json")),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

pub async fn create_todo(
    DatabaseConnection(conn): DatabaseConnection,
    Json(create): Json<CreateTodo>,
) -> StatusCode {
    todo_repo::create_todo(conn, &create.text).await;
    StatusCode::CREATED
}

pub async fn update_todo(
    Path(id): Path<i64>,
    DatabaseConnection(conn): DatabaseConnection,
    Json(update): Json<UpdateTodo>,
) -> StatusCode {
    tracing::debug!("update_todo: id={}, todo={:?}", id, update);
    if todo_repo::update_todo(conn, id, &update.text, update.completed).await {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn delete_todo(
    Path(id): Path<i64>,
    DatabaseConnection(conn): DatabaseConnection,
) -> StatusCode {
    tracing::debug!("delete_todo: id={}", id);

    if todo_repo::delete_todo(conn, id).await {
        StatusCode::NO_CONTENT
    } else {
        tracing::debug!("    -> todo id not found!");
        StatusCode::NOT_FOUND
    }
}
