#[cfg(windows)]
extern "system" {
    fn FreeConsole();
    fn AllocConsole();
    fn WinExec(cmd: *const u8, cmdShow: u8);
}

fn app() -> axum::Router {
    axum::Router::new()
        .nest_service("/", tower_http::services::ServeDir::new("assets"))
        .route("/api", axum::routing::get(|| async {
            axum::response::Html(include_str!("main.rs"))
        }))
}

// #[tokio::main(flavor = "multi_thread", worker_threads = 5)]
#[tokio::main(flavor = "current_thread")]
async fn entry(socket: std::net::TcpListener) {

    let server = axum::Server::from_tcp(socket).unwrap()
        .serve(app().into_make_service_with_connect_info::<std::net::SocketAddr>());
    println!("server run on: {}", server.local_addr());
    server.await.unwrap();
}

#[cfg(windows)]
fn start() {
    let port = 8080;
    unsafe {
        WinExec(format!("explorer http://127.0.0.1:{}\0", port).as_ptr(), 0);
        FreeConsole();
        AllocConsole();
    }
    match std::net::TcpListener::bind(format!("0.0.0.0:{}", port)) {
        Ok(socket) => {
            entry(socket);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

#[cfg(windows)]
fn main() {
    match std::env::var("detatch") {
        Ok(v) if v == "1" => {
        }
        _ => {
            let exe = std::env::current_exe().unwrap();
            let mut process = std::process::Command::new(exe);
            process.arg("detatch");
            process.env("detatch", "1");
            process.spawn().unwrap();
            return;
        },
    }
    start();
}