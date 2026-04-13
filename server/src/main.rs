mod db;
mod models;
mod routes;

use axum::Router;
use std::env;
use dotenv::dotenv;
use crate::routes::user_routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .merge(user_routes());

    let port = env::var("PORT").unwrap_or("8080".to_string());

    println!("Server running on {}", port);

    let listener = tokio::net::TcpListener::bind(
        format!("0.0.0.0:{}", port)
    ).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}