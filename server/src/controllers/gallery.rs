use axum::{
    body::StreamBody,
    extract::Path,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use tokio_util::io::ReaderStream;

use crate::{
    file::path_is_valid,
    models::{Album, CreateAlbum, Image},
    session::SessionId,
    DatabaseConnection,
};

const IMAGE_DIRECTORY: &str = "gallery";

pub async fn get_album(
    Path(id): Path<i64>,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> (StatusCode, String) {
    tracing::debug!("get_album: {}", id);
    let images = sqlx::query!(
        r#"select * from image i
            inner join album2image ai on i.id = ai.image_id
            where ai.album_id = $1"#,
        id
    )
    .fetch_all(&mut *conn)
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
    tracing::debug!("list_album");
    let gallery = sqlx::query!("select id, title, description, image_url from album order by id",)
        .fetch_all(&mut *conn)
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
    _session_id: SessionId,
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(new_album): Json<CreateAlbum>,
) -> StatusCode {
    tracing::debug!("create_album: {:?}", new_album);

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
    .fetch_one(&mut *conn)
    .await
    .expect("Failed to insert new TODO");

    StatusCode::CREATED
}

pub async fn update_album(
    _session_id: SessionId,
    Path(id): Path<i64>,
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(new_album): Json<CreateAlbum>,
) -> StatusCode {
    tracing::debug!("update_album: {:?}", new_album);

    let _rec = sqlx::query!(
        r#"
            UPDATE public.album
            SET title=$2, description=$3, image_url=$4
            WHERE id = $1
        "#,
        id,
        new_album.title,
        new_album.description,
        new_album.image_url,
    )
    .execute(&mut *conn)
    .await
    .expect("Failed to insert new TODO");

    StatusCode::OK
}

pub async fn delete_album(
    _session_id: SessionId,
    Path(id): Path<i64>,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> StatusCode {
    tracing::debug!("delete_album: id={}", id);

    let rec = sqlx::query!(
        r#"
                DELETE FROM public.album
                WHERE id = $1
            "#,
        id
    )
    .execute(&mut *conn)
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

pub async fn get_image(
    Path((id, file_name)): Path<(i64, String)>,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> impl IntoResponse {
    tracing::debug!("Get gallery image");

    println!("Start query");
    let gallery = sqlx::query!(
        "select id, title, description, image_url from album where id = $1",
        id
    )
    .fetch_optional(&mut *conn)
    .await
    .expect("Failed to execute gallery get_image query");

    let Some(gallery) = gallery else {
        return Err((StatusCode::BAD_REQUEST, "Invalid gallery".to_owned()));
    };

    println!("Goit gallery {:?}", gallery);
    let gallery = sanitize_filename::sanitize(gallery.title);
    let file_name = sanitize_filename::sanitize(file_name);

    println!("FileName: {}", file_name);
    // if !path_is_valid(&file_name) {
    //     return Err((StatusCode::BAD_REQUEST, "Invalid path".to_owned()));
    // }

    let path = std::path::Path::new(IMAGE_DIRECTORY)
        .join(gallery)
        .join(&file_name);
    tracing::debug!("Trying to get path:  {:?}", path);

    // `File` implements `AsyncRead`
    let file = match tokio::fs::File::open(path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    let body = StreamBody::new(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str("text/toml; charset=utf-8").unwrap(), // This is wrong, add correct image here
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{}\"", file_name)).unwrap(),
    );

    // let headers = Headers([
    //     (header::CONTENT_TYPE, "text/toml; charset=utf-8"),
    //     (
    //         header::CONTENT_DISPOSITION,
    //         "attachment; filename=\"Cargo.toml\"",
    //     ),
    // ]);

    Ok((headers, body))
}
