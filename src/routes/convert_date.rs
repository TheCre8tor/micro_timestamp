use crate::domain::DateParamName;
use actix_web::{web, HttpResponse};
use chrono::{NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Serialize};


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

pub async fn convert_date(params: web::Path<DateParam>) -> HttpResponse {
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
    Elite: /api/2021-04-09 -- Done
    Elite: /api/1451001600000
    1. */

    let valid_date = match DateParamName::parse(&params.date) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().json(BadDataError { error }),
    };

    let process_date =
        NaiveDate::from_ymd(valid_date.year, valid_date.month, valid_date.day).and_hms(0, 0, 0);

    let result = Timestamp {
        unix: process_date.timestamp(),
        utc: Utc::from_utc_datetime(&Utc, &process_date).to_rfc2822().to_string(),
    };

    HttpResponse::Ok().json(result)
}
