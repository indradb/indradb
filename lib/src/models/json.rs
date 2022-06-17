use std::cmp::Ordering;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

fn hash<H: Hasher>(value: &serde_json::Value, state: &mut H) {
    match value {
        serde_json::Value::Null => {
            state.write_u8(0);
        }
        serde_json::Value::Bool(v) => {
            state.write_u8(1);
            v.hash(state);
        }
        serde_json::Value::Number(v) => {
            state.write_u8(2);
            if v.is_i64() {
                v.as_i64().unwrap().hash(state);
            } else if v.is_u64() {
                v.as_u64().unwrap().hash(state);
            } else {
                // Note that there are 16m different NaN values, and they won't hash the same here
                v.as_f64().unwrap().to_bits().hash(state);
            }
        }
        serde_json::Value::String(v) => {
            state.write_u8(3);
            v.hash(state);
        }
        serde_json::Value::Array(v) => {
            state.write_u8(4);
            for sv in v {
                hash(sv, state);
            }
        }
        serde_json::Value::Object(v) => {
            state.write_u8(5);
            for (sk, sv) in v {
                sk.hash(state);
                hash(sv, state);
            }
        }
    }
}

fn partial_cmp(first: &serde_json::Value, second: &serde_json::Value) -> Option<Ordering> {
    match (first, second) {
        (serde_json::Value::Null, serde_json::Value::Null) => Some(Ordering::Equal),
        (serde_json::Value::Bool(v1), serde_json::Value::Bool(v2)) => v1.partial_cmp(v2),
        (serde_json::Value::Number(v1), serde_json::Value::Number(v2)) => {
            if v1.is_i64() {
                let v1 = v1.as_i64().unwrap();
                if v2.is_i64() {
                    v1.partial_cmp(&v2.as_i64().unwrap())
                } else if v2.is_u64() {
                    match i64::try_from(v2.as_u64().unwrap()) {
                        Ok(v2) => v1.partial_cmp(&v2),
                        Err(_) => Some(Ordering::Less),
                    }
                } else {
                    (v1 as f64).partial_cmp(&v2.as_f64().unwrap())
                }
            } else if v1.is_u64() {
                let v1 = v1.as_u64().unwrap();
                if v2.is_i64() {
                    match u64::try_from(v2.as_i64().unwrap()) {
                        Ok(v2) => v1.partial_cmp(&v2),
                        Err(_) => Some(Ordering::Greater),
                    }
                } else if v2.is_u64() {
                    v1.partial_cmp(&v2.as_u64().unwrap())
                } else {
                    (v1 as f64).partial_cmp(&v2.as_f64().unwrap())
                }
            } else {
                let v1 = v1.as_f64().unwrap();
                if v2.is_i64() {
                    v1.partial_cmp(&(v2.as_i64().unwrap() as f64))
                } else if v2.is_u64() {
                    v1.partial_cmp(&(v2.as_u64().unwrap() as f64))
                } else {
                    v1.partial_cmp(&v2.as_f64().unwrap())
                }
            }
        }
        (serde_json::Value::String(v1), serde_json::Value::String(v2)) => v1.partial_cmp(v2),
        (serde_json::Value::Array(v1), serde_json::Value::Array(v2)) => {
            partial_cmp_by(v1.iter(), v2.iter(), partial_cmp)
        }
        (serde_json::Value::Object(v1), serde_json::Value::Object(v2)) => {
            partial_cmp_by(v1.iter(), v2.iter(), |v1, v2| {
                let (v1_key, v1_value) = v1;
                let (v2_key, v2_value) = v2;
                match v1_key.partial_cmp(v2_key) {
                    Some(Ordering::Equal) => partial_cmp(v1_value, v2_value),
                    non_eq => non_eq,
                }
            })
        }
        _ => None,
    }
}

fn partial_cmp_by<I, F>(mut first: I, mut second: I, mut f: F) -> Option<Ordering>
where
    I: Iterator,
    F: FnMut(I::Item, I::Item) -> Option<Ordering>,
{
    loop {
        let x = match first.next() {
            None => {
                if second.next().is_none() {
                    return Some(Ordering::Equal);
                } else {
                    return Some(Ordering::Less);
                }
            }
            Some(val) => val,
        };

        let y = match second.next() {
            None => return Some(Ordering::Greater),
            Some(val) => val,
        };

        match f(x, y) {
            Some(Ordering::Equal) => (),
            non_eq => return non_eq,
        }
    }
}

/// A [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
/// that extends `serde_json::Value` with extra traits useful for datastore
/// storage and querying. Publicly facing APIs do not use these values, so
/// it's generally only useful for datastore authors.
#[derive(Clone, Eq, Debug, Serialize, Deserialize)]
pub struct Json(pub serde_json::Value);

impl Json {
    pub fn new(value: serde_json::Value) -> Self {
        Self(value)
    }
}

impl From<serde_json::Value> for Json {
    fn from(value: serde_json::Value) -> Self {
        Json(value)
    }
}

impl Hash for Json {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash(&self.0, state);
    }
}

impl PartialEq for Json {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Json {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        partial_cmp(&self.0, &other.0)
    }
}
