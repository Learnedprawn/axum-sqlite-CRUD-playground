use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_json));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = app.fallback(handler_404);
    axum::serve(listener, app).await.unwrap();
    println!("hello");
}

async fn root() -> &'static str {
    "Hello World Vinesh"
}
async fn get_json() -> Json<Value> {
    Json(json!({"data": 42}))
}
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Lol this was so easy?")
}
