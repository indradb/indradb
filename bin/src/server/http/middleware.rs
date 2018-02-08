use iron::prelude::*;
use iron::middleware::AfterMiddleware;
use iron::status;
use router::NoRoute;
use super::util::*;

/// Error middleware
///
/// This produces a standard JSON body if an error occurred, and no JSON
/// body has been specified yet.
pub struct ErrorMiddleware {}

impl ErrorMiddleware {
    pub fn new() -> ErrorMiddleware {
        ErrorMiddleware {}
    }
}

impl AfterMiddleware for ErrorMiddleware {
    fn after(&self, _: &mut Request, res: Response) -> IronResult<Response> {
        Ok(res)
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        if err.error.is::<NoRoute>() {
            Err(create_iron_error(
                status::Status::NotFound,
                "No route found".to_string(),
            ))
        } else {
            Err(err)
        }
    }
}
