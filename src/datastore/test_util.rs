use super::Transaction;
use responses::{Response, ErrorResponse};
use traits::Id;
use std::collections::BTreeMap;
use core::ops::{Add, Sub};
use chrono::duration::Duration;
use chrono::UTC;
use chrono::naive::datetime::NaiveDateTime;
use serde_json::Value as JsonValue;

pub fn create_test_properties(name: &str) -> BTreeMap<String, JsonValue> {
	let mut props = BTreeMap::new();
	props.insert("name".to_string(), JsonValue::String(name.to_string()));
	props
}

pub fn response_from_transaction<T: Transaction<I>, I: Id>(trans: &mut T, len: usize) -> Vec<Result<Response<I>, ErrorResponse<I>>> {
	let result = trans.commit();
	assert!(result.is_ok());
	let payload = result.unwrap();
	assert_eq!(payload.len(), len);
	payload
}

pub fn single_response_from_transaction<T: Transaction<I>, I: Id>(trans: &mut T) -> Result<Response<I>, ErrorResponse<I>> {
	let payload = response_from_transaction(trans, 1);
	payload.get(0).unwrap().clone()
}

pub fn get_before() -> Option<NaiveDateTime> {
	let time = UTC::now().sub(Duration::days(1));
	Option::Some(NaiveDateTime::from_timestamp(time.timestamp(), 0))
}

pub fn get_after() -> Option<NaiveDateTime> {
	let time = UTC::now().add(Duration::days(1));
	Option::Some(NaiveDateTime::from_timestamp(time.timestamp(), 0))
}
