mod db;
mod models;
mod routes;

use axum::Router;
use std::env;
use dotenv::dotenv;
use tower_http::cors::{Any, CorsLayer};
use crate::{routes::user_routes, db::connect_db};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = connect_db().await;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .merge(user_routes(db))
        .layer(cors);

    let port = env::var("PORT").unwrap_or("8080".to_string());

    println!("Server running on {}", port);

    let listener = tokio::net::TcpListener::bind(
        format!("0.0.0.0:{}", port)
    ).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
