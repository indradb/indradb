use pg::r2d2_postgres::{SslMode, PostgresConnectionManager};
use pg::r2d2::{Config, Pool, GetTimeout};
use datastore::{Id, Datastore, Transaction};
use requests::Request;
use responses::{Response, ErrorResponse};
use models;
use util::{SimpleError, parse_json_object, generate_random_secret};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use pg::postgres;
use pg::postgres::rows::Rows;
use pg::postgres::error as pg_error;
use std::collections::BTreeMap;
use chrono::naive::datetime::NaiveDateTime;
use serde_json::Value as JsonValue;
use serde_json;
use pg::num_cpus;

impl Id for i64 {}

#[derive(Clone, Debug)]
pub struct PostgresDatastore {
	pool: Pool<PostgresConnectionManager>,
	secret: String
}

impl PostgresDatastore {
	pub fn new(pool_size: Option<u32>, connection_string: String, secret: String) -> PostgresDatastore {
		let unwrapped_pool_size: u32 = match pool_size {
			Some(val) => val,
			None => {
				let cpus: usize = num_cpus::get();

				if cpus > 512 {
					1024
				} else {
					cpus as u32 * 2
				}
			}
		};

		let pool_config = Config::builder().pool_size(unwrapped_pool_size).build();
		let manager = PostgresConnectionManager::new(&*connection_string, SslMode::None).unwrap();

		PostgresDatastore {
			pool: Pool::new(pool_config, manager).unwrap(),
			secret: secret
		}
	}

	fn get_salted_hash(&self, salt: String, secret: String) -> String {
		let mut sha = Sha256::new();
		sha.input(salt.as_bytes());
		sha.input(self.secret.as_bytes());
		sha.input(secret.as_bytes());
		return format!("1:{}", sha.result_str());
	}
}

impl Datastore<PostgresTransaction, i64> for PostgresDatastore {
	fn has_account(&self, user_id: i64) -> Result<bool, SimpleError> {
		let conn = try!(self.pool.get());
		let results = try!(conn.query("SELECT 1 FROM accounts WHERE id=$1", &[&(user_id as i32)]));

		for _ in &results {
			return Result::Ok(true);
		}

		Result::Ok(false)
	}

	fn create_account(&self, email: String) -> Result<(i64, String), SimpleError> {
		let salt = generate_random_secret();
		let secret = generate_random_secret();
		let hash = self.get_salted_hash(salt.clone(), secret.clone());
		let conn = try!(self.pool.get());
		let results = try!(conn.query("INSERT INTO accounts(email, salt, api_secret_hash) VALUES ($1, $2, $3) RETURNING id", &[&email, &salt, &hash]));

		for row in &results {
			let id: i32 = row.get(0);
			return Result::Ok((id as i64, secret));
		}

		Result::Err(SimpleError::new("Hit unreachable code"))
	}

	fn delete_account(&self, user_id: i64) -> Result<(), SimpleError> {
		let conn = try!(self.pool.get());
		let results = try!(conn.query("DELETE FROM accounts WHERE id=$1 RETURNING 1", &[&(user_id as i32)]));

		for _ in &results {
			return Result::Ok(());
		}

		Result::Err(SimpleError::new("Account not found"))
	}

	fn auth(&self, user_id: i64, secret: String) -> Result<bool, SimpleError> {
		let conn = try!(self.pool.get());
		let get_salt_results = try!(conn.query("SELECT salt FROM accounts WHERE id=$1", &[&(user_id as i32)]));

		for row in &get_salt_results {
			let salt = row.get(0);
			let expected_hash = self.get_salted_hash(salt, secret);
			let auth_results = try!(conn.query("SELECT 1 FROM accounts WHERE id=$1 AND api_secret_hash=$2", &[&(user_id as i32), &expected_hash]));

			for _ in &auth_results {
				return Result::Ok(true);
			}

			return Result::Ok(false);
		}

		Result::Ok(false)
	}

	fn transaction(&self, user_id: i64) -> Result<PostgresTransaction, SimpleError> {
		let conn = try!(self.pool.get());
		let results = try!(conn.query("SELECT id FROM accounts WHERE id=$1", &[&(user_id as i32)]));

		for row in &results {
			return Ok(PostgresTransaction {
				pool: self.pool.clone(),
				account_id: row.get(0),
				requests: Vec::new()
			});
		}

		Err(SimpleError::new("Unknown account"))
	}
}

fn pg_error_to_description(err: pg_error::Error) -> String {
	match err {
		pg_error::Error::Db(err) => {
			match err.detail {
				Some(ref detail) => format!("[{}] {}: {}", err.code.code(), err.message, detail),
				None => format!("[{}] {}", err.code.code(), err.message)
			}
		},
		pg_error::Error::Io(_) => "Could not communicate with the database instance".to_string(),
		pg_error::Error::Conversion(err) => panic!(err)
	}
}

impl From<pg_error::Error> for ErrorResponse {
	fn from(err: pg_error::Error) -> ErrorResponse {
		ErrorResponse::Unexpected(pg_error_to_description(err))
	}
}

impl From<pg_error::Error> for SimpleError {
	fn from(err: pg_error::Error) -> SimpleError {
		SimpleError::new_from_string(pg_error_to_description(err))
	}
}

impl From<GetTimeout> for SimpleError {
	fn from(err: GetTimeout) -> SimpleError {
		SimpleError::new_from_string(format!("Could not fetch connection: {}", err))
	}
}

#[derive(Clone, Debug)]
pub struct PostgresTransaction {
	pool: Pool<PostgresConnectionManager>,
	account_id: i32,
	requests: Vec<Request>
}

impl PostgresTransaction {
	fn get_vertex(&self, trans: &postgres::Transaction, id: i64) -> Result<Response, ErrorResponse> {
		let results = try!(trans.query("SELECT properties, type FROM vertices WHERE id=$1 LIMIT 1", &[&id]));

		for row in &results {
			let properties_str: String = row.get(0);
			let properties_obj = try!(parse_json_object(properties_str));
			let t: String = row.get(1);
			let v = models::Vertex::new_with_properties(id, t, properties_obj);
			return Ok(Response::Vertex(v))
		}

		Err(ErrorResponse::VertexDoesNotExist(id))
	}

	fn create_vertex(&self, trans: &postgres::Transaction, t: String, properties: BTreeMap<String, JsonValue>) -> Result<Response, ErrorResponse> {
		let results = try!(trans.query("
			INSERT INTO vertices (type, owner_id, properties) VALUES ($1, $2, $3) RETURNING id
		", &[&t, &self.account_id, &serde_json::to_string(&properties).unwrap()]));

		for row in &results {
			let id: i64 = row.get(0);
			return Ok(Response::VertexId(id))
		}

		panic!("Unreachable point hit")
	}

	fn set_vertex(&self, trans: &postgres::Transaction, v: models::Vertex) -> Result<Response, ErrorResponse> {
		let results = try!(trans.query("
			UPDATE vertices
			SET type=$1, properties=$2
			WHERE id=$3 AND owner_id=$4
			RETURNING 1
		", &[&v.t, &serde_json::to_string(&v.properties).unwrap(), &v.id, &self.account_id]));

		for _ in &results {
			return Ok(Response::Ok)
		}

		return Err(ErrorResponse::VertexDoesNotExist(v.id))
	}

	fn delete_vertex(&self, trans: &postgres::Transaction, id: i64) -> Result<Response, ErrorResponse> {
		let results = try!(trans.query("DELETE FROM vertices WHERE id=$1 AND owner_id=$2 RETURNING 1", &[&id, &self.account_id]));

		for _ in &results {
			return Ok(Response::Ok)
		}

		Err(ErrorResponse::VertexDoesNotExist(id))
	}

	fn get_edge(&self, trans: &postgres::Transaction, outbound_id: i64, t: String, inbound_id: i64) -> Result<Response, ErrorResponse> {
		let results = try!(trans.query("
			SELECT properties, weight FROM edges WHERE outbound_id=$1 AND type=$2 AND inbound_id=$3 LIMIT 1
		", &[&outbound_id, &t, &inbound_id]));

		for row in &results {
			let properties_str: String = row.get(0);
			let properties_obj = try!(parse_json_object(properties_str));
			let weight: f32 = row.get(1);
			let v = models::Edge::new_with_properties(outbound_id, t, inbound_id, weight, properties_obj);
			return Ok(Response::Edge(v))
		}

		Err(ErrorResponse::EdgeDoesNotExist(outbound_id, t, inbound_id))
	}

	fn set_edge(&self, trans: &postgres::Transaction, e: models::Edge) -> Result<Response, ErrorResponse> {
		if e.weight < -1.0 || e.weight > 1.0 {
			return Err(ErrorResponse::WeightOutOfRange)
		}

		let results = trans.query("
			INSERT INTO edges (outbound_id, type, inbound_id, weight, properties, update_date)
			VALUES ((SELECT id FROM vertices WHERE id=$1 AND owner_id=$2), $3, $4, $5, $6, NOW())
			ON CONFLICT ON CONSTRAINT edges_pkey DO UPDATE SET weight=$5, properties=$6, update_date=NOW()
			RETURNING 1
		", &[&e.outbound_id, &self.account_id, &e.t, &e.inbound_id, &e.weight, &serde_json::to_string(&e.properties).unwrap()]);

		if results.is_err() {
			let err = results.unwrap_err();

			return match err {
				pg_error::Error::Db(ref db_err) => {
					match db_err.code {
						// This should only happen when the inner select fails
						pg_error::SqlState::NotNullViolation => Err(ErrorResponse::VertexDoesNotExist(e.outbound_id)),

						// This should only happen when there is no vertex with id=inbound_id
						pg_error::SqlState::ForeignKeyViolation => Err(ErrorResponse::VertexDoesNotExist(e.inbound_id)),

						// Other db error
						_ => Err(ErrorResponse::Unexpected(db_err.message.clone()))
					}
				},
				pg_error::Error::Io(_) => Err(ErrorResponse::Unexpected("I/O".to_string())),
				pg_error::Error::Conversion(err) => panic!(err)
			};
		}

		for _ in &results.unwrap() {
			return Ok(Response::Ok)
		}

		return Err(ErrorResponse::VertexDoesNotExist(e.outbound_id))
	}

	fn delete_edge(&self, trans: &postgres::Transaction, outbound_id: i64, t: String, inbound_id: i64) -> Result<Response, ErrorResponse> {
		let results = try!(trans.query("
			DELETE FROM EDGES
			WHERE outbound_id=(SELECT id FROM vertices WHERE id=$1 AND owner_id=$2) AND type=$3 AND inbound_id=$4
			RETURNING 1
		", &[&outbound_id, &self.account_id, &t, &inbound_id]));

		for _ in &results {
			return Ok(Response::Ok)
		}

		return Err(ErrorResponse::EdgeDoesNotExist(outbound_id, t, inbound_id))
	}

	fn get_edge_count(&self, trans: &postgres::Transaction, outbound_id: i64, t: String) -> Result<Response, ErrorResponse> {
		let results = try!(trans.query("
			SELECT COUNT(outbound_id) FROM edges WHERE outbound_id=$1 AND type=$2
		", &[&outbound_id, &t]));

		for row in &results {
			let count: i64 = row.get(0);
			return Ok(Response::Count(count))
		}

		panic!("Unreachable point hit")
	}

	fn get_edge_range(&self, trans: &postgres::Transaction, outbound_id: i64, t: String, offset: i64, limit: i64) -> Result<Response, ErrorResponse> {
		if offset < 0 {
			return Err(ErrorResponse::OffsetOutOfRange);
		}

		if limit < 0 {
			return Err(ErrorResponse::LimitOutOfRange);
		}

		let results = try!(trans.query("
			SELECT inbound_id, weight, properties
			FROM edges
			WHERE outbound_id=$1 AND type=$2
			ORDER BY update_date DESC
			OFFSET $3
			LIMIT $4
		", &[&outbound_id, &t, &offset, &limit]));

		self.fill_edges(results, outbound_id, t)
	}

	fn get_edge_time_range(&self, trans: &postgres::Transaction, outbound_id: i64, t: String, high: Option<NaiveDateTime>, low: Option<NaiveDateTime>, limit: i64) -> Result<Response, ErrorResponse> {
		if limit < 0 {
			return Err(ErrorResponse::LimitOutOfRange);
		}

		let results = try!(match (high, low) {
			(Option::Some(high_unboxed), Option::Some(low_unboxed)) => {
				trans.query("
					SELECT inbound_id, weight, properties
					FROM edges
					WHERE outbound_id=$1 AND type=$2 AND update_date <= $3 AND update_date >= $4
					ORDER BY update_date DESC
					LIMIT $5
				", &[&outbound_id, &t, &high_unboxed, &low_unboxed, &limit])
			},
			(Option::Some(high_unboxed), Option::None) => {
				trans.query("
					SELECT inbound_id, weight, properties
					FROM edges
					WHERE outbound_id=$1 AND type=$2 AND update_date <= $3
					ORDER BY update_date DESC
					LIMIT $4
				", &[&outbound_id, &t, &high_unboxed, &limit])
			},
			(Option::None, Option::Some(low_unboxed)) => {
				trans.query("
					SELECT inbound_id, weight, properties
					FROM edges
					WHERE outbound_id=$1 AND type=$2 AND update_date >= $3
					ORDER BY update_date DESC
					LIMIT $4
				", &[&outbound_id, &t, &low_unboxed, &limit])
			},
			_ => {
				trans.query("
					SELECT inbound_id, weight, properties
					FROM edges
					WHERE outbound_id=$1 AND type=$2
					ORDER BY update_date DESC
					LIMIT $3
				", &[&outbound_id, &t, &limit])
			}
		});

		self.fill_edges(results, outbound_id, t)
	}

	fn fill_edges(&self, results: Rows, outbound_id: i64, t: String) -> Result<Response, ErrorResponse> {
		let mut edges: Vec<models::Edge> = Vec::new();

		for row in &results {
			let inbound_id: i64 = row.get(0);
			let weight: f32 = row.get(1);
			let properties_str: String = row.get(2);
			let properties_obj = try!(parse_json_object(properties_str));
			edges.push(models::Edge::new_with_properties(outbound_id, t.clone(), inbound_id, weight, properties_obj));
		}

		Ok(Response::Edges(edges))
	}
}

impl Transaction for PostgresTransaction {
	fn request(&mut self, req: Request) {
		self.requests.push(req)
	}

	fn commit(&self) -> Result<Vec<Result<Response, ErrorResponse>>, SimpleError> {
		let conn = try!(self.pool.get());
		let trans = try!(conn.transaction());

		let results = {
			let results: Vec<Result<Response, ErrorResponse>> = self.requests.iter().map(|request| {
				match *request {
					Request::GetVertex(ref id) => self.get_vertex(&trans, *id),
					Request::CreateVertex(ref t, ref properties) => self.create_vertex(&trans, t.clone(), properties.clone()),
					Request::SetVertex(ref v) => self.set_vertex(&trans, v.clone()),
					Request::DeleteVertex(ref id) => self.delete_vertex(&trans, *id),
					Request::GetEdge(ref outbound_id, ref t, ref inbound_id) => self.get_edge(&trans, *outbound_id, t.clone(), *inbound_id),
					Request::SetEdge(ref e) => self.set_edge(&trans, e.clone()),
					Request::DeleteEdge(ref outbound_id, ref t, ref inbound_id) => self.delete_edge(&trans, *outbound_id, t.clone(), *inbound_id),
					Request::GetEdgeCount(ref outbound_id, ref t) => self.get_edge_count(&trans, *outbound_id, t.clone()),
					Request::GetEdgeRange(ref outbound_id, ref t, ref offset, ref limit) => self.get_edge_range(&trans, *outbound_id, t.clone(), *offset, *limit),
					Request::GetEdgeTimeRange(ref outbound_id, ref t, ref high, ref low, ref limit) => self.get_edge_time_range(&trans, *outbound_id, t.clone(), *high, *low, *limit),
				}
			}).collect();

			results
		};

		try!(trans.commit());
		Result::Ok(results)
	}

	fn rollback(&self) -> Option<SimpleError> {
		// Because nothing is actually done until commit(), there's nothing to do to rollback
		None
	}
}
