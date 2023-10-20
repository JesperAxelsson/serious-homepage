// mod auth;
// mod error;
mod controllers;
mod file;
mod models;
mod session;

use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use tower_http::cors::CorsLayer;

use async_session::MemoryStore;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderValue, Method, StatusCode},
    routing::{get, post},
    Extension, Router,
};

use std::net::SocketAddr;
use tracing_subscriber::{filter, layer::SubscriberExt, reload, util::SubscriberInitExt};

use crate::controllers::{gallery, login, recipies, todos};

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

    let filter = filter::LevelFilter::INFO;
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
        .route(
            "/gallery/:id/:file_name",
            get(gallery::get_image), // .put(gallery::update_album)
                                     // .delete(gallery::delete_album),
        )
        .route("/file/:file_name", get(file::get_file))
        // .route("/file", get_service(ServeDir::new("files")).handle_error(handle_error))
        .route("/image", post(file::upload_many_file))
        .route("/image/:file_name", post(file::save_request_body))
        .layer(Extension(pool))
        .layer(Extension(store));

    // Prepend /api
    let app = Router::new().nest("/api", app).layer(
        CorsLayer::new()
            // .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
            .allow_origin("http://localhost:8000".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::PUT, Method::POST, Method::DELETE]),
    );

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

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
pub struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(pool) = Extension::<PgPool>::from_request_parts(req, state)
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
