
pub mod filters {
    use super::handlers;
    use crate::models::{ListOptions, Todo};
    use sqlx::PgPool;
    use warp::Filter;

    /// The 4 TODOs filters combined.
    pub fn todos(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        todos_get(pool.clone())
            .or(todos_list(pool.clone()))
            .or(todos_create(pool.clone()))
            .or(todos_update(pool.clone()))
            .or(todos_delete(pool.clone()))
    }

    pub fn todos_get(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("todos" / i64)
            .and(warp::get())
            .and(with_pg(pool))
            .and_then(handlers::get_todo)
    }

    /// GET /todos?offset=3&limit=5
    pub fn todos_list(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("todos")
            .and(warp::get())
            .and(warp::query::<ListOptions>())
            .and(with_pg(pool))
            .and_then(handlers::list_todos)
    }

    /// POST /todos with JSON body
    pub fn todos_create(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("todos")
            .and(warp::post())
            .and(json_body())
            .and(with_pg(pool))
            .and_then(handlers::create_todo)
    }

    /// PUT /todos/:id with JSON body
    pub fn todos_update(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("todos" / i64)
            .and(warp::put())
            .and(json_body())
            .and(with_pg(pool))
            .and_then(handlers::update_todo)
    }

    /// DELETE /todos/:id
    pub fn todos_delete(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        // We'll make one of our endpoints admin-only to show how authentication filters are used
        let admin_only = warp::header::exact("authorization", "Bearer admin");

        warp::path!("todos" / i64)
            // It is important to put the auth check _after_ the path filters.
            // If we put the auth check before, the request `PUT /todos/invalid-string`
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

    fn json_body() -> impl Filter<Extract = (Todo,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

/// These are our API handlers, the ends of each filter chain.
/// Notice how thanks to using `Filter::and`, we can define a function
/// with the exact arguments we'd expect from each filter in the chain.
/// No tuples are needed, it's auto flattened for the functions.
pub mod handlers {
    use crate::models::{ListOptions, Todo};
    use sqlx::PgPool;
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn get_todo(id: i64, pool: PgPool) -> Result<impl warp::Reply, Infallible> {
        let rec = sqlx::query!("select id, text, completed from todo where id = $1", id)
            .fetch_one(&pool)
            .await;

        if let Ok(rec) = rec {
            let todo = Todo {
                id: rec.id,
                text: rec.text,
                completed: rec.completed,
            };

            Ok(warp::reply::with_status(
                warp::reply::json(&todo),
                StatusCode::OK,
            ))
        } else {
            let msg = "Entry not found".to_string();
            Ok(warp::reply::with_status(
                warp::reply::json(&msg),
                StatusCode::NOT_FOUND,
            ))
        }
    }

    pub async fn list_todos(
        opts: ListOptions,
        pool: PgPool,
    ) -> Result<impl warp::Reply, Infallible> {
        let todos = sqlx::query!(
            "select id, text, completed from todo order by id limit $1 offset $2",
            opts.limit.unwrap_or(std::i64::MAX),
            opts.offset.unwrap_or(0)
        )
        .fetch_all(&pool)
        .await
        .expect("Failed to execute list_todos query")
        .into_iter()
        .map(|row| Todo {
            id: row.id,
            text: row.text,
            completed: row.completed,
        })
        .collect::<Vec<_>>();

        Ok(warp::reply::json(&todos))
    }

    pub async fn create_todo(create: Todo, pool: PgPool) -> Result<impl warp::Reply, Infallible> {
        log::debug!("create_todo: {:?}", create);

        let _rec = sqlx::query!(
            r#"
                INSERT INTO todo (text)
                VALUES ($1)
                RETURNING id
            "#,
            create.text
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

// #[cfg(test)]
// mod tests {
//     use sqlx::PgPool;
//     use std::env;
//     use warp::http::StatusCode;
//     use warp::test::request;

//     use super::{
//         filters,
//         models::{self, Todo},
//     };

//     #[tokio::test]
//     async fn test_post() {
//         let db = models::blank_db();
//         let pool = PgPool::new(&env::var("DATABASE_URL").expect("Failed to find 'DATABASE_URL'"))
//             .await
//             .expect("Failed to connect to pool");
//         let api = filters::todos(db, pool);

//         let resp = request()
//             .method("POST")
//             .path("/todos")
//             .json(&Todo {
//                 id: 1,
//                 text: "test 1".into(),
//                 completed: false,
//             })
//             .reply(&api)
//             .await;

//         assert_eq!(resp.status(), StatusCode::CREATED);
//     }

//     #[tokio::test]
//     async fn test_post_conflict() {
//         let pool = PgPool::new(&env::var("DATABASE_URL").unwrap())
//             .await
//             .expect("Failed to connect to pool");

//         let db = models::blank_db();
//         db.lock().await.push(todo1());
//         let api = filters::todos(db, pool);

//         let resp = request()
//             .method("POST")
//             .path("/todos")
//             .json(&todo1())
//             .reply(&api)
//             .await;

//         assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
//     }

//     #[tokio::test]
//     async fn test_put_unknown() {
//         let pool = PgPool::new(&env::var("DATABASE_URL").unwrap())
//             .await
//             .expect("Failed to connect to pool");

//         let _ = pretty_env_logger::try_init();
//         let db = models::blank_db();
//         let api = filters::todos(db, pool);

//         let resp = request()
//             .method("PUT")
//             .path("/todos/1")
//             .header("authorization", "Bearer admin")
//             .json(&todo1())
//             .reply(&api)
//             .await;

//         assert_eq!(resp.status(), StatusCode::NOT_FOUND);
//     }

//     fn todo1() -> Todo {
//         Todo {
//             id: 1,
//             text: "test 1".into(),
//             completed: false,
//         }
//     }
// }

// Debug
// Connection Times (ms)
//               min  mean[+/-sd] median   max
// Connect:        0    1   4.7      0      24
// Processing:     1   31  39.4     27     334
// Waiting:        1   31  39.4     27     334
// Total:          1   32  40.1     28     334

// Percentage of the requests served within a certain time (ms)
//   50%     28
//   66%     32
//   75%     55
//   80%     60
//   90%     88
//   95%    116
//   98%    149
//   99%    176
//  100%    334 (longest request)
