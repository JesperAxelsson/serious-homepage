use sqlx::{pool::PoolConnection, Postgres};

use crate::models::Todo;

pub async fn get_todo(conn: PoolConnection<Postgres>, id: i64) -> Result<Option<Todo>, String> {
    let mut conn = conn;
    let res = sqlx::query!("select id, text, completed from todo where id = $1", id)
        .fetch_optional(&mut *conn)
        .await;

    match res {
        Ok(Some(rec)) => {
            let todo = Todo {
                id: rec.id,
                text: rec.text,
                completed: rec.completed,
            };
            Ok(Some(todo))
        }
        Ok(None) => Ok(None),
        Err(_) => Err("Big error!".to_owned()),
    }
}

pub async fn list_todos(
    conn: PoolConnection<Postgres>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Todo>, String> {
    let mut conn = conn;

    let todos = sqlx::query!(
        "select id, text, completed from todo order by id limit $1 offset $2",
        limit.unwrap_or(std::i64::MAX),
        offset.unwrap_or(0)
    )
    .fetch_all(&mut *conn)
    .await
    .expect("Failed to execute list_todos query")
    .into_iter()
    .map(|row| Todo {
        id: row.id,
        text: row.text,
        completed: row.completed,
    })
    .collect::<Vec<_>>();

    Ok(todos)
}

pub async fn create_todo(conn: PoolConnection<Postgres>, text: &str) -> bool {
    let mut conn = conn;
    let _rec = sqlx::query!(
        r#"
                INSERT INTO todo (text)
                VALUES ($1)
                RETURNING id
            "#,
        text
    )
    .fetch_one(&mut *conn)
    .await
    .expect("Failed to insert new TODO");

    println!("Created: {:?}", _rec);

    true
}

pub async fn update_todo(
    conn: PoolConnection<Postgres>,
    id: i64,
    text: &str,
    completed: bool,
) -> bool {
    let mut conn = conn;
    let rec = sqlx::query!(
        r#"
                UPDATE todo
                SET completed = $2, text = $3
                WHERE id = $1
            "#,
        id,
        completed,
        text
    )
    .execute(&mut *conn)
    .await
    .expect("Failed to update TODO");

    // If the for loop didn't return OK, then the ID doesn't exist...
    rec.rows_affected() == 1
}

pub async fn delete_todo(conn: PoolConnection<Postgres>, id: i64) -> bool {
    let mut conn = conn;

    let rec = sqlx::query!(
        r#"
                DELETE FROM todo
                WHERE id = $1
            "#,
        id
    )
    .execute(&mut *conn)
    .await
    .expect("Failed to update TODO");

    rec.rows_affected() == 1
}
