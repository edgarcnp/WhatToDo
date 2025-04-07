use std::sync::Arc;

use axum::serve;
use axum_error::Result;
use sqlx::{pool, sqlite::SqlitePool};
use tokio::net::TcpListener;

mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv(); // Load .env
    let url = std::env::var("DATABASE_URL")?;
    let address = std::env::var("SERVER_ADDRESS")?;

    // DB connection
    let pool = SqlitePool::connect(&url).await?;
    let shared_pool = Arc::new(pool);

    // Axum routes
    let router = routes::app_routes(shared_pool);

    // Bind to address
    let listener = TcpListener::bind(&address).await.unwrap();
    println!("Listening on: {}", &address);
    Ok(serve(listener, router).await.unwrap())
}
