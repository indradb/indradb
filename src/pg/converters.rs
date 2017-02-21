use r2d2::GetTimeout;
use postgres::error::Error as PostgresError;
use r2d2_postgres::Error as R2D2PostgresError;
use errors::Error;
use super::util::*;

impl From<R2D2PostgresError> for Error {
    fn from(err: R2D2PostgresError) -> Error {
        Error::Unexpected(format!("{}", err))
    }
}

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
