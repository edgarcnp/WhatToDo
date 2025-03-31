#![allow(unused)] // Remove later
use std::sync::Arc;

use axum::serve;
use axum_error::Result;
use sqlx::{pool, sqlite::SqlitePool};
use tokio::net::TcpListener;

mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv(); // Get environment variables
    let url = std::env::var("DATABASE_URL")?;

    // Database connection
    let pool = SqlitePool::connect(&url).await?;
    let shared_pool = Arc::new(pool);

    // Define routes
    let router = routes::app_routes(shared_pool);

    // Start the server
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on 0.0.0.0:3000");
    Ok(axum::serve(listener, router).await.unwrap())
}
