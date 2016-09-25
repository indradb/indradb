use std::fmt::Debug;
use serde::ser::Serialize;
use serde::de::Deserialize;
use std::default::Default;
use core::hash::Hash;
use std::marker::Copy;

pub trait Id: Clone + Debug + Serialize + Deserialize + Eq + Default + Hash + Copy {}
