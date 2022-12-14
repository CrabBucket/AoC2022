use ureq::{self, get};
use chrono::{Datelike, Local,};
use std::env;
use dotenv;

pub fn get_input(day: u8) -> Box<str> {
    dotenv::dotenv().ok();
    let now = Local::now();
    let session = env::var("SESSION").unwrap();

    let url = format!("https://adventofcode.com/{}/day/{}/input", now.year(), day);
    let cookies = format!("session={}", session);
    let resp = get(&url).set("Cookie", &cookies).call().unwrap().into_string().unwrap();
    return resp.into();
}
