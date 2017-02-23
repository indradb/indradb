use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::{Config, Pool, PooledConnection};
use std::mem;
use datastore::{Datastore, Transaction};
use models;
use errors::Error;
use util::{generate_random_secret, get_salted_hash};
use postgres;
use postgres::rows::Rows;
use chrono::{UTC, DateTime};
use serde_json::Value as JsonValue;
use num_cpus;
use uuid::Uuid;
use std::i64;
use postgres::error as pg_error;

/// A datastore that is backed by a postgres database.
#[derive(Clone, Debug)]
pub struct PostgresDatastore {
    /// A database connection pool.
    pool: Pool<PostgresConnectionManager>,

    /// A secret value, used as the pepper in hashing sensitive account data.
    secret: String,
}

impl PostgresDatastore {
    /// Creates a new postgres-backed datastore.
    ///
    /// # Arguments
    /// * `pool_size` - The maximum number of connections to maintain to
    ///   postgres. If `None`, it defaults to twice the number of CPUs.
    /// * `connetion_string` - The postgres database connection string.
    /// * `secret` - A secret value. This is used as a pepper in hashing
    ///   sensitive account data. 
    pub fn new(pool_size: Option<u32>, connection_string: String, secret: String) -> PostgresDatastore {
        let unwrapped_pool_size: u32 = match pool_size {
            Some(val) => val,
            None => {
                let cpus: usize = num_cpus::get();
                if cpus > 512 { 1024 } else { cpus as u32 * 2 }
            }
        };

        let pool_config = Config::builder().pool_size(unwrapped_pool_size).build();
        let manager = match PostgresConnectionManager::new(&*connection_string, TlsMode::None) {
            Ok(manager) => manager,
            Err(err) => panic!("Could not connect to the postgres database: {}", err),
        };

        PostgresDatastore {
            pool: match Pool::new(pool_config, manager) {
                Ok(pool) => pool,
                Err(err) => panic!("Could not initialize postgres database pool: {}", err),
            },
            secret: secret,
        }
    }
}

impl Datastore<PostgresTransaction, Uuid> for PostgresDatastore {
    fn has_account(&self, account_id: Uuid) -> Result<bool, Error> {
        let conn = self.pool.get()?;

        let results = conn.query("SELECT 1 FROM accounts WHERE id=$1", &[&account_id])?;

        for _ in &results {
            return Result::Ok(true);
        }

        Result::Ok(false)
    }

    fn create_account(&self, email: String) -> Result<(Uuid, String), Error> {
        let id = Uuid::new_v4();
        let salt = generate_random_secret();
        let secret = generate_random_secret();
        let hash = get_salted_hash(&salt[..], Some(&self.secret[..]), &secret[..]);
        let conn = self.pool.get()?;
        
        conn.execute("
            INSERT INTO accounts(id, email, salt, api_secret_hash)
            VALUES ($1, $2, $3, $4)
            ", &[&id, &email, &salt, &hash]
        )?;

        Ok((id, secret))
    }

    fn delete_account(&self, account_id: Uuid) -> Result<(), Error> {
        let conn = self.pool.get()?;
        
        let results = conn.query("DELETE FROM accounts WHERE id=$1 RETURNING 1", &[&account_id])?;

        for _ in &results {
            return Result::Ok(());
        }

        Err(Error::AccountNotFound)
    }

    fn auth(&self, account_id: Uuid, secret: String) -> Result<bool, Error> {
        let conn = self.pool.get()?;

        let get_salt_results = conn.query("SELECT salt, api_secret_hash FROM accounts WHERE id=$1", &[&account_id])?;

        for row in &get_salt_results {
            let salt: String = row.get(0);
            let expected_hash: String = row.get(1);
            let actual_hash = get_salted_hash(&salt[..], Some(&self.secret[..]), &secret[..]);
            return Ok(expected_hash == actual_hash);
        }

        // Calculate the hash anyways to prevent timing attacks
        get_salted_hash("", Some(&self.secret[..]), &secret[..]);
        Ok(false)
    }

    fn transaction(&self, account_id: Uuid) -> Result<PostgresTransaction, Error> {
        let conn = self.pool.get()?;
        let trans = PostgresTransaction::new(conn, account_id)?;
        Ok(trans)
    }
}

/// A postgres-backed datastore transaction.
#[derive(Debug)]
pub struct PostgresTransaction {
    account_id: Uuid,
    trans: postgres::transaction::Transaction<'static>,
    conn: Box<PooledConnection<PostgresConnectionManager>>,
}

impl PostgresTransaction {
    fn new(conn: PooledConnection<PostgresConnectionManager>, account_id: Uuid) -> Result<Self, Error> {
        let conn = Box::new(conn);

        let trans = unsafe {
            mem::transmute(match conn.transaction() {
                Ok(trans) => trans,
                Err(err) => return Err(Error::Unexpected(format!("Could not create transaction: {}", err)))
            })
        };

        Ok(PostgresTransaction {
            account_id: account_id,
            conn: conn,
            trans: trans,
        })
    }

    fn fill_edges(&self, results: Rows, outbound_id: Uuid) -> Result<Vec<models::Edge<Uuid>>, Error> {
        let mut edges: Vec<models::Edge<Uuid>> = Vec::new();

        for row in &results {
            let t_str: String = row.get(0);
            let t = models::Type::new(t_str).unwrap();
            let inbound_id: Uuid = row.get(1);
            let weight_f32: f32 = row.get(2);
            let weight = models::Weight::new(weight_f32).unwrap();
            let update_datetime: DateTime<UTC> = row.get(3);
            edges.push(models::Edge::new(outbound_id, t, inbound_id, weight, update_datetime));
        }

        Ok(edges)
    }

    fn fill_reversed_edges(&self, results: Rows, inbound_id: Uuid) -> Result<Vec<models::Edge<Uuid>>, Error> {
        let mut edges: Vec<models::Edge<Uuid>> = Vec::new();

        for row in &results {
            let t_str: String = row.get(0);
            let t = models::Type::new(t_str).unwrap();
            let outbound_id: Uuid = row.get(1);
            let weight_f32: f32 = row.get(2);
            let weight = models::Weight::new(weight_f32).unwrap();
            let update_datetime: DateTime<UTC> = row.get(3);
            edges.push(models::Edge::new(outbound_id, t, inbound_id, weight, update_datetime));
        }

        Ok(edges)
    }

    fn handle_get_metadata(&self, results: Rows) -> Result<JsonValue, Error> {
        for row in &results {
            let value: JsonValue = row.get(0);
            return Ok(value);
        }

        Err(Error::MetadataNotFound)
    }

    fn handle_delete_metadata(&self, results: Rows) -> Result<(), Error> {
        for _ in &results {
            return Ok(());
        }

        Err(Error::MetadataNotFound)
    }

    fn handle_set_metadata(&self, result: Result<Rows, pg_error::Error>, foreign_key_error: Error) -> Result<(), Error> {
        match result {
            Ok(rows) => {
                for _ in &rows {
                    return Ok(());
                }

                Err(Error::MetadataNotFound)
            },
            Err(pg_error::Error::Db(ref err)) => {
                if err.code == pg_error::SqlState::ForeignKeyViolation || err.code == pg_error::SqlState::NotNullViolation {
                    // This should only happen when we couldn't get the
                    // "owning" resource for the metadata
                    Err(foreign_key_error)
                } else {
                    try!(result);
                    panic!("Unreacheable code hit")
                }
            },
            _ => {
                try!(result);
                panic!("Unreacheable code hit")
            }
        }
    }
}

impl Transaction<Uuid> for PostgresTransaction {
    fn get_vertex_range(&self, offset: u64, limit: u16) -> Result<Vec<models::Vertex<Uuid>>, Error> {
        if offset > i64::MAX as u64 {
            return Err(Error::Unexpected("Offset out of range".to_string()));
        }

        let results = self.trans.query("
            SELECT id, type FROM vertices
            ORDER BY id
            OFFSET $1
            LIMIT $2
        ", &[&(offset as i64), &(limit as i64)])?;

        let mut vertices: Vec<models::Vertex<Uuid>> = Vec::new();

        for row in &results {
            let id: Uuid = row.get(0);
            let t_str: String = row.get(1);
            let v = models::Vertex::new(id, models::Type::new(t_str).unwrap());
            vertices.push(v);
        }

        Ok(vertices)
    }

    fn get_vertex(&self, id: Uuid) -> Result<models::Vertex<Uuid>, Error> {
        let results = self.trans.query("SELECT type FROM vertices WHERE id=$1 LIMIT 1", &[&id])?;

        for row in &results {
            let t_str: String = row.get(0);
            let v = models::Vertex::new(id, models::Type::new(t_str).unwrap());
            return Ok(v);
        }

        Err(Error::VertexNotFound)
    }

    fn create_vertex(&self, t: models::Type) -> Result<Uuid, Error> {
        let id = Uuid::new_v4();
        self.trans.execute("INSERT INTO vertices (id, type, owner_id) VALUES ($1, $2, $3)", &[&id, &t.0, &self.account_id])?;
        Ok(id)
    }

    fn set_vertex(&self, v: models::Vertex<Uuid>) -> Result<(), Error> {
        let results = self.trans.query("
			UPDATE vertices
			SET type=$1
			WHERE id=$2 AND owner_id=$3
			RETURNING 1
		", &[&v.t.0, &v.id, &self.account_id])?;

        for _ in &results {
            return Ok(());
        }

        Err(Error::VertexNotFound)
    }

    fn delete_vertex(&self, id: Uuid) -> Result<(), Error> {
        let results = self.trans.query("DELETE FROM vertices WHERE id=$1 AND owner_id=$2 RETURNING 1", &[&id, &self.account_id])?;

        for _ in &results {
            return Ok(());
        }

        // We couldn't delete the vertex - it either doesn't exist, or we're
        // unauthorized to delete it. Check if it exists first, and if that
        // doesn't give back a VertexNotFound, we must be unauthorized.
        self.get_vertex(id)?;
        Err(Error::Unauthorized)
    }

    fn get_edge(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid) -> Result<models::Edge<Uuid>, Error> {
        let results = self.trans.query("
            SELECT weight, update_timestamp
            FROM edges
            WHERE outbound_id=$1 AND type=$2 AND inbound_id=$3
            LIMIT 1", &[&outbound_id, &t.0, &inbound_id])?;

        for row in &results {
            let weight_f32: f32 = row.get(0);
            let weight = models::Weight::new(weight_f32).unwrap();
            let update_datetime: DateTime<UTC> = row.get(1);
            let e = models::Edge::new(outbound_id, t, inbound_id, weight, update_datetime);
            return Ok(e);
        }

        Err(Error::EdgeNotFound)
    }

    fn set_edge(&self, e: models::Edge<Uuid>) -> Result<(), Error> {
        let id = Uuid::new_v4();

        // Because this command could fail, we need to set a savepoint to roll
        // back to, rather than spoiling the entire transaction
        let results = {
            let trans = self.trans.savepoint("set_edge")?;

            let results = trans.query("
				INSERT INTO edges (
                    id,
                    outbound_id,
                    type,
                    inbound_id,
                    weight,
                    update_timestamp
                ) VALUES (
                    $1,
                    (SELECT id FROM vertices WHERE id=$2 AND owner_id=$3),
                    $4,
                    $5,
                    $6,
                    CLOCK_TIMESTAMP()
                )
                ON CONFLICT ON CONSTRAINT edges_outbound_id_type_inbound_id_ukey
                DO UPDATE SET weight=$6, update_timestamp=CLOCK_TIMESTAMP()
			", &[&id, &e.outbound_id, &self.account_id, &e.t.0, &e.inbound_id, &e.weight.0]);

            match results {
                Err(err) => {
                    trans.set_rollback();
                    Err(err)
                }
                Ok(_) => {
                    trans.set_commit();
                    Ok(())
                }
            }
        };

        if let Err(pg_error::Error::Db(ref err)) = results {
            if err.code == pg_error::SqlState::NotNullViolation {
                // This should only happen when the inner select fails
                self.get_vertex(e.outbound_id)?;
                return Err(Error::Unauthorized);
            } else if err.code == pg_error::SqlState::ForeignKeyViolation {
                // This should only happen when there is no vertex with id=inbound_id
                return Err(Error::VertexNotFound);
            }
        }

        results?;
        Ok(())
    }

    fn delete_edge(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid) -> Result<(), Error> {
        let results = self.trans.query("
			DELETE FROM edges
			WHERE outbound_id=(
                SELECT id
                FROM vertices
                WHERE id=$1 AND owner_id=$2
            ) AND type=$3 AND inbound_id=$4
			RETURNING 1
		", &[&outbound_id, &self.account_id, &t.0, &inbound_id])?;

        for _ in &results {
            return Ok(());
        }

        self.get_edge(outbound_id, t, inbound_id)?;
        Err(Error::Unauthorized)
    }

    fn get_edge_count(&self, outbound_id: Uuid, t: Option<models::Type>) -> Result<u64, Error> {
        let results = match t {
            Some(t) => {
                self.trans.query("
			        SELECT COUNT(outbound_id) FROM edges WHERE outbound_id=$1 AND type=$2
		        ", &[&outbound_id, &t.0])?
            },
            None => {
                self.trans.query("
                    SELECT COUNT(outbound_id) FROM edges WHERE outbound_id=$1
                ", &[&outbound_id])?
            }
        };

        for row in &results {
            let count: i64 = row.get(0);
            return Ok(count as u64);
        }

        panic!("Unreachable point hit")
    }

    fn get_edge_range(&self, outbound_id: Uuid, t: Option<models::Type>, offset: u64, limit: u16) -> Result<Vec<models::Edge<Uuid>>, Error> {
        if offset > i64::MAX as u64 {
            return Err(Error::Unexpected("Offset out of range".to_string()));
        }

        let results = match t {
            Some(t) => {
                self.trans.query("
                    SELECT type, inbound_id, weight, update_timestamp
                    FROM edges
                    WHERE outbound_id=$1 AND type=$2
                    ORDER BY update_timestamp DESC
                    OFFSET $3
                    LIMIT $4
                ", &[&outbound_id, &t.0, &(offset as i64), &(limit as i64)])?
            },
            None => {
                self.trans.query("
                    SELECT type, inbound_id, weight, update_timestamp
                    FROM edges
                    WHERE outbound_id=$1
                    ORDER BY update_timestamp DESC
                    OFFSET $2
                    LIMIT $3
                ", &[&outbound_id, &(offset as i64), &(limit as i64)])?
            }
        };

        self.fill_edges(results, outbound_id)
    }

    fn get_edge_time_range(&self, outbound_id: Uuid, t: Option<models::Type>, high: Option<DateTime<UTC>>, low: Option<DateTime<UTC>>, limit: u16) -> Result<Vec<models::Edge<Uuid>>, Error> {
        let results = match (t, high, low) {
            (Some(t), Some(high), Some(low)) => {
                self.trans.query("
					SELECT type, inbound_id, weight, update_timestamp
					FROM edges
					WHERE outbound_id=$1 AND type=$2 AND update_timestamp <= $3 AND update_timestamp >= $4
					ORDER BY update_timestamp DESC
					LIMIT $5
				", &[&outbound_id, &t.0, &high, &low, &(limit as i64)])
            }
            (Some(t), Some(high), None) => {
                self.trans.query("
					SELECT type, inbound_id, weight, update_timestamp
					FROM edges
					WHERE outbound_id=$1 AND type=$2 AND update_timestamp <= $3
					ORDER BY update_timestamp DESC
					LIMIT $4
				", &[&outbound_id, &t.0, &high, &(limit as i64)])
            }
            (Some(t), None, Some(low)) => {
                self.trans.query("
					SELECT type, inbound_id, weight, update_timestamp
					FROM edges
					WHERE outbound_id=$1 AND type=$2 AND update_timestamp >= $3
					ORDER BY update_timestamp DESC
					LIMIT $4
				", &[&outbound_id, &t.0, &low, &(limit as i64)])
            }
            (Some(t), None, None) => {
                self.trans.query("
					SELECT type, inbound_id, weight, update_timestamp
					FROM edges
					WHERE outbound_id=$1 AND type=$2
					ORDER BY update_timestamp DESC
					LIMIT $3
				", &[&outbound_id, &t.0, &(limit as i64)])
            },
            (None, Some(high), Some(low)) => {
                self.trans.query("
					SELECT type, inbound_id, weight, update_timestamp
					FROM edges
					WHERE outbound_id=$1 AND update_timestamp <= $2 AND update_timestamp >= $3
					ORDER BY update_timestamp DESC
					LIMIT $4
				", &[&outbound_id, &high, &low, &(limit as i64)])
            }
            (None, Some(high), None) => {
                self.trans.query("
					SELECT type, inbound_id, weight, update_timestamp
					FROM edges
					WHERE outbound_id=$1 AND update_timestamp <= $2
					ORDER BY update_timestamp DESC
					LIMIT $3
				", &[&outbound_id, &high, &(limit as i64)])
            }
            (None, None, Some(low)) => {
                self.trans.query("
					SELECT type, inbound_id, weight, update_timestamp
					FROM edges
					WHERE outbound_id=$1 AND update_timestamp >= $2
					ORDER BY update_timestamp DESC
					LIMIT $3
				", &[&outbound_id, &low, &(limit as i64)])
            }
            (None, None, None) => {
                self.trans.query("
					SELECT type, inbound_id, weight, update_timestamp
					FROM edges
					WHERE outbound_id=$1
					ORDER BY update_timestamp DESC
					LIMIT $2
				", &[&outbound_id, &(limit as i64)])
            }
        }?;

        self.fill_edges(results, outbound_id)
    }

    fn get_reversed_edge_count(&self, inbound_id: Uuid, t: Option<models::Type>) -> Result<u64, Error> {
        let results = match t {
            Some(t) => {
                self.trans.query("
                    SELECT COUNT(inbound_id) FROM edges WHERE inbound_id=$1 AND type=$2
                ", &[&inbound_id, &t.0])?
            },
            None => {
                self.trans.query("
                    SELECT COUNT(inbound_id) FROM edges WHERE inbound_id=$1
                ", &[&inbound_id])?
            }
        };

        for row in &results {
            let count: i64 = row.get(0);
            return Ok(count as u64);
        }

        panic!("Unreachable point hit")
    }

    fn get_reversed_edge_range(&self, inbound_id: Uuid, t: Option<models::Type>, offset: u64, limit: u16) -> Result<Vec<models::Edge<Uuid>>, Error> {
        if offset > i64::MAX as u64 {
            return Err(Error::Unexpected("Offset out of range".to_string()));
        }

        let results = match t {
            Some(t) => {
                self.trans.query("
                    SELECT type, outbound_id, weight, update_timestamp
                    FROM edges
                    WHERE inbound_id=$1 AND type=$2
                    ORDER BY update_timestamp DESC
                    OFFSET $3
                    LIMIT $4
                ", &[&inbound_id, &t.0, &(offset as i64), &(limit as i64)])?
            },
            None => {
                self.trans.query("
                    SELECT type, outbound_id, weight, update_timestamp
                    FROM edges
                    WHERE inbound_id=$1
                    ORDER BY update_timestamp DESC
                    OFFSET $2
                    LIMIT $3
                ", &[&inbound_id, &(offset as i64), &(limit as i64)])?
            }
        };

        self.fill_reversed_edges(results, inbound_id)
    }

    fn get_reversed_edge_time_range(&self, inbound_id: Uuid, t: Option<models::Type>, high: Option<DateTime<UTC>>, low: Option<DateTime<UTC>>, limit: u16) -> Result<Vec<models::Edge<Uuid>>, Error> {
        let results = match (t, high, low) {
            (Some(t), Some(high), Some(low)) => {
                self.trans.query("
					SELECT type, outbound_id, weight, update_timestamp
					FROM edges
					WHERE inbound_id=$1 AND type=$2 AND update_timestamp <= $3 AND update_timestamp >= $4
					ORDER BY update_timestamp DESC
					LIMIT $5
				", &[&inbound_id, &t.0, &high, &low, &(limit as i64)])
            }
            (Some(t), Some(high), None) => {
                self.trans.query("
					SELECT type, outbound_id, weight, update_timestamp
					FROM edges
					WHERE inbound_id=$1 AND type=$2 AND update_timestamp <= $3
					ORDER BY update_timestamp DESC
					LIMIT $4
				", &[&inbound_id, &t.0, &high, &(limit as i64)])
            }
            (Some(t), None, Some(low)) => {
                self.trans.query("
					SELECT type, outbound_id, weight, update_timestamp
					FROM edges
					WHERE inbound_id=$1 AND type=$2 AND update_timestamp >= $3
					ORDER BY update_timestamp DESC
					LIMIT $4
				", &[&inbound_id, &t.0, &low, &(limit as i64)])
            }
            (Some(t), None, None) => {
                self.trans.query("
					SELECT type, outbound_id, weight, update_timestamp
					FROM edges
					WHERE inbound_id=$1 AND type=$2
					ORDER BY update_timestamp DESC
					LIMIT $3
				", &[&inbound_id, &t.0, &(limit as i64)])
            },
            (None, Some(high), Some(low)) => {
                self.trans.query("
					SELECT type, outbound_id, weight, update_timestamp
					FROM edges
					WHERE inbound_id=$1 AND update_timestamp <= $2 AND update_timestamp >= $3
					ORDER BY update_timestamp DESC
					LIMIT $4
				", &[&inbound_id, &high, &low, &(limit as i64)])
            }
            (None, Some(high), None) => {
                self.trans.query("
					SELECT type, outbound_id, weight, update_timestamp
					FROM edges
					WHERE inbound_id=$1 AND update_timestamp <= $2
					ORDER BY update_timestamp DESC
					LIMIT $3
				", &[&inbound_id, &high, &(limit as i64)])
            }
            (None, None, Some(low)) => {
                self.trans.query("
					SELECT type, outbound_id, weight, update_timestamp
					FROM edges
					WHERE inbound_id=$1 AND update_timestamp >= $2
					ORDER BY update_timestamp DESC
					LIMIT $3
				", &[&inbound_id, &low, &(limit as i64)])
            }
            (None, None, None) => {
                self.trans.query("
					SELECT type, outbound_id, weight, update_timestamp
					FROM edges
					WHERE inbound_id=$1
					ORDER BY update_timestamp DESC
					LIMIT $2
				", &[&inbound_id, &(limit as i64)])
            }
        }?;

        self.fill_reversed_edges(results, inbound_id)
    }

    fn get_global_metadata(&self, key: String) -> Result<JsonValue, Error> {
        let results = self.trans.query("SELECT value FROM global_metadata WHERE key=$1", &[&key])?;
        self.handle_get_metadata(results)
    }

    fn set_global_metadata(&self, key: String, value: JsonValue) -> Result<(), Error> {
        let results = self.trans.query("
			INSERT INTO global_metadata (key, value)
			VALUES ($1, $2)
			ON CONFLICT ON CONSTRAINT global_metadata_pkey
			DO UPDATE SET value=$2
			RETURNING 1
		", &[&key, &value]);

        self.handle_set_metadata(results, Error::Unexpected("Unexpected error when setting global metadata".to_string()))
    }

    fn delete_global_metadata(&self, key: String) -> Result<(), Error> {
        let results = self.trans.query(
            "DELETE FROM global_metadata WHERE key=$1 RETURNING 1",
            &[&key]
        )?;

        self.handle_delete_metadata(results)
    }

    fn get_account_metadata(&self, owner_id: Uuid, key: String) -> Result<JsonValue, Error> {
        let results = self.trans.query(
            "SELECT value FROM account_metadata WHERE owner_id=$1 AND key=$2",
            &[&owner_id, &key]
        )?;

        self.handle_get_metadata(results)
    }

    fn set_account_metadata(&self, owner_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
        let results = self.trans.query("
			INSERT INTO account_metadata (owner_id, key, value)
			VALUES ($1, $2, $3)
			ON CONFLICT ON CONSTRAINT account_metadata_pkey
			DO UPDATE SET value=$3
			RETURNING 1
		", &[&owner_id, &key, &value]);

        self.handle_set_metadata(results, Error::AccountNotFound)
    }

    fn delete_account_metadata(&self, owner_id: Uuid, key: String) -> Result<(), Error> {
        let results = self.trans.query(
            "DELETE FROM account_metadata WHERE owner_id=$1 AND key=$2 RETURNING 1",
            &[&owner_id, &key]
        )?;
        self.handle_delete_metadata(results)
    }

    fn get_vertex_metadata(&self, owner_id: Uuid, key: String) -> Result<JsonValue, Error> {
        let results = self.trans.query(
            "SELECT value FROM vertex_metadata WHERE owner_id=$1 AND key=$2",
            &[&owner_id, &key]
        )?;
        
        self.handle_get_metadata(results)
    }

    fn set_vertex_metadata(&self, owner_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
        let results = self.trans.query("
			INSERT INTO vertex_metadata (owner_id, key, value)
			VALUES ($1, $2, $3)
			ON CONFLICT ON CONSTRAINT vertex_metadata_pkey
			DO UPDATE SET value=$3
			RETURNING 1
		", &[&owner_id, &key, &value]);

        self.handle_set_metadata(results, Error::VertexNotFound)
    }

    fn delete_vertex_metadata(&self, owner_id: Uuid, key: String) -> Result<(), Error> {
        let results = self.trans.query(
            "DELETE FROM vertex_metadata WHERE owner_id=$1 AND key=$2 RETURNING 1",
            &[&owner_id, &key]
        )?;
        
        self.handle_delete_metadata(results)
    }

    fn get_edge_metadata(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, key: String) -> Result<JsonValue, Error> {
        let results = self.trans.query("
			SELECT value
			FROM edge_metadata
			WHERE owner_id=(
                SELECT id FROM edges WHERE outbound_id=$1 AND type=$2 AND inbound_id=$3
            ) AND key=$4
		", &[&outbound_id, &t.0, &inbound_id, &key])?;

        self.handle_get_metadata(results)
    }

    fn set_edge_metadata(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
        let results = self.trans.query("
			INSERT INTO edge_metadata (owner_id, key, value)
			VALUES (
                (SELECT id FROM edges WHERE outbound_id=$1 AND type=$2 AND inbound_id=$3),
                $4,
                $5
            )
			ON CONFLICT ON CONSTRAINT edge_metadata_pkey
			DO UPDATE SET value=$5
			RETURNING 1
		", &[&outbound_id, &t.0, &inbound_id, &key, &value]);

        self.handle_set_metadata(results, Error::EdgeNotFound)
    }

    fn delete_edge_metadata(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, key: String) -> Result<(), Error> {
        let results = self.trans.query("
			DELETE FROM edge_metadata
			WHERE owner_id=(
                SELECT id FROM edges WHERE outbound_id=$1 AND type=$2 AND inbound_id=$3
            ) AND key=$4
			RETURNING 1
		", &[&outbound_id, &t.0, &inbound_id, &key])?;

        self.handle_delete_metadata(results)
    }

    fn commit(self) -> Result<(), Error> {
        self.trans.set_commit();
        self.trans.commit()?;
        Ok(())
    }

    fn rollback(self) -> Result<(), Error> {
        self.trans.set_rollback();
        self.trans.commit()?;
        Ok(())
    }
}
