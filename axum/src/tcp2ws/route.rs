use super::api;
use super::config::Config;

async fn exit() {
    tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        std::process::exit(0);
    });
}

pub fn router(config: Config) -> axum::Router {
    axum::Router::new()
        .nest_service("/", tower_http::services::ServeDir::new("assets"))
        .route("/exit", axum::routing::get(exit))
        .nest("/api", api::router())
        .with_state(config)
}