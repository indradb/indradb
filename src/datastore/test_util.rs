use core::ops::{Add, Sub};
use chrono::duration::Duration;
use chrono::UTC;
use chrono::naive::datetime::NaiveDateTime;

pub fn get_before() -> Option<NaiveDateTime> {
	let time = UTC::now().sub(Duration::days(1));
	Option::Some(NaiveDateTime::from_timestamp(time.timestamp(), 0))
}

pub fn get_after() -> Option<NaiveDateTime> {
	let time = UTC::now().add(Duration::days(1));
	Option::Some(NaiveDateTime::from_timestamp(time.timestamp(), 0))
}
