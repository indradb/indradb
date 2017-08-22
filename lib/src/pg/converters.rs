use r2d2::GetTimeout;
use postgres::error::Error as PostgresError;
use errors::Error;
use super::util::pg_error_to_description;

impl From<PostgresError> for Error {
    fn from(err: PostgresError) -> Error {
        Error::Unexpected(pg_error_to_description(err))
    }
}

impl From<GetTimeout> for Error {
    fn from(err: GetTimeout) -> Error {
        Error::Unexpected(format!("Could not fetch connection: {}", err))
    }
}
