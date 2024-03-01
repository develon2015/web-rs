use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::{get, post}, Json};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Default, Serialize, Deserialize, Clone)]
struct Data {
    a: u8,
    b: String,
}

async fn test1(Json(data): Json<Data>) -> impl IntoResponse {
    Json(data)
}

async fn test2(State(mut app_state): State<AppState>) -> impl IntoResponse {
    app_state.data.a += 1; // No effect
    Json(app_state.data)
}

async fn state(State(state): State<Arc<Mutex<u8>>>, Json(mut data): Json<Data>) -> impl IntoResponse {
    let mut state = state.lock().await;
    *state = *state + 1;
    data.a = *state;
    Json(data)
}

#[derive(Default, Clone)]
struct AppState {
    data: Data,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let count = Arc::new(Mutex::new(0u8));
    let make_service = axum::Router::new()
        .route("/version", get(|| async { "alpha 1.0" }))
        .route("/data", get(|| async { Json(Data::default()) }))
        .route("/echo", post(test1))
        .route("/echo2", post(test2))
        .route("/state", post(state).with_state(count.clone()))
        .with_state(AppState::default())
    ;
    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(tcp_listener, make_service).await.unwrap();
}
