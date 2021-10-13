mod auth;
mod error;
mod file;
mod gallery;
mod login;
mod models;
mod recipies;
mod todos;

use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use warp::{http::Method, Filter};

use todos::filters;
// use gallery::filters;

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
    dotenv().ok();

    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        // env::set_var("RUST_LOG", "todos=info");
        // env::set_var("RUST_LOG", "images=info");
        // env::set_var("RUST_LOG", "recipies=info");
        env::set_var("RUST_LOG", "info");
    }

    let db_url = env::var("DATABASE_URL").expect("Failed to find 'DATABASE_URL'");

    pretty_env_logger::init();

    let download_route = warp::path("images").and(warp::fs::dir("./images/"));

    // Postgres
    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to pool");

    let api = download_route
        // .with(warp::log("images"))
        .or(filters::todos(pool.clone()).with(warp::log("todos")))
        .or(recipies::filters::recipies(pool.clone()).with(warp::log("recipies")))
        .or(gallery::filters::filter(pool.clone()).with(warp::log("gallery")))
        .or(file::filters::filter(pool.clone()).with(warp::log("file")))
        .or(login::filters::login_routes(pool.clone()).with(warp::log("login")))
        .recover(error::handle_rejection);

    let cors = warp::cors()
        // .allow_origin("*")
        .allow_origin("http://localhost:8080/")
        .allow_origin("http://localhost:3030/")
        .allow_credentials(true)
        .allow_any_origin()
        .allow_headers(vec![
            "origin",
            "date",
            "content-type",
            "content-length",
            "access-control-allow-origin",
        ])
        .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    // View access logs by setting `RUST_LOG=todos`.
    let routes = api.with(cors);
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
