#[macro_use] extern crate rocket;
use rocket::State;
use rocket::fairing::AdHoc;
use rocket::serde::Deserialize;
mod startup;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct AppConfig {
    address: String,
    port: u16,
    version: String,
}

#[get("/version")]
async fn version(config: &State<AppConfig>) -> String {
    format!("server running at {}:{}, version: {}", config.address, config.port, config.version)
    // "version 1.0".to_string()
}

async fn go() -> Result<(), rocket::Error> {
    let app = startup::start_with_figment();
    // let app = startup::start_with_default_config();
    let app = app.mount("/", routes![version]);
    let app = app.attach(AdHoc::config::<AppConfig>());
    let _ = app.launch().await?;
    println!("Shutdown!");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), rocket::Error> {
    go().await
}