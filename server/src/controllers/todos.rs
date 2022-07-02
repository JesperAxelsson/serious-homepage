use crate::{
    internal_error,
    models::{CreateTodo, ListOptions, Todo, UpdateTodo},
    DatabaseConnection, session::SessionId,
};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};

pub async fn get_todo(
    _session_id: SessionId,
    Path(id): Path<i64>,
    DatabaseConnection(conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
    let mut conn = conn;
    let res = sqlx::query!("select id, text, completed from todo where id = $1", id)
        .fetch_one(&mut conn)
        .await
        .map_err(internal_error);

    match res {
        Ok(rec) => {
            let todo = Todo {
                id: rec.id,
                text: rec.text,
                completed: rec.completed,
            };

            Ok(serde_json::to_string(&todo).expect("Failed to parse to json"))
        }
        Err(e) => Err(e),
    }
}

pub async fn list_todos(
    Query(opts): Query<ListOptions>,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
    let todos = sqlx::query!(
        "select id, text, completed from todo order by id limit $1 offset $2",
        opts.limit.unwrap_or(std::i64::MAX),
        opts.offset.unwrap_or(0)
    )
    .fetch_all(&mut conn)
    .await
    .expect("Failed to execute list_todos query")
    .into_iter()
    .map(|row| Todo {
        id: row.id,
        text: row.text,
        completed: row.completed,
    })
    .collect::<Vec<_>>();

    Ok(serde_json::to_string(&todos).expect("Failed to parse to json"))
}

pub async fn create_todo(
    Json(create): Json<CreateTodo>,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> StatusCode {
    log::debug!("create_todo: {:?}", create);
    let _rec = sqlx::query!(
        r#"
                INSERT INTO todo (text)
                VALUES ($1)
                RETURNING id
            "#,
        create.text
    )
    .fetch_one(&mut conn)
    .await
    .expect("Failed to insert new TODO");

    StatusCode::CREATED
}

pub async fn update_todo(
    Path(id): Path<i64>,
    Json(update): Json<UpdateTodo>,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> StatusCode {
    log::debug!("update_todo: id={}, todo={:?}", id, update);

    let rec = sqlx::query!(
        r#"
                UPDATE todo
                SET completed = $2, text = $3
                WHERE id = $1
            "#,
        id,
        update.completed,
        update.text
    )
    .execute(&mut conn)
    .await
    .expect("Failed to update TODO");

    log::debug!("    -> todo id not found!");

    // If the for loop didn't return OK, then the ID doesn't exist...
    if rec.rows_affected() == 1 {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn delete_todo(
    Path(id): Path<i64>,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> StatusCode {
    log::debug!("delete_todo: id={}", id);

    let rec = sqlx::query!(
        r#"
                DELETE FROM todo
                WHERE id = $1
            "#,
        id
    )
    .execute(&mut conn)
    .await
    .expect("Failed to update TODO");

    if rec.rows_affected() == 1 {
        StatusCode::NO_CONTENT
    } else {
        log::debug!("    -> todo id not found!");
        StatusCode::NOT_FOUND
    }
}
