use crate::routes::{convert_date, default_date};
use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .default_service(web::get().to(default_date))
                .route("/{date}", web::get().to(convert_date)),
        )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
