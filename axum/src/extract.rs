use axum::Json;
use axum::extract::ConnectInfo;
use axum::response::IntoResponse;
use serde::*;

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
}

// https://docs.rs/axum/latest/axum/extract/index.html
async fn test(
    ConnectInfo(ip): ConnectInfo<std::net::SocketAddr>, // 此提取器要求您使用 Router::into_make_service_with_connect_info 来运行您的应用程序
    method: axum::http::Method, uri: axum::http::Uri, version: axum::http::Version,
    headers: axum::http::HeaderMap,
    query: Option<axum::extract::Query<User>>, // 可以通过Option或者Result包装使提取器成为可选的, 比如该可选的查询字符串序列化, Result还提供失败原因
    Json(payload): Json<User>, // 该提取器消耗body
    // req: axum::http::Request<axum::body::Body>, // Request提供整个请求的最大控制, 但是不能与其它消费body的提取器一起使用
) -> impl IntoResponse {
    println!("client: {}", ip);
    println!("{} {} {:?}", method, uri, version);
    println!("headers: {:?}", headers);
    println!("Host: {:?}", headers.get(axum::http::header::HOST));
    println!("query: {:?}", query);
    println!("payload: {:?}", payload);
    let user = serde_json::to_string(&payload).unwrap();
    println!("序列化: {:?}", user);
    let user: User = serde_json::from_str(&user).unwrap();
    println!("反序列化: {:?}", user);
    // println!("req: {:?}", req.uri());
    (axum::http::StatusCode::OK, Json(user)) // 元组形式重写响应码
}

async fn test_body(
    method: axum::http::Method,
    // 以字符串形式消费body, 此时String必须是最后一个提取器
    // 意味着您不能两次使用请求正文
    // 官方说法：axum通过要求最后一个提取器实现FromRequest和其他所有提取器实现FromRequestParts来执行这一要求
    // 理解为：
    // 如果消费body, 则提取器不会实现FromRequestParts
    // 实现了FromRequestParts的提取器也会实现FromRequest
    // 但是又说不能同时实现FromRequest和FromRequestParts（除非它是在包装另一个抽取器）？
    // 这就有点矛盾了
    // 应该说成：axum要求实现FromRequest的提取器必须是最后一个提取器, 其他提取器则必须实现FromRequestParts
    body: String,
) -> impl IntoResponse {
    println!("{method} body -> {}", body);
    Json(
        serde_json::json!({ "body": body }) as serde_json::Value
    )
}

fn app() -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::post(test))
        .route("/body", axum::routing::post(test_body))
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let server = axum::Server::bind(&"[::]:8080".parse().unwrap())
        .serve(app().into_make_service_with_connect_info::<std::net::SocketAddr>());
    println!("server run on: {}", server.local_addr());
    server.await.unwrap();
}