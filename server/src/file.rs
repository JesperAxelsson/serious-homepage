pub mod filters {
    use super::handlers;
    use sqlx::PgPool;
    use warp::Filter;

    pub fn filter(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        file_upload(pool.clone())
        // get_files(pool.clone())
        //     .or(file_upload(pool.clone()))
    }

    /// GET /file/<id>
    // pub fn get_files(
    //     pool: PgPool,
    // ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    //     // warp::path!("file" / i64)
    //     warp::path!("file")
    //         // .and(warp::get())
    //         // .and(with_pg(pool))
    //         .and(warp::fs::dir("./files/"))
    //         // .and_then(handlers::get_file)
    //         // .recover(handle_rejection)
    // }

    /// POST /file with JSON body
    pub fn file_upload(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("file")
            .and(warp::post())
            .and(warp::multipart::form().max_length(5_000_000))
            // .and(warp::fs::dir("./files/"))
            .and_then(handlers::create_file)
    }

    // fn with_pg(
    //     pool: PgPool,
    // ) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
    //     warp::any().map(move || pool.clone())
    // }

    // fn json_file() -> impl Filter<Extract = (Album,), Error = warp::Rejection> + Clone {
    //     // When accepting a body, we want a JSON body
    //     // (and to reject huge payloads)...
    //     warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    // }

    // async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    //     let (code, message) = if err.is_not_found() {
    //         (StatusCode::NOT_FOUND, "Not Found".to_string())
    //     } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
    //         (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    //     } else {
    //         eprintln!("unhandled error: {:?}", err);
    //         (
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             "Internal Server Error".to_string(),
    //         )
    //     };

    //     Ok(warp::reply::with_status(message, code))
    // }
}

/// These are our API handlers, the ends of each filter chain.
/// Notice how thanks to using `Filter::and`, we can define a function
/// with the exact arguments we'd expect from each filter in the chain.
/// No tuples are needed, it's auto flattened for the functions.
mod handlers {
    use bytes::BufMut;
    use futures::TryStreamExt;
    use sqlx::PgPool;
    use std::convert::Infallible;
    use warp::{
        http::StatusCode,
        multipart::{FormData, Part},
    };

    pub async fn get_file(id: i64, pool: PgPool) -> Result<impl warp::Reply, Infallible> {
        // let files = sqlx::query!(
        //     r#"select * from file i
        //     inner join file2file ai on i.id = ai.file_id
        //     where ai.file_id = $1"#,
        //     id
        // )
        // .fetch_all(&pool)
        // .await
        // .expect("Failed to execute list_gallery query")
        // .into_iter()
        // .map(|row| Image {
        //     id: row.id,
        //     title: row.title,
        //     description: row.description,
        //     file_url: row.file_url,
        //     preview_url: String::from(""),
        // })
        // .collect::<Vec<_>>();

        let files: Vec<String> = Vec::new();

        Ok(warp::reply::json(&files))
    }

    pub async fn create_file(
        form: FormData, // new_file: Album,
                        // pool: PgPool
    ) -> Result<impl warp::Reply, Infallible> {
        println!("Createfile!");

        let parts: Vec<Part> = form.try_collect().await.ok().expect("Failed to parse file");

        for p in parts {
            println!("Part: {:?}", p);
            if p.name() == "file" {
                let content_type = p.content_type();
                let file_ending;
                match content_type {
                    Some(file_type) => match file_type {
                        "application/pdf" => file_ending = "pdf",
                        "image/png" => file_ending = "png",
                        "image/jpg" => file_ending = "jpg",
                        "image/jpeg" => file_ending = "jpg",
                        v => {
                            eprintln!("invalid file type found: {}", v);
                            return Ok(StatusCode::BAD_REQUEST);
                        }
                    },
                    None => {
                        eprintln!("file type could not be determined");
                        return Ok(StatusCode::BAD_REQUEST);
                    }
                }

                let file_name: String = p.filename().unwrap().to_string();
                println!("Got filename: {}", file_name);

                let value = p
                    .stream()
                    .try_fold(Vec::new(), |mut vec, data| {
                        vec.put(data);
                        async move { Ok(vec) }
                    })
                    .await
                    .ok()
                    .unwrap();

                println!("Start writing file");
                tokio::fs::write(format!("./images/{}", file_name), &value)
                    .await
                    .unwrap();
                println!("File writing done");

                println!("Got fileending: {}", file_ending);
            }
        }

        // log::debug!("create_todo: {:?}", new_file);

        // let _rec = sqlx::query!(
        //     r#"
        //         INSERT INTO public.file
        //         (title, description, file_url)
        //         VALUES ($1, $2, $3)
        //         RETURNING id
        //     "#,
        //     new_file.title,
        //     new_file.description,
        //     new_file.file_url,
        // )
        // .fetch_one(&pool)
        // .await
        // .expect("Failed to insert new TODO");

        Ok(StatusCode::CREATED)
    }
}
