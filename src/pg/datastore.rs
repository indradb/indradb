use r2d2_postgres::{SslMode, PostgresConnectionManager};
use r2d2::{Config, Pool, GetTimeout, PooledConnection};
use std::mem;
use datastore::{Datastore, Transaction};
use models;
use errors::Error;
use util::{generate_random_secret, get_salted_hash};
use postgres;
use postgres::rows::Rows;
use chrono::naive::datetime::NaiveDateTime;
use serde_json::Value as JsonValue;
use num_cpus;
use uuid::Uuid;
use std::i64;
use postgres::error as pg_error;
use super::util::*;

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
}

impl Datastore<PostgresTransaction, Uuid> for PostgresDatastore {
	fn has_account(&self, account_id: Uuid) -> Result<bool, Error> {
		let conn = try!(self.pool.get());
		let results = try!(conn.query("SELECT 1 FROM accounts WHERE id=$1", &[&account_id]));

		for _ in &results {
			return Result::Ok(true);
		}

		Result::Ok(false)
	}

	fn create_account(&self, email: String) -> Result<(Uuid, String), Error> {
		let id = Uuid::new_v4();
		let salt = generate_random_secret();
		let secret = generate_random_secret();
		let hash = get_salted_hash(salt.clone(), Some(self.secret.clone()), secret.clone());
		let conn = try!(self.pool.get());
		try!(conn.execute("INSERT INTO accounts(id, email, salt, api_secret_hash) VALUES ($1, $2, $3, $4)", &[&id, &email, &salt, &hash]));
		Ok((id, secret))
	}

	fn delete_account(&self, account_id: Uuid) -> Result<(), Error> {
		let conn = try!(self.pool.get());
		let results = try!(conn.query("DELETE FROM accounts WHERE id=$1 RETURNING 1", &[&account_id]));

		for _ in &results {
			return Result::Ok(());
		}

		Err(Error::AccountNotFound)
	}

	fn auth(&self, account_id: Uuid, secret: String) -> Result<bool, Error> {
		let conn = try!(self.pool.get());
		let get_salt_results = try!(conn.query("SELECT salt FROM accounts WHERE id=$1", &[&account_id]));

		for row in &get_salt_results {
			let salt = row.get(0);
			let expected_hash = get_salted_hash(salt, Some(self.secret.clone()), secret);
			let auth_results = try!(conn.query("SELECT 1 FROM accounts WHERE id=$1 AND api_secret_hash=$2", &[&account_id, &expected_hash]));

			for _ in &auth_results {
				return Result::Ok(true);
			}

			return Result::Ok(false);
		}

		Result::Ok(false)
	}

	fn transaction(&self, account_id: Uuid) -> Result<PostgresTransaction, Error> {
		let conn = try!(self.pool.get());
		let trans = try!(PostgresTransaction::new(conn, account_id));
		Ok(trans)
	}
}

#[derive(Debug)]
pub struct PostgresTransaction {
	account_id: Uuid,
	trans: postgres::Transaction<'static>,
	conn: Box<PooledConnection<PostgresConnectionManager>>,
}

impl PostgresTransaction {
	fn new(conn: PooledConnection<PostgresConnectionManager>, account_id: Uuid) -> Result<Self, Error> {
		let conn = Box::new(conn);
		let trans = unsafe { mem::transmute(try!(conn.transaction())) };

		Ok(PostgresTransaction {
			account_id: account_id,
			conn: conn,
			trans: trans,
		})
	}

	fn fill_edges(&self, results: Rows, outbound_id: Uuid, t: String) -> Result<Vec<models::Edge<Uuid>>, Error> {
		let mut edges: Vec<models::Edge<Uuid>> = Vec::new();

		for row in &results {
			let inbound_id: Uuid = row.get(0);
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

		Err(Error::MetadataNotFound)
	}

	fn handle_update_metadata_results(&self, results: Rows) -> Result<(), Error> {
		for _ in &results {
			return Ok(());
		}

		Err(Error::MetadataNotFound)
	}
}

impl Transaction<Uuid> for PostgresTransaction {
	fn get_vertex(&self, id: Uuid) -> Result<models::Vertex<Uuid>, Error> {
		let results = try!(self.trans.query("SELECT type FROM vertices WHERE id=$1 LIMIT 1", &[&id]));

		for row in &results {
			let t: String = row.get(0);
			let v = models::Vertex::new(id, t);
			return Ok(v)
		}

		Err(Error::VertexNotFound)
	}

	fn create_vertex(&self, t: String) -> Result<Uuid, Error> {
		let id = Uuid::new_v4();

		try!(self.trans.execute("
			INSERT INTO vertices (id, type, owner_id) VALUES ($1, $2, $3)
		", &[&id, &t, &self.account_id]));

		Ok(id)
	}

	fn set_vertex(&self, v: models::Vertex<Uuid>) -> Result<(), Error> {
		let results = try!(self.trans.query("
			UPDATE vertices
			SET type=$1
			WHERE id=$2 AND owner_id=$3
			RETURNING 1
		", &[&v.t, &v.id, &self.account_id]));

		for _ in &results {
			return Ok(())
		}

		Err(Error::VertexNotFound)
	}

	fn delete_vertex(&self, id: Uuid) -> Result<(), Error> {
		let results = try!(self.trans.query("DELETE FROM vertices WHERE id=$1 AND owner_id=$2 RETURNING 1", &[&id, &self.account_id]));

		for _ in &results {
			return Ok(())
		}

		Err(Error::VertexNotFound)
	}

	fn get_edge(&self, outbound_id: Uuid, t: String, inbound_id: Uuid) -> Result<models::Edge<Uuid>, Error> {
		let results = try!(self.trans.query("
			SELECT weight FROM edges WHERE outbound_id=$1 AND type=$2 AND inbound_id=$3 LIMIT 1
		", &[&outbound_id, &t, &inbound_id]));

		for row in &results {
			let weight: f32 = row.get(0);
			let e = models::Edge::new(outbound_id, t, inbound_id, weight);
			return Ok(e)
		}

		Err(Error::EdgeNotFound)
	}

	fn set_edge(&self, e: models::Edge<Uuid>) -> Result<(), Error> {
		if e.weight < -1.0 || e.weight > 1.0 {
			return Err(Error::OutOfRange("weight".to_string()));
		}

		let id = Uuid::new_v4();

		// Because this command could fail, we need to set a savepoint to roll
		// back to, rather than spoiling the entire transaction
		let trans = try!(self.trans.savepoint("set_edge"));

		let results = trans.query("
			INSERT INTO edges (id, outbound_id, type, inbound_id, weight, update_date)
			VALUES ($1, (SELECT id FROM vertices WHERE id=$2 AND owner_id=$3), $4, $5, $6, NOW())
			ON CONFLICT ON CONSTRAINT edges_outbound_id_type_inbound_id_ukey DO UPDATE SET weight=$6, update_date=NOW()
			RETURNING 1
		", &[&id, &e.outbound_id, &self.account_id, &e.t, &e.inbound_id, &e.weight]);

		let returnable = match results {
			Ok(results) => {
				if results.len() > 0 {
					Ok(())
				} else {
					Err(Error::VertexNotFound)
				}
			},
			Err(pg_error::Error::Db(ref db_err)) => {
				match db_err.code {
					// This should only happen when the inner select fails
					pg_error::SqlState::NotNullViolation => Err(Error::VertexNotFound),

					// This should only happen when there is no vertex with id=inbound_id
					pg_error::SqlState::ForeignKeyViolation => Err(Error::VertexNotFound),

					// Other db error
					_ => Err(Error::Unexpected(format!("Unknown database error: {}", db_err.message.clone())))
				}
			},
			Err(pg_error::Error::Io(_)) => {
				Err(Error::Unexpected("Database I/O error".to_string()))
			},
			Err(pg_error::Error::Conversion(err)) => panic!(err)
		};

		if returnable.is_err() {
			trans.set_rollback();
		} else {
			trans.set_commit();
		}

		returnable
	}

	fn delete_edge(&self, outbound_id: Uuid, t: String, inbound_id: Uuid) -> Result<(), Error> {
		let results = try!(self.trans.query("
			DELETE FROM EDGES
			WHERE outbound_id=(SELECT id FROM vertices WHERE id=$1 AND owner_id=$2) AND type=$3 AND inbound_id=$4
			RETURNING 1
		", &[&outbound_id, &self.account_id, &t, &inbound_id]));

		for _ in &results {
			return Ok(())
		}

		Err(Error::EdgeNotFound)
	}

	fn get_edge_count(&self, outbound_id: Uuid, t: String) -> Result<u64, Error> {
		let results = try!(self.trans.query("
			SELECT COUNT(outbound_id) FROM edges WHERE outbound_id=$1 AND type=$2
		", &[&outbound_id, &t]));

		for row in &results {
			let count: i64 = row.get(0);
			return Ok(count as u64)
		}

		panic!("Unreachable point hit")
	}

	fn get_edge_range(&self, outbound_id: Uuid, t: String, offset: u64, limit: u16) -> Result<Vec<models::Edge<Uuid>>, Error> {
		if offset > i64::MAX as u64 {
			return Err(Error::OutOfRange("offset".to_string()));
		}
		
		let results = try!(self.trans.query("
			SELECT inbound_id, weight
			FROM edges
			WHERE outbound_id=$1 AND type=$2
			ORDER BY update_date DESC
			OFFSET $3
			LIMIT $4
		", &[&outbound_id, &t, &(offset as i64), &(limit as i64)]));

		self.fill_edges(results, outbound_id, t)
	}

	fn get_edge_time_range(&self, outbound_id: Uuid, t: String, high: Option<NaiveDateTime>, low: Option<NaiveDateTime>, limit: u16) -> Result<Vec<models::Edge<Uuid>>, Error> {
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
			ON CONFLICT ON CONSTRAINT global_metadata_pkey
			DO UPDATE SET value=$2
			RETURNING 1
		", &[&key, &value]));

		self.handle_update_metadata_results(results)
	}

	fn delete_global_metadata(&self, key: String) -> Result<(), Error> {
		let results = try!(self.trans.query("DELETE FROM global_metadata WHERE key=$1 RETURNING 1", &[&key]));
		self.handle_update_metadata_results(results)
	}

	fn get_account_metadata(&self, owner_id: Uuid, key: String) -> Result<JsonValue, Error> {
		let results = try!(self.trans.query("SELECT value FROM account_metadata WHERE owner_id=$1 AND key=$2", &[&owner_id, &key]));
		self.handle_get_metadata_results(results)
	}

	fn set_account_metadata(&self, owner_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
		let results = try!(self.trans.query("
			INSERT INTO account_metadata (owner_id, key, value)
			VALUES ($1, $2, $3)
			ON CONFLICT ON CONSTRAINT account_metadata_pkey
			DO UPDATE SET value=$3
			RETURNING 1
		", &[&owner_id, &key, &value]));

		self.handle_update_metadata_results(results)
	}

	fn delete_account_metadata(&self, owner_id: Uuid, key: String) -> Result<(), Error> {
		let results = try!(self.trans.query("DELETE FROM account_metadata WHERE owner_id=$1 AND key=$2 RETURNING 1", &[&owner_id, &key]));
		self.handle_update_metadata_results(results)
	}

	fn get_vertex_metadata(&self, owner_id: Uuid, key: String) -> Result<JsonValue, Error> {
		let results = try!(self.trans.query("SELECT value FROM vertex_metadata WHERE owner_id=$1 AND key=$2", &[&owner_id, &key]));
		self.handle_get_metadata_results(results)
	}

	fn set_vertex_metadata(&self, owner_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
		let results = try!(self.trans.query("
			INSERT INTO vertex_metadata (owner_id, key, value)
			VALUES ($1, $2, $3)
			ON CONFLICT ON CONSTRAINT vertex_metadata_pkey
			DO UPDATE SET value=$3
			RETURNING 1
		", &[&owner_id, &key, &value]));

		self.handle_update_metadata_results(results)
	}

	fn delete_vertex_metadata(&self, owner_id: Uuid, key: String) -> Result<(), Error> {
		let results = try!(self.trans.query("DELETE FROM vertex_metadata WHERE owner_id=$1 AND key=$2 RETURNING 1", &[&owner_id, &key]));
		self.handle_update_metadata_results(results)
	}

	fn get_edge_metadata(&self, outbound_id: Uuid, t: String, inbound_id: Uuid, key: String) -> Result<JsonValue, Error> {
		let results = try!(self.trans.query("
			SELECT value
			FROM edge_metadata
			WHERE owner_id=(SELECT id FROM edges WHERE outbound_id=$1 AND type=$2 AND inbound_id=$3) AND key=$4
		", &[&outbound_id, &t, &inbound_id, &key]));

		self.handle_get_metadata_results(results)
	}

	fn set_edge_metadata(&self, outbound_id: Uuid, t: String, inbound_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
		let results = try!(self.trans.query("
			INSERT INTO edge_metadata (owner_id, key, value)
			VALUES ((SELECT id FROM edges WHERE outbound_id=$1 AND type=$2 AND inbound_id=$3), $4, $5)
			ON CONFLICT ON CONSTRAINT edge_metadata_pkey
			DO UPDATE SET value=$5
			RETURNING 1
		", &[&outbound_id, &t, &inbound_id, &key, &value]));

		self.handle_update_metadata_results(results)
	}

	fn delete_edge_metadata(&self, outbound_id: Uuid, t: String, inbound_id: Uuid, key: String) -> Result<(), Error> {
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
