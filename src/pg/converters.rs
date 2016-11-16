use r2d2::GetTimeout;
use postgres::error as pg_error;
use errors::Error;
use super::util::*;

impl From<pg_error::Error> for Error {
	fn from(err: pg_error::Error) -> Error {
		Error::Unexpected(pg_error_to_description(err))
	}
}

impl From<GetTimeout> for Error {
	fn from(err: GetTimeout) -> Error {
		Error::Unexpected(format!("Could not fetch connection: {}", err))
	}
}