use regex::Regex;
use std::str;
use std::process::Command;
use nutrino::*;
use uuid::Uuid;
use std::str::FromStr;

lazy_static! {
    static ref ACCOUNT_ID_MATCHER: Regex = Regex::new(r"Account ID: (.+)").unwrap();
    static ref ACCOUNT_SECRET_MATCHER: Regex = Regex::new(r"Account secret: (.+)").unwrap();
}

pub fn create_account(email: String) -> Result<(Uuid, String), Error> {
    let create_user_output = Command::new("./target/debug/nutrino-user")
        .arg("add")
        .arg(email)
        .output();

    match create_user_output {
        Ok(output) => {
            if !output.status.success() {
                Err(Error::Unexpected(format!("Unexpected exit status running `nutrino-user \
                                               add`: {}",
                                              output.status)))
            } else {
                let stdout = str::from_utf8(&output.stdout).unwrap();
                let account_id_str = ACCOUNT_ID_MATCHER.captures(stdout).unwrap().at(1).unwrap();
                let account_id = Uuid::from_str(account_id_str).unwrap();
                let secret =
                    ACCOUNT_SECRET_MATCHER.captures(stdout).unwrap().at(1).unwrap().to_string();
                Ok((account_id, secret))
            }
        }
        Err(err) => Err(Error::Unexpected(format!("Could not run `nutrino-user`: {}", err))),
    }
}

pub fn delete_account(account_id: Uuid) -> Result<(), Error> {
    let remove_user_status = Command::new("./target/debug/nutrino-user")
        .arg("remove")
        .arg(account_id.to_string())
        .status();

    match remove_user_status {
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(Error::Unexpected(format!("Unexpected exit status running `nutrino-user \
                                               remove`: {}",
                                              status.code().unwrap())))
            }
        }
        Err(err) => {
            Err(Error::Unexpected(format!("Unexpected error running `nutrino-user`: {}", err)))
        }
    }
}
