use actix_web::{get, HttpResponse, Responder};

#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Hello from Rust REST")
}