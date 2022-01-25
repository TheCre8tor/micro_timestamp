use actix_web::web::Json;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Debug)]
pub struct Timestamp {
    unix: i64,
    utc: String,
}

#[derive(Deserialize)]
struct DateParam {
    date: String,
}

#[get("/api")]
async fn default_date() -> impl Responder {
    let date: DateTime<Utc> = Utc::now();
    let seconds_to_milli = date.timestamp() * 1000;

    let response = Timestamp {unix: seconds_to_milli, utc: date.to_string()};

    Json(response)
}

#[get("/api/{date}")]
async fn convert_date(params: web::Path<DateParam>) -> impl Responder {
    let date: DateTime<Utc> = params.date.parse().unwrap_or(Utc::now());
    format!("{}", date)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(convert_date).service(default_date))
        .bind("127.0.0.1:7070")?
        .run()
        .await
}
