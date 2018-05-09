use regex;
use actix_web::{error, Error};
use statics;
use std::fs::File;
use std::io::Read;
use std::path::Path;

lazy_static! {
    static ref SCRIPT_NAME_VALIDATOR: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_-]+(\.lua)?$").unwrap();
}

// TODO: use async file i/o

/// Gets the path to and the contents of a script by its name.
///
/// # Errors
/// Returns an error if the script has an invalid name, does not exist,
/// or could not be read.
pub fn get_script_file(name: String) -> Result<(String, String), Error> {
    if !SCRIPT_NAME_VALIDATOR.is_match(&name[..]) {
        return Err(error::ErrorBadRequest("Invalid script name"));
    }

    let path = Path::new(&statics::SCRIPT_ROOT[..]).join(name);

    let path_str = match path.to_str() {
        Some(path_str) => path_str,
        None => {
            return Err(error::ErrorInternalServerError("Could not stringify script path"));
        }
    };

    match File::open(&path) {
        Ok(mut file) => {
            let mut contents = String::new();

            match file.read_to_string(&mut contents) {
                Ok(_) => Ok((path_str.to_string(), contents)),
                Err(_) => Err(error::ErrorNotFound("Could not read script"))
            }
        }
        Err(_) => Err(error::ErrorNotFound("Could not load script"))
    }
}
