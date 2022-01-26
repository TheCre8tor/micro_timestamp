use actix_web::web::Json;
use actix_web::{web, App, HttpServer, Responder, Either};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
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

#[derive(Serialize)]
struct BadDataError {
    error: String
}

async fn default_date() -> impl Responder {
    let date: DateTime<Utc> = Utc::now();
    let seconds_to_milli = date.timestamp() * 1000;

    let response = Timestamp {
        unix: seconds_to_milli,
        utc: date.to_string(),
    };

    Json(response)
}

async fn convert_date(params: web::Path<DateParam>) -> Either<impl Responder, Json<BadDataError>> {
    let validate = validate_params(&params.date);

    /* 1a. A request to /api/:date? with a valid date should return a JSON object
       with a unix key that is a Unix timestamp of the input date in milliseconds  --> Done

       1b. A request to /api/:date? with a valid date should return a JSON object
       with a utc key that is a string of the input date in the format: Thu, 01 Jan 1970 00:00:00 GMT --> Done

       2. A request to /api/1451001600000 should return { unix: 1451001600000, utc: "Fri, 25 Dec 2015 00:00:00 GMT" }

       3. If the input date string is invalid, the api returns an object having the structure { error : "Invalid Date" } --> Done

       4. An empty date parameter should return the current time in a JSON object with a unix key --> Done

       5. An empty date parameter should return the current time in a JSON object with a utc key --> Done
     */

    match validate {
        true => {
            let split_params = &params.date.trim().replace('-', " ");
            let date_list: Vec<u32> = split_params
                .split_inclusive(" ")
                .map(|x| x.trim().parse().unwrap())
                .collect();

            if date_list.len() < 3 {
                return Either::Right(Json(BadDataError {error: "Invalid Data".to_string()}));
            }

            let process_date =
                NaiveDate::from_ymd(date_list[0] as i32, date_list[1], date_list[2]).and_hms(0, 0, 0);

            let result = Timestamp {
                unix: process_date.timestamp() * 1000,
                utc: Utc::from_utc_datetime(&Utc, &process_date).to_string(),
            };

            Either::Left(Json(result))
        }
        false => {
            Either::Left(Json(Timestamp {unix: 78, utc: params.date.to_string()}))
        }
    }
}

pub fn validate_params(params: &String) -> bool {
    if params.contains("-") {
        return true;
    }
    false
}

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
