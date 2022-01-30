use serde::Deserialize;
use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Deserialize, Debug)]
pub struct DateParamName {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl DateParamName {
    pub fn parse(date: &String) -> Result<Self, String> {
        if date.contains("-") {
            date_route(date)
        } else {
            unix_timestamp_route(date)
        }
    }
}

fn transform_string(value: &str) -> i32 {
    match value != " " {
        true => value.trim().to_string().parse::<i32>().unwrap(),
        false => 0,
    }
}

fn date_route(date: &String) -> Result<DateParamName, String> {
    // 1. Replace all "-" with an empty space
    let filter_date = date.replace("-", " ");

    // 2. a. Split filter_date where there's an empty value
    //    b. Map through filter_date values, convert all
    //       strings into i64 primitive value and replace
    //       all empty string with 0
    //    c. Transform the result into Vec<i64> collection.
    let date_list: Vec<i32> = filter_date
        .split_inclusive(" ")
        .map(transform_string)
        .collect();

    // 3. a. Check if date_list length equals 3, if false
    //       return an error of type Err.
    //    b. And if true, check each items in date_list if
    //       it contains 0, return error of type Err, else
    //       return DateParamName.
    return if date_list.len().eq(&3) {
        // 4. Validate if Y:M:D values is not greater than expected
        if date_list[0] < 0 || date_list[1] > 12 || date_list[2] > 31 {
            return Err(format!("{} is not a valid date", date));
        }

        if date_list.contains(&0i32) {
            Err(format!("{} is not a valid date", date))
        } else {
            Ok(DateParamName {
                year: date_list[0],
                month: date_list[1] as u32,
                day: date_list[2] as u32,
            })
        }
    } else {
        Err(format!("{} is not a valid date", date))
    };
}

fn unix_timestamp_route(date: &String) -> Result<DateParamName, String> {
    let parse_timestamp = match date.parse::<i64>() {
        Ok(timestamp) => timestamp,
        Err(_) => return Err(format!("Enter a valid unix timestamp"))
    };

    let timestamp = match NaiveDateTime::from_timestamp_opt(parse_timestamp, 0) {
        Some(datetime) => datetime,
        None => return Err(format!("Date out of range"))
    };

    let datetime = DateTime::<Utc>::from_utc(timestamp, Utc);
    let timestamp_str = datetime.format("%Y-%m-%d").to_string();

    date_route(&timestamp_str)
}
