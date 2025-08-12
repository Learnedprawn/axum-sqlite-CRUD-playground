use std::ops::Sub;
use std::str::FromStr;
use std::sync::Arc;

// use anyhow::Ok;
use axum::extract::State;
use axum::routing::{patch, post};
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};

use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{ConnectOptions, SqlitePool};

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

#[derive(Serialize)]
struct Message {
    message: String,
}

#[derive(Deserialize)]
pub struct Submission {
    message: String,
}

enum ApiResponse {
    Ok,
    Created,
    JsonData(Vec<Message>),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Ok => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}

enum ApiError {
    BadRequest,
    Forbidden,
    Unauthorised,
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        // match self {
        //     self::Inter => ().into_response(),
        // }
        match self {
            ApiError::BadRequest => (StatusCode::BAD_REQUEST).into_response(),
            ApiError::Forbidden => (StatusCode::FORBIDDEN).into_response(),
            ApiError::Unauthorised => (StatusCode::UNAUTHORIZED).into_response(),
            ApiError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        }
    }
}

#[tokio::main]
// use std::error::Error;
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
async fn main() -> anyhow::Result<()> {
    let opts = SqliteConnectOptions::from_str("sqlite::memory:")?;
    let pool = SqlitePool::connect_with(opts).await?;

    let state = Arc::new(AppState { pool });

    let app = Router::new()
        .route("/", get(root))
        .route("/tasks", get(get_tasks).post(create_tasks))
        .route("/tasks/{task_id}", patch(update_tasks).delete(delete_tasks))
        .route("/table", get(create_table))
        .with_state(state);
    // .route("/create", post(create_table("test".to_string(), &pool)));
    // create_table(&pool, String::from_str("test"))
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = app.fallback(handler_404);
    axum::serve(listener, app).await.unwrap();
    println!("hello");
    Ok(())
}

// fn init_router(state: Arc<AppState>) -> Router {
//     Router::new().route("/", get(root)).with_state(state)
// }
async fn root() -> &'static str {
    "Hello World Vinesh"
}

#[derive(Serialize)]
struct TaskRow {
    task_id: u64,
    name: String,
    priority: Option<i64>,
}

async fn get_tasks(State(state): State<Arc<AppState>>) -> Result<ApiResponse, ApiError> {
    sqlx::query_as!(TaskRow, "Select * From task")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e}),
            )
        });
    Ok(ApiResponse::Ok)
}
async fn create_tasks(State(state): State<Arc<AppState>>) -> Result<ApiResponse, ApiError> {
    Ok(ApiResponse::Ok)
}

async fn update_tasks(State(state): State<Arc<AppState>>) -> Result<ApiResponse, ApiError> {
    Ok(ApiResponse::Ok)
}

async fn delete_tasks(State(state): State<Arc<AppState>>) -> Result<ApiResponse, ApiError> {
    Ok(ApiResponse::Ok)
}

#[axum::debug_handler]
async fn create_table(State(state): State<Arc<AppState>>) -> Result<ApiResponse, ApiError> {
    let pool = &state.pool;
    let connection = pool.acquire().await.unwrap();
    Ok(ApiResponse::Ok)
}

async fn json_handler(Json(json): Json<Submission>) -> Result<ApiResponse, ApiError> {
    println!("{}", json.message);
    Ok(ApiResponse::Ok)
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Lol this was so easy?")
}
