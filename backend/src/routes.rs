use axum::{
    Form, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{SqlitePool, query_as};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

// Define shared state type
pub type DbState = Arc<SqlitePool>;

#[derive(Serialize, Deserialize)]
pub struct Todo {
    id: i64,
    description: String,
    status: bool,
}

#[derive(Deserialize)]
pub struct CreateTodo {
    description: String,
}

#[derive(Serialize)]
struct ResponseMessage {
    message: String,
}

pub fn app_routes(pool: DbState) -> Router {
    Router::new()
        .route("/", get(list_todo))
        .route("/create", post(create_todo))
        .route("/update", put(update_todo))
        .route("/delete/:id", delete(delete_todo))
        .layer(CorsLayer::very_permissive())
        .with_state(pool)
}

async fn list_todo(State(pool): State<DbState>) -> Result<Json<Vec<Todo>>, StatusCode> {
    let todos = query_as!(Todo, "SELECT id, description, status FROM todos")
        .fetch_all(&*pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todos))
}

async fn create_todo(
    State(pool): State<DbState>,
    Form(todo): Form<CreateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let id = sqlx::query!(
        "INSERT INTO todos (description) VALUES (?) RETURNING id",
        todo.description
    )
    .fetch_one(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .id;

    let new_todo = Todo {
        id,
        description: todo.description,
        status: false,
    };

    Ok(Json(new_todo))
}

async fn delete_todo(
    State(pool): State<DbState>,
    Path(id): Path<i64>,
) -> Result<Json<ResponseMessage>, StatusCode> {
    let result = sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(&*pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(ResponseMessage {
        message: "Task successfully deleted!".to_string(),
    }))
}

async fn update_todo(
    State(pool): State<DbState>,
    Form(todo): Form<Todo>,
) -> Result<Json<ResponseMessage>, StatusCode> {
    let result = sqlx::query!(
        "UPDATE todos SET description = ?, status = ? WHERE id = ?",
        todo.description,
        todo.status,
        todo.id,
    )
    .execute(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(ResponseMessage {
        message: "Task successfully updated!".to_string(),
    }))
}
