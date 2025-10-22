use chrono::{NaiveDate, Datelike};


pub fn add_months(start: NaiveDate, months: u32) -> NaiveDate {
let years = months / 12;
let months = months % 12;
NaiveDate::from_ymd_opt(start.year() + years as i32, start.month(), start.day())
.unwrap()
.checked_add_months(chrono::Months::new(months as u32))
.unwrap()
}