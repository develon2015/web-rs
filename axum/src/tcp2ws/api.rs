use axum::*;
use tokio::*;
use response::IntoResponse;
use http::StatusCode;
use http::Method;

use super::config::Config;

async fn config(method: Method, config: extract::State<Config>, payload: Option<Json<Config>>) -> impl IntoResponse {
    match method {
        Method::GET => {
            let extract::State(config) = config;
            Ok((StatusCode::OK, Json(config)))
        }
        Method::POST => {
            if let Some(Json(config)) = payload {
                fs::write(super::config::CONFIG, serde_json::to_string_pretty(&config).unwrap()).await.unwrap();
                Ok((StatusCode::OK, Json(config)))
            } else {
                Err((StatusCode::NOT_FOUND, "POST error"))
            }
        }
        _ => Err((StatusCode::NOT_FOUND, "Only allow GET and POST request"))
    }
}

pub fn router() -> axum::Router<Config> {
    axum::Router::new()
        .route("/config", axum::routing::any(config))
}