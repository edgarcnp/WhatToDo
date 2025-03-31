use axum::{
    Form, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, pool, query_as};
use std::{i64, sync::Arc};
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
        .route("/create", get(create))
        .route("/update", get(update))
        .route("/delete/:id", get(delete))
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

async fn create(State(pool): State<DbState>, Form(todo): Form<Todo>) -> Result<String, StatusCode> {
    sqlx::query!(
        "INSERT INTO todos (description) VALUES (?)",
        todo.description
    )
    .execute(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // Convert SQLx errors to 500

    Ok("Task successfully added!".to_string())
}

async fn delete(State(pool): State<DbState>, Path(id): Path<i64>) -> Result<String, StatusCode> {
    sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(&*pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // Convert SQLx errors to 500

    Ok("Task successfully deleted !".to_string())
}

async fn update(State(pool): State<DbState>, Form(todo): Form<Todo>) -> Result<String, StatusCode> {
    sqlx::query!(
        "UPDATE todos SET description = ?, status = ? WHERE id = ?",
        todo.description,
        todo.status,
        todo.id,
    )
    .execute(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // Convert SQLx errors to 500

    Ok("Task successfully updated!".to_string())
}
