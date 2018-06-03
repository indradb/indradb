use std::collections::BTreeMap;
use std::path::Path;
use std::fs::File;
use statics;
use regex;
use std::io::Read;

lazy_static! {
    static ref SCRIPT_NAME_VALIDATOR: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_-]+(\.lua)?$").unwrap();
}

#[derive(Clone, Debug)]
pub enum ReaderError {
    InvalidName,
    InvalidPath,
    Read
}

pub struct ReaderValue<'a> {
    pub path: &'a str,
    pub contents: &'a str
}

pub struct Reader {
    cache: BTreeMap<String, String>
}

impl Reader {
    pub fn new() -> Self {
        Self { cache: BTreeMap::new() }
    }

    pub fn get(&self, name: &str) -> Result<ReaderValue, ReaderError> {
        if !SCRIPT_NAME_VALIDATOR.is_match(name) {
            return Err(ReaderError::InvalidName);
        }

        let path = Path::new(&*statics::SCRIPT_ROOT).join(name);
        let path_str = path.to_str().ok_or_else(|| ReaderError::InvalidPath)?;

        // TODO: this could probably be optimized a bit with the entry API,
        // though it's complicated by the fact that reading the file contents
        // may yield an error
        if !self.cache.contains_key(name) {
            match File::open(name) {
                Ok(mut file) => {
                    let mut contents = String::new();

                    match file.read_to_string(&mut contents) {
                        Ok(_) => self.cache.insert(name.to_string(), contents.clone()),
                        Err(_) => return Err(ReaderError::Read)
                    }
                }
                Err(_) => return Err(ReaderError::Read)
            };
        }

        Ok(ReaderValue {
            path: path_str,
            contents: &self.cache[name]
        })
    }
}
