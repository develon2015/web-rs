use std::net::IpAddr;
use rocket::fairing::AdHoc;
use rocket::response::content;
use rocket::http::uri::{Origin, Host};
use rocket::http::{Accept, Status, Header, hyper::header};
use rocket::Request;
use rocket::Response;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::outcome::Outcome::*;
use rocket::response::Responder;

/// 仅少数实现FromRequest trait的结构可用：
/// https://api.rocket.rs/v0.5-rc/rocket/request/trait.FromRequest.html#implementors
#[get("/")]
fn handler(uri: &Origin, host: &Host, addr: IpAddr, my_accept: MyAccept) -> content::RawHtml<String> {
    content::RawHtml(format!("{addr} visit {host}{uri}, my_accept: {:#?}", my_accept.value))
}

#[get("/resp")]
fn response() -> ResponderExample {
    ResponderExample
}

#[derive(Debug)]
struct MyAccept<'a> {
    value: &'a Accept,
}

/// 为自定义类型实现FromRequest trait
#[rocket::async_trait]
impl<'r> FromRequest<'r> for MyAccept<'r> {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.accept() {
            Some(accept) => Success(MyAccept { value: accept }),
            // None => Forward(()) // 重定向说是
            None => Failure((Status::BadRequest, "发送什么事了？".to_string()))
        }
    }

}

/// 参考：
/// https://github.com/SergioBenitez/Rocket/pull/2466
struct ResponderExample;

impl<'r, 'o: 'r> Responder<'r, 'o> for ResponderExample {
    fn respond_to(self, _req: &'r Request<'_>) -> rocket::response::Result<'o> {
        let body = "It worked!";
        Ok(Response::build()
            .status(Status::UnprocessableEntity)
            .header(Header::new(header::CONNECTION.as_str(), "keep-alive"))
            .raw_header("Server", "value")
            .sized_body(body.len(), std::io::Cursor::new(body))
            .finalize()
        )
    }
}

pub fn ad_hoc() -> AdHoc {
    AdHoc::on_ignite("my_type", |rocket| async move {
        rocket.mount("/my_type", routes![handler, response])
    })
}