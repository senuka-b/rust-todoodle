use chrono::{NaiveDateTime, TimeZone, Utc };

use crate::input;

pub fn date() -> i64{
    let input: String = input::read::read_string("Enter due date (yyyy-mm-dd hh:mm:ss)", true);

    match NaiveDateTime::parse_from_str(&input.trim(), "%Y-%m-%d %H:%M:%S") {
        Ok(result) => {

            Utc.from_utc_datetime(&result).timestamp()
        },

        Err(er) => {
            
            println!("{} ", er);

            println!("\nUnable to convert due date to a suitable format! Please try again\n");

            date()
        }
    }
}