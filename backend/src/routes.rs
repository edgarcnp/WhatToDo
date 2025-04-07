use axum::{
    Form, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};
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
        .route("/delete/{id}", delete(delete_todo))
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Router;
    use axum::body::Body;
    use hyper::Request;
    use sqlx::SqlitePool;
    use tokio::sync::OnceCell;
    use tower::ServiceExt;

    static DB_POOL: OnceCell<DbState> = OnceCell::const_new();

    async fn setup() -> Router {
        let pool = SqlitePool::connect("sqlite::memory:?cache=shared")
            .await
            .unwrap();
        sqlx::query!("CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY, description TEXT, status BOOLEAN)")
            .execute(&pool)
            .await
            .unwrap();

        let shared_pool = Arc::new(pool);
        DB_POOL.set(shared_pool.clone()).ok();

        app_routes(shared_pool)
    }

    #[tokio::test]
    async fn test_list_todo() {
        let app = setup().await;
        let response = app
            .clone()
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_todo() {
        let app = setup().await;
        let request = Request::post("/create")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(Body::from("description=Test Task"))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_update_todo() {
        let app = setup().await;
        let create_request = Request::post("/create")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(Body::from("description=Test Task"))
            .unwrap();
        let _ = app.clone().oneshot(create_request).await.unwrap();

        let update_request = Request::put("/update")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(Body::from("id=1&description=Updated Task&status=true"))
            .unwrap();

        let response = app.clone().oneshot(update_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_delete_todo() {
        let app = setup().await;
        let create_request = Request::post("/create")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(Body::from("description=Test Task"))
            .unwrap();
        let _ = app.clone().oneshot(create_request).await.unwrap();

        let delete_request = Request::delete("/delete/1").body(Body::empty()).unwrap();
        let response = app.clone().oneshot(delete_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
