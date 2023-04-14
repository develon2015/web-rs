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
                Err((StatusCode::BAD_REQUEST, "POST error, please check your payload"))
            }
        }
        _ => Err((StatusCode::METHOD_NOT_ALLOWED, "Only allow GET and POST requests"))
    }
}

pub fn router() -> axum::Router<Config> {
    axum::Router::new()
        .route("/config", axum::routing::any(config))
}