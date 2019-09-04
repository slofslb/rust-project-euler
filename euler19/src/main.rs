//extern crate chrono;
//use chrono::{NaiveDate, DateTime, Duration, Utc};
use time::Duration;
use chrono::prelude::*;

fn main() {
    let mut count = 0;
    let mut d = Utc.ymd(1901, 1, 1);
    while d <= Utc.ymd(2000,12,31) {
        if d.weekday() == Weekday::Sun && d.day() == 1 {
            println!("{:?}", d);
            count += 1;
        }
        d = d + Duration::days(1);
    }
    println!("{}", count);
}
// 171
