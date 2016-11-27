use iron::prelude::*;
use iron::headers::{Authorization, Basic};
use nutrino::Datastore;
use std::collections::BTreeMap;
use iron::modifiers::Header as HeaderModifier;
use statics;
use uuid::Uuid;
use iron::middleware::{BeforeMiddleware, AfterMiddleware};
use serde_json;
use iron::status;
use router::NoRoute;
use util::SimpleError;
use super::util::*;
use core::str::FromStr;

header! { (WWWAuthenticate, "WWW-Authenticate") => [String] }

// -- Basic http auth middleware
pub struct BasicAuthMiddleware {
}

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

		if account_id.is_some() && secret.is_some() && statics::DATASTORE.auth(account_id.unwrap(), secret.unwrap()).unwrap_or(false) {
			req.extensions.insert::<AccountKey>(AccountKey {
				account_id: account_id.unwrap()
			});

			return Ok(());
		}

		let error_message = "Authentication failed".to_string();

		let mut d: BTreeMap<String, String> = BTreeMap::new();
		d.insert("error".to_string(), error_message.clone());
		let body = serde_json::to_string(&d).unwrap();

		let www_authenticate_header = WWWAuthenticate("Basic realm=\"main\"".to_owned());
		let www_authenticate_modifier = HeaderModifier(www_authenticate_header);
		let json_content_type_modifier = HeaderModifier(get_json_content_type());

		let modifiers = (status::Unauthorized, json_content_type_modifier, www_authenticate_modifier, body);
		Err(IronError::new(SimpleError::new(error_message), modifiers))
	}
}

// -- Error middleware
pub struct ErrorMiddleware {
}

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
			Err(create_iron_error(status::Status::NotFound, "No route found".to_string()))
		} else {
			Err(err)
		}
	}
}
