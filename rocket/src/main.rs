#[macro_use] extern crate rocket;
use rocket::response::content;
use rocket::{State, Request};
use rocket::fairing::AdHoc;
use rocket::serde::Deserialize;
mod startup;
mod fs;
mod my_type;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct AppConfig {
    address: String,
    port: u16,
    version: String,
}

#[get("/")]
async fn index() -> String {
    format!("Hello")
}

#[get("/version")]
async fn version(config: &State<AppConfig>) -> String {
    format!("server running at {}:{}, version: {}", config.address, config.port, config.version)
    // "version 1.0".to_string()
}

#[catch(404)]
fn catch404(req: &Request) -> content::RawHtml<String> {
    content::RawHtml(format!("404 for {}", req.uri()))
}

async fn go() -> Result<(), rocket::Error> {
    let app = startup::start_with_figment();
    // let app = startup::start_with_default_config();
    let app = app.mount("/", routes![index, version]);
    let app = app.register("/", catchers![catch404]);
    let app = app.attach(AdHoc::config::<AppConfig>());
    let app = app.attach(fs::ad_hoc());
    let app = app.attach(my_type::ad_hoc());
    // let app = app.mount("/json", json::router()); // （反）序列化只支持Rust夜间构建版本
    let _ = app.launch().await?;
    println!("Shutdown!");
    Ok(())
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    go().await
}