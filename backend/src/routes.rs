use axum::{
    Form, Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, pool, query_as};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

// Define shared state type
type DbState = Arc<SqlitePool>;

#[derive(Serialize, Deserialize)]
struct Todo {
    id: i64,
    description: String,
    status: bool,
}

pub fn app_routes(pool: DbState) -> Router {
    Router::new()
        .route("/", get(list))
        .route("")
        .layer(CorsLayer::very_permissive())
        .with_state(pool)
}

async fn list(State(pool): State<DbState>) -> Result<Json<Vec<Todo>>, StatusCode> {
    let todos = sqlx::query_as!(Todo, "SELECT id, description, status FROM todos")
        .fetch_all(&*pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // Convert SQLx errors to 500

    Ok(Json(todos))
}

async fn create(State(pool): State<DbState>, Form(todo): Form<Todo>) -> Result<String> {
    sqlx::query!(
        "INSERT INTO todos (description) VALUES (?)",
        todo.description
    )
    .execute(&*pool)
    .await?;

    Ok(format!("Succesfully added!"))
}
