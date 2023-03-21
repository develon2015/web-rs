#[macro_use] extern crate rocket;
use rocket::State;
use rocket::fairing::AdHoc;
use rocket::fs::FileServer;
use rocket::fs::relative;
use rocket::serde::Deserialize;
mod startup;

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

fn fs() -> AdHoc {
    AdHoc::on_ignite("准备点火", |rocket| async move {
        rocket.mount("/fs", FileServer::from(relative!("static")))
            .attach(AdHoc::on_request("onRequest", |req, data| {
                Box::pin(async move {
                    println!("{} {} {}", req.method(), req.uri(), req.accept().unwrap().first().unwrap().to_string());
                })
            }))
    })
}

async fn go() -> Result<(), rocket::Error> {
    let app = startup::start_with_figment();
    // let app = startup::start_with_default_config();
    let app = app.mount("/", routes![index, version]);
    let app = app.attach(AdHoc::config::<AppConfig>());
    let app = app.attach(fs());
    let _ = app.launch().await?;
    println!("Shutdown!");
    Ok(())
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    go().await
}