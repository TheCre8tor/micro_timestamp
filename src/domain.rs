use actix_web::cookie::time::Error;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DateParamName {
    year: i64,
    month: u32,
    day: u32,
}

impl DateParamName {
    pub fn parse(date: &String) -> Result<Self, String> {
        let split_value = date.replace("-", " ");
        let string_to_vec = split_value.split_inclusive(" ").map(transform_string).collect();

        println!("{:?}", string_to_vec);

        // if string_to_vec.len() == 3 {
        //     if string_to_vec.contains(&"".to_string()) {
        //         return Err(format!("You provided an invalid date"));
        //     }
        //
        //     let con = string_to_vec;
        //     return Ok(DateParamName(string_to_vec));
        // }

        Err(format!("You provided an invalid date"))
    }
}

fn transform_string(value: &str) -> i64 {
    let mut aloc: Vec<i64> = Vec::new();

    let did = if value != "" {
        value.trim().to_string().parse().unwrap()
    };

    // aloc
}