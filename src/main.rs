use std::str::FromStr;
use std::sync::Arc;

use anyhow::Ok;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};

use serde::Serialize;
use serde_json::{Value, json};
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

enum ApiResponse {
    Ok,
    Created,
    JsonData(Vec<Message>),
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

#[axum::debug_handler]
async fn create_table(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let pool = &state.pool;
    let connection = pool.acquire().await.unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Lol this was so easy?")
}
