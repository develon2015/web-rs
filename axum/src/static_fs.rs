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
async fn entry() {
    let port = 8080;
    #[cfg(windows)]
    unsafe {
        WinExec(format!("explorer http://127.0.0.1:{}\0", port).as_ptr(), 0);
        FreeConsole();
        AllocConsole();
    }
    let server = axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(app().into_make_service_with_connect_info::<std::net::SocketAddr>());
    println!("server run on: {}", server.local_addr());
    server.await.unwrap();
}

#[cfg(windows)]
fn main() {
    let exe = std::env::current_exe().unwrap();
    match std::env::var("detatch") {
        Ok(v) if v == "1" => {
            let mut a = std::process::Command::new(exe);
            a.arg("detatch");
            a.env("detatch", "1");
            a.spawn().unwrap();
            return;
        },
        _ => ()
    }
    entry();
}