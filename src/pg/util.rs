use postgres::error as pg_error;

pub fn pg_error_to_description(err: pg_error::Error) -> String {
    match err {
        pg_error::Error::Db(err) => {
            match err.detail {
                Some(ref detail) => format!("[{}] {}: {}", err.code.code(), err.message, detail),
                None => format!("[{}] {}", err.code.code(), err.message),
            }
        }
        pg_error::Error::Io(_) => "Could not communicate with the database instance".to_string(),
        pg_error::Error::Conversion(err) => panic!(err),
    }
}
