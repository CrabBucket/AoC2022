use ureq::{self, get};
use chrono::{Datelike, Local,};
use std::env;
use dotenv;

pub fn get_input() -> Box<str> {
    dotenv::dotenv();
    let now = Local::now();
    let session = env::var("SESSION").unwrap();

    let url = format!("https://adventofcode.com/{}/day/{}/input", now.year(), now.day()+1);
    let cookies = format!("session={}", session);
    let resp = get(&url).set("Cookie", &cookies).call().unwrap().into_string().unwrap();
    return resp.into();
}
