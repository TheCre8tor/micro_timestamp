use actix_web::web;
use actix_web::{web::Json, Either, Responder};
use chrono::{NaiveDate, TimeZone, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct Timestamp {
    pub unix: i64,
    pub utc: String,
}

#[derive(Deserialize)]
pub struct DateParam {
    date: String,
}

#[derive(Serialize)]
pub struct BadDataError {
    error: String,
}

pub async fn convert_date(
    params: web::Path<DateParam>,
) -> Either<impl Responder, Json<BadDataError>> {

    /* 1a. A request to /api/:date? with a valid date should return a JSON object
      with a unix key that is a Unix timestamp of the input date in milliseconds  --> Done

      1b. A request to /api/:date? with a valid date should return a JSON object
      with a utc key that is a string of the input date in the format: Thu, 01 Jan 1970 00:00:00 GMT --> Done

      2. A request to /api/1451001600000 should return { unix: 1451001600000, utc: "Fri, 25 Dec 2015 00:00:00 GMT" }

      3. If the input date string is invalid, the api returns an object having the structure { error : "Invalid Date" }

      4. An empty date parameter should return the current time in a JSON object with a unix key --> Done

      5. An empty date parameter should return the current time in a JSON object with a utc key --> Done
    */

    /* Test Cases -->
     Elite: /api/2021-04-09
     Elite: /api/1451001600000
     1. */

    let _split_params = &params.date.replace('-', " ");

    // let date_list: Vec<u32> = split_params
    //     .split_inclusive(" ")
    //     .map(|x| x.trim().parse().unwrap())
    //     .collect();
    //
    // if date_list.len() < 3 {
    //     return Either::Right(Json(BadDataError {
    //         error: "Invalid Data".to_string(),
    //     }));
    // }
    //
    // let process_date = NaiveDate::from_ymd(date_list[0] as i32, date_list[1], date_list[2])
    //     .and_hms(0, 0, 0);
    //
    // let result = Timestamp {
    //     unix: process_date.timestamp() * 1000,
    //     utc: Utc::from_utc_datetime(&Utc, &process_date).to_string(),
    // };

    // Either::Left(Json(result))
    Either::Left(Json(BadDataError {error: "".to_string()}))
}
