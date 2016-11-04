use pg::r2d2_postgres::{SslMode, PostgresConnectionManager};
use pg::r2d2::{Config, Pool, GetTimeout, PooledConnection};
use std::mem;
use datastore::{Datastore, Transaction};
use traits::Id;
use models;
use util::{Error, generate_random_secret};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use pg::postgres;
use pg::postgres::rows::Rows;
use pg::postgres::error as pg_error;
use chrono::naive::datetime::NaiveDateTime;
use serde_json::Value as JsonValue;
use pg::num_cpus;
use std::i32;

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
	fn has_account(&self, account_id: i64) -> Result<bool, Error> {
		if account_id > i32::MAX as i64 as i64 {
			return Result::Err(Error::Unexpected("`account_id` too large".to_string()));
		}

		let conn = try!(self.pool.get());
		let results = try!(conn.query("SELECT 1 FROM accounts WHERE id=$1", &[&(account_id as i32)]));

		for _ in &results {
			return Result::Ok(true);
		}

		Result::Ok(false)
	}

	fn create_account(&self, email: String) -> Result<(i64, String), Error> {
		let salt = generate_random_secret();
		let secret = generate_random_secret();
		let hash = self.get_salted_hash(salt.clone(), secret.clone());
		let conn = try!(self.pool.get());
		let results = try!(conn.query("INSERT INTO accounts(email, salt, api_secret_hash) VALUES ($1, $2, $3) RETURNING id", &[&email, &salt, &hash]));

		for row in &results {
			let id: i32 = row.get(0);
			return Result::Ok((id as i64, secret));
		}

		panic!("Hit unreachable code");
	}

	fn delete_account(&self, account_id: i64) -> Result<(), Error> {
		if account_id > i32::MAX as i64 {
			return Result::Err(Error::Unexpected("`account_id` too large".to_string()));
		}

		let conn = try!(self.pool.get());
		let results = try!(conn.query("DELETE FROM accounts WHERE id=$1 RETURNING 1", &[&(account_id as i32)]));

		for _ in &results {
			return Result::Ok(());
		}

		Err(Error::AccountNotFound)
	}

	fn auth(&self, account_id: i64, secret: String) -> Result<bool, Error> {
		if account_id > i32::MAX as i64 {
			return Result::Err(Error::Unexpected("`account_id` too large".to_string()));
		}

		let conn = try!(self.pool.get());
		let get_salt_results = try!(conn.query("SELECT salt FROM accounts WHERE id=$1", &[&(account_id as i32)]));

		for row in &get_salt_results {
			let salt = row.get(0);
			let expected_hash = self.get_salted_hash(salt, secret);
			let auth_results = try!(conn.query("SELECT 1 FROM accounts WHERE id=$1 AND api_secret_hash=$2", &[&(account_id as i32), &expected_hash]));

			for _ in &auth_results {
				return Result::Ok(true);
			}

			return Result::Ok(false);
		}

		Result::Ok(false)
	}

	fn transaction(&self, account_id: i64) -> Result<PostgresTransaction, Error> {
		if account_id > i32::MAX as i64 {
			return Result::Err(Error::Unexpected("`account_id` too large".to_string()));
		}

		let conn = try!(self.pool.get());
		let trans = try!(PostgresTransaction::new(conn, account_id as i32));
		Ok(trans)
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

#[derive(Debug)]
pub struct PostgresTransaction {
	account_id: i32,
	trans: postgres::Transaction<'static>,
	conn: Box<PooledConnection<PostgresConnectionManager>>,
}

impl PostgresTransaction {
	fn new(conn: PooledConnection<PostgresConnectionManager>, account_id: i32) -> Result<Self, Error> {
		let conn = Box::new(conn);
		let trans = unsafe { mem::transmute(try!(conn.transaction())) };

		Ok(PostgresTransaction {
			account_id: account_id,
			conn: conn,
			trans: trans,
		})
	}

	fn fill_edges(&self, results: Rows, outbound_id: i64, t: String) -> Result<Vec<models::Edge<i64>>, Error> {
		let mut edges: Vec<models::Edge<i64>> = Vec::new();

		for row in &results {
			let inbound_id: i64 = row.get(0);
			let weight: f32 = row.get(1);
			edges.push(models::Edge::new(outbound_id, t.clone(), inbound_id, weight));
		}

		Ok(edges)
	}

	fn handle_get_metadata_results(&self, results: Rows) -> Result<JsonValue, Error> {
		for row in &results {
			let value: JsonValue = row.get(0);
			return Ok(value)
		}

		Err(Error::MetadataDoesNotExist)
	}

	fn handle_update_metadata_results(&self, results: Rows) -> Result<(), Error> {
		for _ in &results {
			return Ok(());
		}

		Err(Error::MetadataDoesNotExist)
	}
}

impl Transaction<i64> for PostgresTransaction {
	fn get_vertex(&self, id: i64) -> Result<models::Vertex<i64>, Error> {
		let results = try!(self.trans.query("SELECT type FROM vertices WHERE id=$1 LIMIT 1", &[&id]));

		for row in &results {
			let t: String = row.get(0);
			let v = models::Vertex::new(id, t);
			return Ok(v)
		}

		Err(Error::VertexDoesNotExist)
	}

	fn create_vertex(&self, t: String) -> Result<i64, Error> {
		let results = try!(self.trans.query("
			INSERT INTO vertices (type, owner_id) VALUES ($1, $2) RETURNING id
		", &[&t, &self.account_id]));

		for row in &results {
			let id: i64 = row.get(0);
			return Ok(id)
		}

		panic!("Unreachable point hit")
	}

	fn set_vertex(&self, v: models::Vertex<i64>) -> Result<(), Error> {
		let results = try!(self.trans.query("
			UPDATE vertices
			SET type=$1
			WHERE id=$2 AND owner_id=$3
			RETURNING 1
		", &[&v.t, &v.id, &self.account_id]));

		for _ in &results {
			return Ok(())
		}

		Err(Error::VertexDoesNotExist)
	}

	fn delete_vertex(&self, id: i64) -> Result<(), Error> {
		let results = try!(self.trans.query("DELETE FROM vertices WHERE id=$1 AND owner_id=$2 RETURNING 1", &[&id, &self.account_id]));

		for _ in &results {
			return Ok(())
		}

		Err(Error::VertexDoesNotExist)
	}

	fn get_edge(&self, outbound_id: i64, t: String, inbound_id: i64) -> Result<models::Edge<i64>, Error> {
		let results = try!(self.trans.query("
			SELECT weight FROM edges WHERE outbound_id=$1 AND type=$2 AND inbound_id=$3 LIMIT 1
		", &[&outbound_id, &t, &inbound_id]));

		for row in &results {
			let weight: f32 = row.get(0);
			let e = models::Edge::new(outbound_id, t, inbound_id, weight);
			return Ok(e)
		}

		Err(Error::EdgeDoesNotExist)
	}

	fn set_edge(&self, e: models::Edge<i64>) -> Result<(), Error> {
		if e.weight < -1.0 || e.weight > 1.0 {
			return Err(Error::WeightOutOfRange);
		}

		let results = self.trans.query("
			INSERT INTO edges (outbound_id, type, inbound_id, weight, update_date)
			VALUES ((SELECT id FROM vertices WHERE id=$1 AND owner_id=$2), $3, $4, $5, NOW())
			ON CONFLICT ON CONSTRAINT edges_outbound_id_type_inbound_id_ukey DO UPDATE SET weight=$5, update_date=NOW()
			RETURNING 1
		", &[&e.outbound_id, &self.account_id, &e.t, &e.inbound_id, &e.weight]);

		match results {
			Ok(results) => {
				for _ in &results {
					return Ok(());
				}
			},
			Err(err) => {
				return match err {
					pg_error::Error::Db(ref db_err) => {
						match db_err.code {
							// This should only happen when the inner select fails
							pg_error::SqlState::NotNullViolation => Err(Error::VertexDoesNotExist),

							// This should only happen when there is no vertex with id=inbound_id
							pg_error::SqlState::ForeignKeyViolation => Err(Error::VertexDoesNotExist),

							// Other db error
							_ => Err(Error::Unexpected(format!("Unknown database error: {}", db_err.message.clone())))
						}
					},
					pg_error::Error::Io(_) => Err(Error::Unexpected("Database I/O error".to_string())),
					pg_error::Error::Conversion(err) => panic!(err)
				};
			}
		}

		Err(Error::VertexDoesNotExist)
	}

	fn delete_edge(&self, outbound_id: i64, t: String, inbound_id: i64) -> Result<(), Error> {
		let results = try!(self.trans.query("
			DELETE FROM EDGES
			WHERE outbound_id=(SELECT id FROM vertices WHERE id=$1 AND owner_id=$2) AND type=$3 AND inbound_id=$4
			RETURNING 1
		", &[&outbound_id, &self.account_id, &t, &inbound_id]));

		for _ in &results {
			return Ok(())
		}

		Err(Error::EdgeDoesNotExist)
	}

	fn get_edge_count(&self, outbound_id: i64, t: String) -> Result<i64, Error> {
		let results = try!(self.trans.query("
			SELECT COUNT(outbound_id) FROM edges WHERE outbound_id=$1 AND type=$2
		", &[&outbound_id, &t]));

		for row in &results {
			let count: i64 = row.get(0);
			return Ok(count)
		}

		panic!("Unreachable point hit")
	}

	fn get_edge_range(&self, outbound_id: i64, t: String, offset: i64, limit: i32) -> Result<Vec<models::Edge<i64>>, Error> {
		if offset < 0 {
			return Err(Error::OffsetOutOfRange);
		}

		if limit < 0 {
			return Err(Error::LimitOutOfRange);
		}

		let results = try!(self.trans.query("
			SELECT inbound_id, weight
			FROM edges
			WHERE outbound_id=$1 AND type=$2
			ORDER BY update_date DESC
			OFFSET $3
			LIMIT $4
		", &[&outbound_id, &t, &offset, &(limit as i64)]));

		self.fill_edges(results, outbound_id, t)
	}

	fn get_edge_time_range(&self, outbound_id: i64, t: String, high: Option<NaiveDateTime>, low: Option<NaiveDateTime>, limit: i32) -> Result<Vec<models::Edge<i64>>, Error> {
		if limit < 0 {
			return Err(Error::LimitOutOfRange);
		}

		let results = try!(match (high, low) {
			(Option::Some(high_unboxed), Option::Some(low_unboxed)) => {
				self.trans.query("
					SELECT inbound_id, weight
					FROM edges
					WHERE outbound_id=$1 AND type=$2 AND update_date <= $3 AND update_date >= $4
					ORDER BY update_date DESC
					LIMIT $5
				", &[&outbound_id, &t, &high_unboxed, &low_unboxed, &(limit as i64)])
			},
			(Option::Some(high_unboxed), Option::None) => {
				self.trans.query("
					SELECT inbound_id, weight
					FROM edges
					WHERE outbound_id=$1 AND type=$2 AND update_date <= $3
					ORDER BY update_date DESC
					LIMIT $4
				", &[&outbound_id, &t, &high_unboxed, &(limit as i64)])
			},
			(Option::None, Option::Some(low_unboxed)) => {
				self.trans.query("
					SELECT inbound_id, weight
					FROM edges
					WHERE outbound_id=$1 AND type=$2 AND update_date >= $3
					ORDER BY update_date DESC
					LIMIT $4
				", &[&outbound_id, &t, &low_unboxed, &(limit as i64)])
			},
			_ => {
				self.trans.query("
					SELECT inbound_id, weight
					FROM edges
					WHERE outbound_id=$1 AND type=$2
					ORDER BY update_date DESC
					LIMIT $3
				", &[&outbound_id, &t, &(limit as i64)])
			}
		});

		self.fill_edges(results, outbound_id, t)
	}

	fn get_global_metadata(&self, key: String) -> Result<JsonValue, Error> {
		let results = try!(self.trans.query("SELECT value FROM global_metadata WHERE key=$1", &[&key]));
		self.handle_get_metadata_results(results)
	}

	fn set_global_metadata(&self, key: String, value: JsonValue) -> Result<(), Error> {
		let results = try!(self.trans.query("
			INSERT INTO global_metadata (key, value)
			VALUES ($1, $2)
			ON CONFLICT ON CONSTRAINT global_metadata_key_ukey
			DO UPDATE SET value=$2
			RETURNING 1
		", &[&key, &value]));

		self.handle_update_metadata_results(results)
	}

	fn delete_global_metadata(&self, key: String) -> Result<(), Error> {
		let results = try!(self.trans.query("DELETE FROM global_metadata WHERE key=$1 RETURNING 1", &[&key]));
		self.handle_update_metadata_results(results)
	}

	fn get_account_metadata(&self, owner_id: i64, key: String) -> Result<JsonValue, Error> {
		if owner_id > i32::MAX as i64 {
			return Result::Err(Error::Unexpected("`owner_id` too large".to_string()));
		}

		let results = try!(self.trans.query("SELECT value FROM account_metadata WHERE owner_id=$1 AND key=$2", &[&(owner_id as i32), &key]));
		self.handle_get_metadata_results(results)
	}

	fn set_account_metadata(&self, owner_id: i64, key: String, value: JsonValue) -> Result<(), Error> {
		if owner_id > i32::MAX as i64 {
			return Result::Err(Error::Unexpected("`owner_id` too large".to_string()));
		}

		let results = try!(self.trans.query("
			INSERT INTO account_metadata (owner_id, key, value)
			VALUES ($1, $2, $3)
			ON CONFLICT ON CONSTRAINT account_metadata_owner_id_key_ukey
			DO UPDATE SET value=$3
			RETURNING 1
		", &[&(owner_id as i32), &key, &value]));

		self.handle_update_metadata_results(results)
	}

	fn delete_account_metadata(&self, owner_id: i64, key: String) -> Result<(), Error> {
		if owner_id > i32::MAX as i64 {
			return Result::Err(Error::Unexpected("`owner_id` too large".to_string()));
		}

		let results = try!(self.trans.query("DELETE FROM account_metadata WHERE owner_id=$1 AND key=$2 RETURNING 1", &[&(owner_id as i32), &key]));
		self.handle_update_metadata_results(results)
	}

	fn get_vertex_metadata(&self, owner_id: i64, key: String) -> Result<JsonValue, Error> {
		let results = try!(self.trans.query("SELECT value FROM vertex_metadata WHERE owner_id=$1 AND key=$2", &[&owner_id, &key]));
		self.handle_get_metadata_results(results)
	}

	fn set_vertex_metadata(&self, owner_id: i64, key: String, value: JsonValue) -> Result<(), Error> {
		let results = try!(self.trans.query("
			INSERT INTO vertex_metadata (owner_id, key, value)
			VALUES ($1, $2, $3)
			ON CONFLICT ON CONSTRAINT vertex_metadata_owner_id_key_ukey
			DO UPDATE SET value=$3
			RETURNING 1
		", &[&owner_id, &key, &value]));

		self.handle_update_metadata_results(results)
	}

	fn delete_vertex_metadata(&self, owner_id: i64, key: String) -> Result<(), Error> {
		let results = try!(self.trans.query("DELETE FROM vertex_metadata WHERE owner_id=$1 AND key=$2 RETURNING 1", &[&owner_id, &key]));
		self.handle_update_metadata_results(results)
	}

	fn get_edge_metadata(&self, outbound_id: i64, t: String, inbound_id: i64, key: String) -> Result<JsonValue, Error> {
		let results = try!(self.trans.query("
			SELECT value
			FROM edge_metadata
			WHERE owner_id=(SELECT id FROM edges WHERE outbound_id=$1 AND type=$2 AND inbound_id=$3) AND key=$4
		", &[&outbound_id, &t, &inbound_id, &key]));

		self.handle_get_metadata_results(results)
	}

	fn set_edge_metadata(&self, outbound_id: i64, t: String, inbound_id: i64, key: String, value: JsonValue) -> Result<(), Error> {
		let results = try!(self.trans.query("
			INSERT INTO edge_metadata (owner_id, key, value)
			VALUES ((SELECT id FROM edges WHERE outbound_id=$1 AND type=$2 AND inbound_id=$3), $4, $5)
			ON CONFLICT ON CONSTRAINT edge_metadata_owner_id_key_ukey
			DO UPDATE SET value=$5
			RETURNING 1
		", &[&outbound_id, &t, &inbound_id, &key, &value]));

		self.handle_update_metadata_results(results)
	}

	fn delete_edge_metadata(&self, outbound_id: i64, t: String, inbound_id: i64, key: String) -> Result<(), Error> {
		let results = try!(self.trans.query("
			DELETE FROM edge_metadata
			WHERE owner_id=(SELECT id FROM edges WHERE outbound_id=$1 AND type=$2 AND inbound_id=$3) AND key=$4
			RETURNING 1
		", &[&outbound_id, &t, &inbound_id, &key]));

		self.handle_update_metadata_results(results)
	}

	fn commit(self) -> Result<(), Error> {
		self.trans.set_commit();
		try!(self.trans.commit());
		Ok(())
	}

	fn rollback(self) -> Result<(), Error> {
		self.trans.set_rollback();
		try!(self.trans.commit());
		Ok(())
	}
}
