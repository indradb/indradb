//! Utility functions.

use crate::errors::Result;
use rand::prelude::*;
use rand::rngs::OsRng;
use std::env;

const TEMP_PATH_RANDOM_PART_LENGTH: usize = 8;

/// Gets the path to a file or directory within the temporary directory, in a
/// platform-independent manner. Note that this will panic if the path cannot
/// be formatted into UTF-8.
pub fn generate_temporary_path() -> String {
    let mut path = env::temp_dir();
    path.push(generate_random_secret(TEMP_PATH_RANDOM_PART_LENGTH));
    path.to_str()
        .expect("Expected to be able to parse the temp path into UTF-8")
        .to_string()
}

/// Generates a securely random string consisting of letters (uppercase and
/// lowercase) and digits.
pub fn generate_random_secret(count: usize) -> String {
    let mut chars = vec![];
    let options = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    for _ in 0..count {
        // Don't use `choose_multiple`, because it shuffles the list (i.e.
        // prevents duplicates)
        let c: u8 = *options.choose(&mut OsRng).unwrap();
        chars.push(c);
    }

    String::from_utf8(chars).unwrap()
}

pub fn remove_nones_from_iterator<I, T>(iter: I) -> impl Iterator<Item = Result<T>>
where
    I: Iterator<Item = Result<Option<T>>>,
{
    iter.filter_map(|item| match item {
        Err(err) => Some(Err(err)),
        Ok(Some(value)) => Some(Ok(value)),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::{generate_random_secret, generate_temporary_path};
    use regex::Regex;

    #[test]
    fn should_generate_temporary_path() {
        let first = generate_temporary_path();
        let second = generate_temporary_path();
        assert!(first.len() > 0);
        assert!(second.len() > 0);
        assert_ne!(first, second);
    }

    #[test]
    fn should_generate_random_secret() {
        let secret = generate_random_secret(62);
        assert!(Regex::new(r"[a-zA-Z0-9]{62}").unwrap().is_match(&secret[..]));
    }
}
