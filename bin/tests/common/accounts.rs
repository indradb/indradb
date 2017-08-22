use regex::Regex;
use std::str;
use std::process::Command;
use braid::*;
use uuid::Uuid;
use std::str::FromStr;

lazy_static! {
    static ref ACCOUNT_ID_MATCHER: Regex = Regex::new(r"Account ID: (.+)").unwrap();
    static ref ACCOUNT_SECRET_MATCHER: Regex = Regex::new(r"Account secret: (.+)").unwrap();
}

pub fn create_account() -> Result<(Uuid, String), Error> {
    let create_user_output = Command::new("./target/debug/braid-account")
        .arg("add")
        .output();

    match create_user_output {
        Ok(output) => {
            if !output.status.success() {
                let message = format!(
                    "Unexpected exit status running `braid-account add`: {}",
                    output.status
                );
                Err(Error::Unexpected(message))
            } else {
                let stdout = str::from_utf8(&output.stdout).unwrap();
                let account_id_str = ACCOUNT_ID_MATCHER
                    .captures(stdout)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str();
                let account_id = Uuid::from_str(account_id_str).unwrap();
                let secret = ACCOUNT_SECRET_MATCHER
                    .captures(stdout)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str();
                Ok((account_id, secret.to_string()))
            }
        }
        Err(err) => Err(Error::Unexpected(
            format!("Could not run `braid-account`: {}", err),
        )),
    }
}

pub fn delete_account(account_id: Uuid) -> Result<(), Error> {
    let remove_user_status = Command::new("./target/debug/braid-account")
        .arg("remove")
        .arg(account_id.to_string())
        .status();

    match remove_user_status {
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                let message = format!(
                    "Unexpected exit status running `braid-account remove`: {}",
                    status.code().unwrap()
                );
                Err(Error::Unexpected(message))
            }
        }
        Err(err) => {
            Err(Error::Unexpected(
                format!("Unexpected error running `braid-account`: {}", err),
            ))
        }
    }
}
