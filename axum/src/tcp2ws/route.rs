async fn exit() {
    tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        std::process::exit(0);
    });
}

pub fn router() -> axum::Router {
    axum::Router::new()
        .nest_service("/", tower_http::services::ServeDir::new("assets"))
        .route("/exit", axum::routing::get(exit))
}