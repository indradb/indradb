use iron::prelude::*;
use iron::headers::{Authorization, Basic};
use braid::Datastore;
use std::collections::BTreeMap;
use statics;
use uuid::Uuid;
use iron::middleware::{AfterMiddleware, BeforeMiddleware};
use serde_json;
use iron::status;
use router::NoRoute;
use util::SimpleError;
use super::util::*;
use core::str::FromStr;
use iron::headers::ContentType;

/// Basic HTTP auth middleware.
pub struct BasicAuthMiddleware {}

impl BasicAuthMiddleware {
    pub fn new() -> BasicAuthMiddleware {
        BasicAuthMiddleware {}
    }

    fn get_account_id(&self, auth: Option<&Authorization<Basic>>) -> Option<Uuid> {
        if let Some(auth) = auth {
            if let Ok(val) = Uuid::from_str(&auth.username[..]) {
                return Some(val);
            }
        }

        None
    }

    fn get_secret(&self, auth: Option<&Authorization<Basic>>) -> Option<String> {
        if let Some(auth) = auth {
            return auth.password.clone();
        }

        None
    }
}

impl BeforeMiddleware for BasicAuthMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let auth = req.headers.get::<Authorization<Basic>>();
        let account_id = self.get_account_id(auth);
        let secret = self.get_secret(auth);

        if account_id.is_some() && secret.is_some()
            && statics::DATASTORE
                .auth(account_id.unwrap(), secret.unwrap())
                .unwrap_or(false)
        {
            req.extensions.insert::<AccountKey>(AccountKey {
                account_id: account_id.unwrap(),
            });

            return Ok(());
        }

        let error_message = "Authentication failed".to_string();

        let mut d: BTreeMap<String, String> = BTreeMap::new();
        d.insert("error".to_string(), error_message.clone());
        let body = serde_json::to_string(&d).unwrap();

        // NOTE: Right now we're manually constructing the `Response` and
        // `IronError`. Ideally this would not happen, however otherwise we'd
        // need to resort to the use of `Modifier`. Because we're setting a
        // custom header, that requires a lot of plumbing at the moment. There
        // should be a more terse way to do this - at least in future versions
        // of iron.
        let mut response = Response::new();
        response.status = Some(status::Unauthorized);
        response.headers.set(ContentType(get_json_mime()));
        response.headers.set_raw(
            "WWW-Authenticate",
            vec![b"Basic realm=\"main\"".to_vec()],
        );
        response.body = Some(Box::new(body));

        let error = IronError {
            error: Box::new(SimpleError::new(error_message)),
            response: response,
        };

        Err(error)
    }
}

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
