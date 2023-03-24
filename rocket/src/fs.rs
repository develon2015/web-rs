use rocket::fairing::AdHoc;
use rocket::fs::FileServer;
use rocket::fs::relative;

pub fn ad_hoc() -> AdHoc {
    AdHoc::on_ignite("准备点火", |rocket| async move {
        rocket.mount("/fs", FileServer::from(relative!("static")))
            .attach(AdHoc::on_request("onRequest", |req, _data| {
                Box::pin(async move {
                    if req.uri().to_string().starts_with("/fs/") {
                        println!("{} {} {}", req.method(), req.uri(), req.accept().unwrap().first().unwrap().to_string());
                    }
                })
            }))
    })
}