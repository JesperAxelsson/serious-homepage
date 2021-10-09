pub mod filters {
    use super::handlers;
    use crate::models::Album;
    use sqlx::PgPool;
    use warp::Filter;

    pub fn filter(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        get_album_images(pool.clone())
            .or(album_list(pool.clone()))
            .or(album_create(pool.clone()))
    }

    /// GET /album/<id>
    pub fn get_album_images(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("album" / i64)
            .and(warp::get())
            .and(with_pg(pool))
            .and_then(handlers::get_album)
    }

    /// GET /album
    pub fn album_list(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("album")
            .and(warp::get())
            .and(with_pg(pool))
            .and_then(handlers::list_album)
    }

    /// POST /album with JSON body
    pub fn album_create(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("album")
            .and(warp::post())
            .and(json_album())
            .and(with_pg(pool))
            .and_then(handlers::create_album)
    }

    fn with_pg(
        pool: PgPool,
    ) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || pool.clone())
    }

    fn json_album() -> impl Filter<Extract = (Album,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

/// These are our API handlers, the ends of each filter chain.
/// Notice how thanks to using `Filter::and`, we can define a function
/// with the exact arguments we'd expect from each filter in the chain.
/// No tuples are needed, it's auto flattened for the functions.
mod handlers {
    use crate::models::{Album, Image};
    use sqlx::PgPool;
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn get_album(id: i64, pool: PgPool) -> Result<impl warp::Reply, Infallible> {
        let images = sqlx::query!(
            r#"select * from image i 
            inner join album2image ai on i.id = ai.image_id 
            where ai.album_id = $1"#,
            id
        )
        .fetch_all(&pool)
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

        Ok(warp::reply::json(&images))
    }

    pub async fn list_album(pool: PgPool) -> Result<impl warp::Reply, Infallible> {
        let gallery =
            sqlx::query!("select id, title, description, image_url from album order by id",)
                .fetch_all(&pool)
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

        Ok(warp::reply::json(&gallery))
    }

    pub async fn create_album(
        new_album: Album,
        pool: PgPool,
    ) -> Result<impl warp::Reply, Infallible> {
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
        .fetch_one(&pool)
        .await
        .expect("Failed to insert new TODO");

        Ok(StatusCode::CREATED)
    }
}
