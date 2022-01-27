use crate::routes::convert_date::Timestamp;
use actix_web::{web::Json, Responder};
use chrono::{DateTime, Utc};

pub async fn default_date() -> impl Responder {
    let date: DateTime<Utc> = Utc::now();
    let seconds_to_milli = date.timestamp() * 1000;

    let response = Timestamp {
        unix: seconds_to_milli,
        utc: date.to_string(),
    };

    Json(response)
}
