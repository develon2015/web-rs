use axum::*;
use extract::*;
use response::IntoResponse;
use serde::*;

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
}

async fn ws_handler(
    upgrade: WebSocketUpgrade,
    ConnectInfo(ip): ConnectInfo<std::net::SocketAddr>, // 此提取器要求您使用 Router::into_make_service_with_connect_info 来运行您的应用程序
    method: axum::http::Method, uri: axum::http::Uri, version: axum::http::Version,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    println!("----------------------------------------------------------------------------------------------------");
    println!("client: {}", ip);
    println!("{} {} {:?}", method, uri, version);
    println!("Host: {:?}", headers.get(http::header::HOST));
    println!("headers: {:?}", headers);
    let mut resp = upgrade.on_upgrade(|mut socket| async move {
        println!("{:?}", socket.protocol());
        // socket.send(ws::Message::Ping(vec![])).await.unwrap();move async
        socket.send(ws::Message::Text(format!("Welcome"))).await.unwrap();
        let mut interval = tokio::time::interval(core::time::Duration::from_millis(1000));
        loop {
            tokio::select! {
                msg = socket.recv() => {
                    let msg = msg.unwrap().unwrap();
                    match msg {
                        ws::Message::Text(msg) => {
                            println!("recv: {msg}");
                            socket.send(ws::Message::Text(msg)).await.unwrap();
                        }
                        _ => {
                            println!("recv: {:?}", msg);
                        }
                    }
                }
                _ = interval.tick() => {
                    socket.send(ws::Message::Text(format!("Waiting..."))).await.unwrap();
                }
                else => {
                    println!("All error...");
                    break;
                }
            };
        }
    });
    println!("------------------------------------------");
    resp.headers_mut().append(http::header::SERVER, http::HeaderValue::from_static("Axum"));
    println!("{:?} {}", resp.version(), resp.status());
    println!("{:?}", resp.headers());
    resp
}

fn app() -> axum::Router {
    axum::Router::new()
        .route("/ws", axum::routing::get(ws_handler))
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let server = axum::Server::bind(&"[::]:8080".parse().unwrap())
        .serve(app().into_make_service_with_connect_info::<std::net::SocketAddr>());
    println!("server run on: {}", server.local_addr());
    server.await.unwrap();
}