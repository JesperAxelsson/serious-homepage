pub mod filters {
    use super::handlers;
    use crate::models::{Album, ListOptions, Todo};
    use sqlx::PgPool;
    use warp::Filter;

    pub fn filter(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        get(pool.clone())
            .or(album_list(pool.clone()))
            .or(gallery_create(pool.clone()))
            .or(gallery_update(pool.clone()))
            .or(gallery_delete(pool.clone()))
    }

    pub fn get(
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

    /// POST /gallery with JSON body
    pub fn gallery_create(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("gallery")
            .and(warp::post())
            .and(json_album())
            .and(with_pg(pool))
            .and_then(handlers::create_album)
    }

    /// PUT /gallery/:id with JSON body
    pub fn gallery_update(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("gallery" / i64)
            .and(warp::put())
            .and(json_body())
            .and(with_pg(pool))
            .and_then(handlers::update_todo)
    }

    /// DELETE /gallery/:id
    pub fn gallery_delete(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        // We'll make one of our endpoints admin-only to show how authentication filters are used
        let admin_only = warp::header::exact("authorization", "Bearer admin");

        warp::path!("gallery" / i64)
            // It is important to put the auth check _after_ the path filters.
            // If we put the auth check before, the request `PUT /gallery/invalid-string`
            // would try this filter and reject because the authorization header doesn't match,
            // rather because the param is wrong for that other path.
            .and(admin_only)
            .and(warp::delete())
            .and(with_pg(pool))
            .and_then(handlers::delete_todo)
    }

    fn with_pg(
        pool: PgPool,
    ) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || pool.clone())
    }

    // fn json_body2<T>() -> impl Filter<Extract = (Todo,), Error = warp::Rejection> + Clone {
    //     // When accepting a body, we want a JSON body
    //     // (and to reject huge payloads)...
    //     warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    // }

    fn json_body() -> impl Filter<Extract = (Todo,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
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
    use crate::models::{Album, Image, ListOptions, Todo};
    use sqlx::PgPool;
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn get_album(id: i64, pool: PgPool) -> Result<impl warp::Reply, Infallible> {
        let rec = sqlx::query!("select id, text, completed from todo where id = $1", id)
            .fetch_one(&pool)
            .await;

        let images = sqlx::query!(
            r#"select * from image i 
            inner join album2image ai on i.id = ai.image_id 
            where ai.album_id = 1"#,
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

    pub async fn update_todo(
        id: i64,
        update: Todo,
        pool: PgPool,
    ) -> Result<impl warp::Reply, Infallible> {
        log::debug!("update_todo: id={}, todo={:?}", id, update);

        let rec = sqlx::query!(
            r#"
                UPDATE todo 
                SET completed = $2, text = $3
                WHERE id = $1
            "#,
            update.id,
            update.completed,
            update.text
        )
        .execute(&pool)
        .await
        .expect("Failed to update TODO");

        log::debug!("    -> todo id not found!");

        // If the for loop didn't return OK, then the ID doesn't exist...
        if rec == 1 {
            Ok(StatusCode::OK)
        } else {
            Ok(StatusCode::NOT_FOUND)
        }
    }

    pub async fn delete_todo(id: i64, pool: PgPool) -> Result<impl warp::Reply, Infallible> {
        log::debug!("delete_todo: id={}", id);

        let rec = sqlx::query!(
            r#"
                DELETE FROM todo 
                WHERE id = $1
            "#,
            id
        )
        .execute(&pool)
        .await
        .expect("Failed to update TODO");

        if rec == 1 {
            // respond with a `204 No Content`, which means successful,
            // yet no body expected...
            Ok(StatusCode::NO_CONTENT)
        } else {
            log::debug!("    -> todo id not found!");
            Ok(StatusCode::NOT_FOUND)
        }
    }
}
