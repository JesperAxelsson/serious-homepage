// mod auth;
// mod error;
// mod file;
mod gallery;
mod login;
mod models;
mod recipies;
mod todos;

use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

use async_session::{MemoryStore, SessionStore as _};
use axum::{
    async_trait,
    extract::{rejection::TypedHeaderRejectionReason, FromRequest, RequestParts},
    headers::{self, Cookie},
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router, TypedHeader,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber::{filter, layer::SubscriberExt, reload, util::SubscriberInitExt};

const AXUM_SESSION_COOKIE_NAME: &str = "serious_session";

// TODO: Use salt and only store hashed passwords!

/// Provides a RESTful web server managing some Todos.
///
/// API will be:
///
/// - `GET /todos`: return a JSON list of Todos.
/// - `POST /todos`: create a new Todo.
/// - `PUT /todos/:id`: update a specific Todo.
/// - `DELETE /todos/:id`: delete a specific Todo.
///
/// - `GET /recipe`: return a JSON list of Todos.
/// - `POST /recipe`: create a new Todo.
/// - `PUT /recipe/:id`: update a specific Todo.
/// - `DELETE /recipe/:id`: delete a specific Todo.
///
#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    let filter = filter::LevelFilter::DEBUG;
    let (filter, _reload_handle) = reload::Layer::new(filter);

    tracing_subscriber::registry()
        .with(filter)
        // .with(tracing_subscriber::EnvFilter::new(
        //     std::env::var("RUST_LOG").unwrap_or_else(|_| "example_sessions=debug".into()),
        // ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();

    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        // env::set_var("RUST_LOG", "todos=info");
        // env::set_var("RUST_LOG", "images=info");
        // env::set_var("RUST_LOG", "recipies=info");
        env::set_var("RUST_LOG", "info");
    }

    // `MemoryStore` just used as an example. Don't use this in production.
    let store = MemoryStore::new();

    let db_url = env::var("DATABASE_URL").expect("Failed to find 'DATABASE_URL'");
    // Postgres
    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to pool");

    // pretty_env_logger::init();

    let app = Router::new()
        // .route("/", get(root))
        .route("/login", post(login::login))
        .route("/logout", post(login::logout))
        .route("/isin", get(login::protected))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        // Todo
        .route("/todo", get(todos::list_todos).post(todos::create_todo))
        .route(
            "/todo/:id",
            get(todos::get_todo)
                .put(todos::update_todo)
                .delete(todos::delete_todo),
        )
        // Recipe
        .route(
            "/recipe",
            get(recipies::list_recipies).post(recipies::create_recipe),
        )
        .route(
            "/recipe/:id",
            get(recipies::get_recipe)
                .put(recipies::update_recipe)
                .delete(recipies::delete_recipe),
        )
        // Gallery
        .route(
            "/gallery",
            get(gallery::list_album).post(gallery::create_album),
        )
        .route(
            "/gallery/:id",
            get(gallery::get_album)
                .put(gallery::update_album)
                .delete(gallery::delete_album),
        )
        .layer(Extension(pool))
        .layer(Extension(store));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    // let download_route = warp::path("images").and(warp::fs::dir("./images/"));

    // let cors = warp::cors()
    //     // .allow_origin("*")
    //     .allow_origin("http://localhost:8080/")
    //     .allow_origin("http://localhost:3030/")
    //     .allow_credentials(true)
    //     .allow_any_origin()
    //     .allow_headers(vec![
    //         "origin",
    //         "date",
    //         "content-type",
    //         "content-length",
    //         "access-control-allow-origin",
    //     ])
    //     .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE]);
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
pub struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<B> FromRequest<B> for DatabaseConnection
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(pool) = Extension::<PgPool>::from_request(req)
            .await
            .map_err(internal_error)?;

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub enum SessionIdFromSession {
    FoundUserId(SessionId),
    NotFound,
}

#[async_trait]
impl<B> FromRequest<B> for SessionIdFromSession
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        tracing::debug!("From req cookies!");

        let Extension(store) = Extension::<MemoryStore>::from_request(req)
            .await
            .expect("`MemoryStore` extension missing");

        // let cookie = Option::<TypedHeader<Cookie>>::from_request(req)
        //     .await
        //     .unwrap();

        // let session_cookie = cookie
        //     .as_ref()
        //     .and_then(|cookie| cookie.get(AXUM_SESSION_COOKIE_NAME));

        let cookies = TypedHeader::<headers::Cookie>::from_request(req)
            .await
            .map_err(|e| match *e.name() {
                header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => {
                        (StatusCode::UNAUTHORIZED, "Cookie header missing")
                    }
                    _ => panic!("unexpected error getting Cookie header(s): {}", e),
                },
                _ => panic!("unexpected error getting cookies: {}", e),
            })?;

        let session_cookie = cookies.get(AXUM_SESSION_COOKIE_NAME).ok_or((
            StatusCode::UNAUTHORIZED,
            "Could not get axum cookie from cookie",
        ))?;

        tracing::debug!("Session cookie: {:?}", session_cookie);

        if let Some(session) = store.load_session(session_cookie.to_owned()).await.unwrap() {
            if let Some(user_id) = session.get::<SessionId>("user_id") {
                tracing::debug!(
                    "UserIdFromSession: session decoded success, user_id={:?}",
                    user_id
                );
                Ok(Self::FoundUserId(user_id))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "No `user_id` found in session",
                ))
            }
        } else {
            Err((StatusCode::UNAUTHORIZED, "Failed to find active cookie"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SessionId(Uuid);

impl SessionId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
