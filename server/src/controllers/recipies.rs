use crate::{
    internal_error,
    models::{CreateRecipe, Recipe},
    session::SessionId,
    DatabaseConnection,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};

pub async fn get_recipe(
    Path(id): Path<i64>,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> impl IntoResponse {
    let rec = sqlx::query!(
        "select id, title, description, content from recipe where id = $1",
        id
    )
    .fetch_one(&mut conn)
    .await
    .map_err(internal_error);

    match rec {
        Ok(rec) => {
            let recipe = Recipe {
                id: rec.id,
                title: rec.title,
                description: rec.description,
                content: rec.content,
            };

            Ok(Json(recipe))
        }
        Err(e) => Err(e),
    }
}

pub async fn list_recipies(DatabaseConnection(mut conn): DatabaseConnection) -> String {
    let recipies = sqlx::query!("select id, title, description, content from recipe order by id")
        .fetch_all(&mut conn)
        .await
        .expect("Failed to execute list_recipies query")
        .into_iter()
        .map(|row| Recipe {
            id: row.id,
            title: row.title,
            description: row.description,
            content: row.content,
        })
        .collect::<Vec<_>>();

    serde_json::to_string(&recipies).expect("Failed to convert to list of recipies")
}

pub async fn create_recipe(
    _session_id: SessionId,
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(create): Json<CreateRecipe>,
) -> StatusCode {
    tracing::debug!("create_recipe: {:?}", create);

    let _rec = sqlx::query!(
        r#"
                INSERT INTO recipe (title, description, content)
                VALUES ($1, $2, $3)
                RETURNING id
            "#,
        create.title,
        create.description,
        create.content,
    )
    .fetch_one(&mut conn)
    .await
    .expect("Failed to insert new recipe");

    StatusCode::CREATED
}

pub async fn update_recipe(
    _session_id: SessionId,
    Path(id): Path<i64>,
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(update): Json<CreateRecipe>,
) -> StatusCode {
    tracing::debug!("update_recipe: id={}, recipe={:?}", id, update);

    let rec = sqlx::query!(
        r#"
                UPDATE recipe 
                SET title = $2, description = $3, content = $4
                WHERE id = $1
            "#,
        id,
        update.title,
        update.description,
        update.content,
    )
    .execute(&mut conn)
    .await
    .expect("Failed to update recipe");

    tracing::debug!("    -> recipe id not found!");

    // If the for loop didn't return OK, then the ID doesn't exist...
    if rec.rows_affected() == 1 {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn delete_recipe(
    _session_id: SessionId,
    Path(id): Path<i64>,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> StatusCode {
    tracing::debug!("delete_recipe: id={}", id);

    let rec = sqlx::query!(
        r#"
                DELETE FROM recipe 
                WHERE id = $1
            "#,
        id
    )
    .execute(&mut conn)
    .await
    .expect("Failed to update recipe");

    if rec.rows_affected() == 1 {
        // respond with a `204 No Content`, which means successful,
        // yet no body expected...
        StatusCode::NO_CONTENT
    } else {
        tracing::debug!("    -> recipe id not found!");
        StatusCode::NOT_FOUND
    }
}
