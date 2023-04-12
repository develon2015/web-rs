use axum::response::IntoResponse;

use super::config::Config;

async fn config(config: axum::extract::State<Config>) -> impl IntoResponse {
    let axum::extract::State(config) = config;
    axum::Json(config)
}

pub fn router() -> axum::Router<Config> {
    axum::Router::new()
        .route("/config", axum::routing::get(config))
}