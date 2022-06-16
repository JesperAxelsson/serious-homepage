use axum::{extract::Path, http::StatusCode, Json};

use crate::{
    models::{Album, CreateAlbum, Image},
    DatabaseConnection,
};

pub async fn get_album(
    Path(id): Path<i64>,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> (StatusCode, String) {
    let images = sqlx::query!(
        r#"select * from image i
            inner join album2image ai on i.id = ai.image_id
            where ai.album_id = $1"#,
        id
    )
    .fetch_all(&mut conn)
    .await
    .expect("Failed to execute list_gallery query")
    .into_iter()
    .map(|row| Image {
        id: row.id,
        title: row.title,
        description: row.description,
        image_url: row.image_url,
        preview_url: String::from(""),
    })
    .collect::<Vec<_>>();

    (
        StatusCode::OK,
        serde_json::to_string(&images).expect("Failed to parse to json"),
    )
}

pub async fn list_album(DatabaseConnection(mut conn): DatabaseConnection) -> (StatusCode, String) {
    let gallery = sqlx::query!("select id, title, description, image_url from album order by id",)
        .fetch_all(&mut conn)
        .await
        .expect("Failed to execute list_gallery query")
        .into_iter()
        .map(|row| Album {
            id: row.id,
            title: row.title,
            description: row.description,
            image_url: row.image_url.unwrap_or(String::from("")),
        })
        .collect::<Vec<_>>();

    (
        StatusCode::OK,
        serde_json::to_string(&gallery).expect("Failed to parse to json"),
    )
}

pub async fn create_album(
    Json(new_album): Json<CreateAlbum>,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> StatusCode {
    log::debug!("create_todo: {:?}", new_album);

    let _rec = sqlx::query!(
        r#"
            INSERT INTO public.album
            (title, description, image_url)
            VALUES ($1, $2, $3)
            RETURNING id
        "#,
        new_album.title,
        new_album.description,
        new_album.image_url,
    )
    .fetch_one(&mut conn)
    .await
    .expect("Failed to insert new TODO");

    StatusCode::CREATED
}
