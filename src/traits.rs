use std::fmt::Debug;
use serde::ser::Serialize;
use serde::de::Deserialize;
use std::default::Default;
use core::hash::Hash;
use std::marker::Copy;
use uuid::Uuid;

pub trait Id: Clone + Debug + Serialize + Deserialize + Eq + Default + Hash + Copy {}

impl Id for Uuid {}
