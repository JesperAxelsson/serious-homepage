use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Clone)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub password: String,
    pub salt: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub mod filters {
    use crate::auth::{with_auth, Role};

    use super::{handlers, LoginRequest};
    use sqlx::PgPool;
    use warp::Filter;

    pub fn login_routes(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        login(pool.clone()).or(logout(pool.clone()))
    }

    pub fn login(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("login")
            // .and(reply)
            .and(json_login_request())
            .and(with_pg(pool))
            .and_then(handlers::login)
    }

    pub fn logout(
        pool: PgPool,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("logout" / i64)
            .and(warp::post())
            .and(with_auth(Role::Admin))
            .and(with_pg(pool))
            .and_then(handlers::logout)
    }

    fn with_pg(
        pool: PgPool,
    ) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || pool.clone())
    }

    fn json_login_request(
    ) -> impl Filter<Extract = (LoginRequest,), Error = warp::Rejection> + Clone {
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
    use sqlx::PgPool;
    use warp::{
        reject,
        reply::{self, Response},
    };

    use crate::error::Error;
    use crate::{
        auth::{self, Role},
        login::LoginResponse,
    };

    use super::{Account, LoginRequest};

    pub async fn login(
        credentials: LoginRequest,
        pool: PgPool,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let account = sqlx::query_as!(
            Account,
            "select * from account where name = $1",
            credentials.name
        )
        .fetch_one(&pool)
        .await;

        match account {
            Err(_) => {
                // let reply = Response::default();
                // Ok(warp::reply::with_status(reply, StatusCode::BAD_REQUEST).into_response())
                Err(reject::custom(Error::WrongCredentialsError))
            }
            Ok(user) => {
                if credentials.password == user.password {
                    // TODO: Base64 encode...
                    // let token = format!("Basic {}:{}", user.name, user.password);
                    let token = auth::create_jwt(&credentials.name, Role::Admin).unwrap();
                    Ok(reply::json(&LoginResponse { token }))

                    // let reply = Response::default();
                    // Ok(warp::reply::with_header(reply, AUTHORIZATION, auth).into_response())
                    // Ok(StatusCode::OK)
                } else {
                    // let reply = Response::default();
                    Err(reject::custom(Error::WrongCredentialsError))
                    // Ok(warp::reply::with_status(reply, StatusCode::UNAUTHORIZED).into_response())
                    // Ok(StatusCode::UNAUTHORIZED)
                }
            }
        }
    }

    pub async fn logout(
        id: i64,
        username: String,
        pool: PgPool,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let reply = Response::default();
        println!("Username! {} {} {}", username, id, pool.size());
        Ok(warp::reply::with_header(reply, "server", "warp"))
    }
}
