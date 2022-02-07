use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct TestAlive {
    message: String,
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(TestAlive {
        message: "I'm Alive!".to_string(),
    })
}
