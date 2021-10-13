pub mod filters {
    use super::handlers;
    use crate::{
        auth::{with_auth, Role},
        models::{CreateRecipe, Recipe},
    };
    use sqlx::PgPool;
    use warp::Filter;

    // pub fn logout(
    //     pool: PgPool,
    // ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    //     warp::path!("logout")
    //         .and(warp::post())
    //         .and(with_auth(Role::Admin))
    //         .and(with_pg(pool))
    //         .and_then(handlers::logout)
    // }

    /// The 4 recipes filters combined.
    pub fn recipies(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        recipe_get(pool.clone())
            .or(recipe_list(pool.clone()))
            .or(recipe_create(pool.clone()))
            .or(recipe_update(pool.clone()))
            .or(recipe_delete(pool.clone()))
    }

    pub fn recipe_get(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("recipe" / i64)
            .and(warp::get())
            // .and(with_auth(Role::Admin))
            .and(with_pg(pool))
            .and_then(handlers::get_recipe)
    }

    /// GET /recipe
    pub fn recipe_list(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("recipe")
            .and(warp::get())
            .and(with_pg(pool))
            .and_then(handlers::list_recipies)
    }

    /// POST /recipe with JSON body
    pub fn recipe_create(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("recipe")
            .and(warp::post())
            .and(json_create_body())
            .and(with_pg(pool))
            .and_then(handlers::create_recipe)
    }

    /// PUT /recipe/:id with JSON body
    pub fn recipe_update(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("recipe")
            .and(warp::put())
            .and(json_body())
            .and(with_pg(pool))
            .and_then(handlers::update_recipe)
    }

    /// DELETE /recipe/:id
    pub fn recipe_delete(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        // We'll make one of our endpoints admin-only to show how authentication filters are used
        // let admin_only = warp::header::exact("authorization", "Bearer admin");

        warp::path!("recipe" / i64)
            // It is important to put the auth check _after_ the path filters.
            // If we put the auth check before, the request `PUT /recipies/invalid-string`
            // would try this filter and reject because the authorization header doesn't match,
            // rather because the param is wrong for that other path.
            // .and(admin_only)
            .and(warp::delete())
            .and(with_pg(pool))
            .and_then(handlers::delete_recipe)
    }

    fn with_pg(
        pool: PgPool,
    ) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || pool.clone())
    }

    fn json_body() -> impl Filter<Extract = (Recipe,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    fn json_create_body() -> impl Filter<Extract = (CreateRecipe,), Error = warp::Rejection> + Clone
    {
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
    use crate::models::{CreateRecipe, Recipe};
    use sqlx::PgPool;
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn get_recipe(id: i64, pool: PgPool) -> Result<impl warp::Reply, Infallible> {
        let rec = sqlx::query!(
            "select id, title, description, content from recipe where id = $1",
            id
        )
        .fetch_one(&pool)
        .await;

        if let Ok(rec) = rec {
            let recipe = Recipe {
                id: rec.id,
                title: rec.title,
                description: rec.description,
                content: rec.content,
            };

            Ok(warp::reply::with_status(
                warp::reply::json(&recipe),
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

    pub async fn list_recipies(pool: PgPool) -> Result<impl warp::Reply, Infallible> {
        let recipies =
            sqlx::query!("select id, title, description, content from recipe order by id")
                .fetch_all(&pool)
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

        Ok(warp::reply::json(&recipies))
    }

    pub async fn create_recipe(
        create: CreateRecipe,
        pool: PgPool,
    ) -> Result<impl warp::Reply, Infallible> {
        log::debug!("create_recipe: {:?}", create);

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
        .fetch_one(&pool)
        .await
        .expect("Failed to insert new recipe");

        Ok(StatusCode::CREATED)
    }

    pub async fn update_recipe(
        // id: i64,
        update: Recipe,
        pool: PgPool,
    ) -> Result<impl warp::Reply, Infallible> {
        log::debug!("update_recipe: id={}, recipe={:?}", update.id, update);

        let rec = sqlx::query!(
            r#"
                UPDATE recipe 
                SET title = $2, description = $3, content = $4
                WHERE id = $1
            "#,
            update.id,
            update.title,
            update.description,
            update.content,
        )
        .execute(&pool)
        .await
        .expect("Failed to update recipe");

        log::debug!("    -> recipe id not found!");

        // If the for loop didn't return OK, then the ID doesn't exist...
        if rec.rows_affected() == 1 {
            Ok(StatusCode::OK)
        } else {
            Ok(StatusCode::NOT_FOUND)
        }
    }

    pub async fn delete_recipe(id: i64, pool: PgPool) -> Result<impl warp::Reply, Infallible> {
        log::debug!("delete_recipe: id={}", id);

        let rec = sqlx::query!(
            r#"
                DELETE FROM recipe 
                WHERE id = $1
            "#,
            id
        )
        .execute(&pool)
        .await
        .expect("Failed to update recipe");

        if rec.rows_affected() == 1 {
            // respond with a `204 No Content`, which means successful,
            // yet no body expected...
            Ok(StatusCode::NO_CONTENT)
        } else {
            log::debug!("    -> recipe id not found!");
            Ok(StatusCode::NOT_FOUND)
        }
    }
}
