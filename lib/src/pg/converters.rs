use postgres::error::Error as PostgresError;
use r2d2::Error as R2D2Error;
use errors::Error;
use super::util::pg_error_to_description;

impl From<PostgresError> for Error {
    fn from(err: PostgresError) -> Error {
        Error::Unexpected(pg_error_to_description(err))
    }
}

impl From<R2D2Error> for Error {
    fn from(err: R2D2Error) -> Error {
        Error::Unexpected(format!("{}", err))
    }
}
