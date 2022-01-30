use actix_web::{web, App, HttpServer};
use micro_timestamp::routes::{convert_date, default_date};
use std::io;
// use tracing_actix_web::TracingLogger;

#[tokio::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .default_service(web::get().to(default_date))
                .route("/{date}", web::get().to(convert_date)),
        )
    })
    .bind("127.0.0.1:7070")?
    .run()
    .await
}
