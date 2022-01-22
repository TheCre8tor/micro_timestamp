use std::io;
use actix_web::{web::{get, Json}, App, HttpRequest, HttpServer, Responder};
use serde::{Serialize};


// Routes -->
async fn greet(req: HttpRequest) -> impl Responder {
    println!("Request: {:?}", req.match_info());

    let name = req.match_info().get("uri").unwrap_or("World");
    format!("Hello {}!", name)
}

async fn current_temperature() -> impl Responder {
    Json( Measurement { temperature: 42.3, likes: true } )
}

#[derive(Serialize)]
struct Measurement {
    temperature: f32,
    likes: bool
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", get().to(greet))
            // .route("/{uri}", get().to(greet))
            .route("/measure", get().to(current_temperature))
    })
        .bind(("127.0.0.1", 7070))?
        .run()
        .await
}
