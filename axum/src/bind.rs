use std::{net::SocketAddr, error::Error};

use axum::{
    routing::get,
    Router,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // build our application with a single route
    let app: Router = Router::new().route("/", get(|| async { "Hello, World!" }));
    // SocketAddr
    let addr: SocketAddr = format!("[::]:0").parse().unwrap();
    let server = match axum::Server::try_bind(&addr) {
        Ok(it) => it,
        Err(e) => {
            println!("服务运行失败! {}", e.source().unwrap().to_string());
            return;
        }
    };
    // TcpListener
    let listener = tokio::net::TcpListener::bind("[::]:0").await.unwrap();
    let server = axum::Server::from_tcp(listener.into_std().unwrap()).unwrap();
    // Builder -> Server
    let server = server.serve(app.into_make_service());
    println!("server run on: {}", server.local_addr());
    // Launch
    server.await.unwrap();
}