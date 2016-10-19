use regex::Regex;
use std::str;
use std::process::Command;
use nutrino::*;

lazy_static! {
    static ref ACCOUNT_ID_MATCHER: Regex = Regex::new(r"Account ID: (\d+)").unwrap();
    static ref ACCOUNT_SECRET_MATCHER: Regex = Regex::new(r"Account secret: (.+)").unwrap();
}

pub fn create_account(email: String) -> Result<(i64, String), Error> {
    let create_user_output = Command::new("./target/debug/nutrino-user")
        .arg("add")
        .arg(email)
        .output();

    match create_user_output {
        Ok(output) => {
            if !output.status.success() {
                Err(Error::Unexpected(format!("Unexpected exit status running `nutrino-user`: {}", output.status)))
            } else {
                let stdout = str::from_utf8(&output.stdout).unwrap();
                let account_id: i64 = ACCOUNT_ID_MATCHER.captures(stdout).unwrap().at(1).unwrap().parse::<i64>().unwrap();
                let secret: String = ACCOUNT_SECRET_MATCHER.captures(stdout).unwrap().at(1).unwrap().to_string();
                Ok((account_id, secret))
            }
        },
        Err(err) => {
            Err(Error::Unexpected(format!("Could not run `nutrino-user`: {}", err)))
        }
    }
}

pub fn delete_account(user_id: i64) -> Result<(), Error> {
    let remove_user_status = Command::new("./target/debug/nutrino-user")
        .arg("remove")
        .arg(user_id.to_string())
        .status();

    match remove_user_status {
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(Error::Unexpected(format!("Unexpected exit status running `nutrino-user`: {}", status.code().unwrap())))
            }
        },
        Err(err) => {
            Err(Error::Unexpected(format!("Unexpected error running `nutrino-user`: {}", err)))
        }
    }
}
