use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::{Config, Pool, PooledConnection};
use std::mem;
use super::super::{Datastore, Transaction, VertexQuery, EdgeQuery, QueryTypeConverter};
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
use super::util::CTEQueryBuilder;
use postgres::types::ToSql;

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

impl Datastore<PostgresTransaction> for PostgresDatastore {
    fn has_account(&self, account_id: Uuid) -> Result<bool, Error> {
        let conn = self.pool.get()?;

        let results = conn.query("SELECT 1 FROM accounts WHERE id=$1", &[&account_id])?;

        for _ in &results {
            return Result::Ok(true);
        }

        Result::Ok(false)
    }

    fn create_account(&self, email: String) -> Result<(Uuid, String), Error> {
        let id = models::id();
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

    fn vertex_query_to_sql(&self, q: VertexQuery, sql_query_builder: &mut CTEQueryBuilder) {
         match q {
            VertexQuery::All(start_id, limit) => {
                let query_template = "SELECT id, owner_id, type FROM %t WHERE id > %p ORDER BY id LIMIT %p";
                let params: Vec<Box<ToSql>> = vec![Box::new(start_id), Box::new(limit as i64)];
                sql_query_builder.push(query_template, "vertices", params);
            },
            VertexQuery::Vertex(id) => {
                let query_template = "SELECT id, owner_id, type FROM %t WHERE id=%p LIMIT 1";
                let params: Vec<Box<ToSql>> = vec![Box::new(id)];
                sql_query_builder.push(query_template, "vertices", params);
            },
            VertexQuery::Vertices(vertices) => {
                let mut params_template_builder = vec![];
                let mut params: Vec<Box<ToSql>> = vec![];

                for id in vertices.iter() {
                    params_template_builder.push("%p");
                    params.push(Box::new(id.clone()));
                }

                let query_template = format!("SELECT id, owner_id, type FROM %t WHERE id IN ({}) ORDER BY id", params_template_builder.join(", "));
                sql_query_builder.push(&query_template[..], "vertices", params);
            },
            VertexQuery::Pipe(edge_query, converter, limit) => {
                self.edge_query_to_sql(*edge_query, sql_query_builder);
                let params: Vec<Box<ToSql>> = vec![Box::new(limit as i64)];

                let query_template = match converter {
                    QueryTypeConverter::Outbound => "SELECT id, owner_id, type FROM vertices WHERE id IN (SELECT outbound_id FROM %t) ORDER BY id LIMIT %p",
                    QueryTypeConverter::Inbound => "SELECT id, owner_id, type FROM vertices WHERE id IN (SELECT inbound_id FROM %t) ORDER BY id LIMIT %p"
                };

                sql_query_builder.push(query_template, "", params);
            }
        }
    }

    fn edge_query_to_sql(&self, q: EdgeQuery, sql_query_builder: &mut CTEQueryBuilder) {
        match q {
            EdgeQuery::All(t, high, low, limit) => {
                let (where_clause, limit_clause, params) = self.edge_filters_to_sql(t, high, low, limit);
                let query_template = match where_clause.len() {
                    0 => format!("SELECT id, outbound_id, type, inbound_id, update_timestamp, weight FROM %t ORDER BY update_timestamp DESC {}", limit_clause),
                    _ => format!("SELECT id, outbound_id, type, inbound_id, update_timestamp, weight FROM %t WHERE {} ORDER BY update_timestamp DESC {}", where_clause, limit_clause)
                };

                sql_query_builder.push(&query_template[..], "edges", params);
            },
            EdgeQuery::Edge(outbound_id, t, inbound_id) => {
                let params: Vec<Box<ToSql>> = vec![Box::new(outbound_id), Box::new(t.0), Box::new(inbound_id)];

                sql_query_builder.push(
                    "SELECT id, outbound_id, type, inbound_id, update_timestamp, weight FROM %t WHERE outbound_id=%p AND type=%p AND inbound_id=%p",
                    "edges",
                    params
                )
            },
            EdgeQuery::Edges(edges) => {
                let mut params_template_builder = vec![];
                let mut params: Vec<Box<ToSql>> = vec![];

                for edge in edges.iter() {
                    let (outbound_id, t, inbound_id) = edge.clone();
                    params_template_builder.push("(%p, %p, %p)");
                    params.push(Box::new(outbound_id));
                    params.push(Box::new(t.0));
                    params.push(Box::new(inbound_id));
                }

                let query_template = format!("SELECT id, outbound_id, type, inbound_id, update_timestamp, weight FROM %t WHERE (outbound_id, type, inbound_id) IN ({})", params_template_builder.join(", "));
                sql_query_builder.push(&query_template[..], "edges", params);
            },
            EdgeQuery::Pipe(vertex_query, converter, t, high, low, limit) => {
                self.vertex_query_to_sql(*vertex_query, sql_query_builder);

                let (where_clause, limit_clause, params) = self.edge_filters_to_sql(t, high, low, limit);
                let query_template = match (converter, where_clause.len()) {
                    (QueryTypeConverter::Outbound, 0) => {
                        format!("SELECT id, outbound_id, type, inbound_id, update_timestamp, weight FROM edges WHERE outbound_id IN (SELECT id FROM %t) ORDER BY update_timestamp DESC {}", limit_clause)
                    },
                    (QueryTypeConverter::Outbound, _) => {
                        format!("SELECT id, outbound_id, type, inbound_id, update_timestamp, weight FROM edges WHERE outbound_id IN (SELECT id FROM %t) AND {} ORDER BY update_timestamp DESC {}", where_clause, limit_clause)
                    },
                    (QueryTypeConverter::Inbound, 0) => {
                        format!("SELECT id, outbound_id, type, inbound_id, update_timestamp, weight FROM edges WHERE inbound_id IN (SELECT id FROM %t) ORDER BY update_timestamp DESC {}", limit_clause)
                    },
                    (QueryTypeConverter::Inbound, _) => {
                        format!("SELECT id, outbound_id, type, inbound_id, update_timestamp, weight FROM edges WHERE inbound_id IN (SELECT id FROM %t) AND {} ORDER BY update_timestamp DESC {}", where_clause, limit_clause)
                    }
                 };
                
                sql_query_builder.push(&query_template[..], "", params);
            }
        }
    }

    fn edge_filters_to_sql(&self, t: Option<models::Type>, high: Option<DateTime<UTC>>, low: Option<DateTime<UTC>>, limit: u32) -> (String, String, Vec<Box<ToSql>>) {
        let mut where_clause_template_builder = vec![];
        let mut params: Vec<Box<ToSql>> = vec![];

        if let Some(t) = t {
            where_clause_template_builder.push("type = %p");
            params.push(Box::new(t.0));
        }

        if let Some(high) = high {
            where_clause_template_builder.push("update_timestamp <= %p");
            params.push(Box::new(high));
        }

        if let Some(low) = low {
            where_clause_template_builder.push("update_timestamp >= %p");
            params.push(Box::new(low));
        }

        params.push(Box::new(limit as i64));
        (where_clause_template_builder.join(" AND "), "LIMIT %p".to_string(), params)
    }

    fn ensure_edges_are_owned_by_account(&self, sql_query_builder: &mut CTEQueryBuilder) {
        sql_query_builder.push("SELECT %t.id FROM %t JOIN vertices ON %t.outbound_id=vertices.id WHERE vertices.owner_id=%p", "vertices", vec![Box::new(self.account_id)]);
    }
}

impl Transaction for PostgresTransaction {
    fn create_vertex(&self, t: models::Type) -> Result<Uuid, Error> {
        let id = models::id();
        self.trans.execute("INSERT INTO vertices (id, type, owner_id) VALUES ($1, $2, $3)", &[&id, &t.0, &self.account_id])?;
        Ok(id)
    }

    fn get_vertices(&self, q: VertexQuery) -> Result<Vec<models::Vertex>, Error> {
        let mut sql_query_builder = CTEQueryBuilder::new();
        self.vertex_query_to_sql(q, &mut sql_query_builder);
        let (query, params) = sql_query_builder.to_query_payload("SELECT id, type FROM %t", vec![]);
        let params_refs: Vec<&ToSql> = params.iter().map(|x| &**x).collect();

        let results = self.trans.query(&query[..], &params_refs[..])?;
        let mut vertices: Vec<models::Vertex> = Vec::new();

        for row in &results {
            let id: Uuid = row.get(0);
            let t_str: String = row.get(1);
            let v = models::Vertex::new(id, models::Type::new(t_str).unwrap());
            vertices.push(v);
        }

        Ok(vertices)
    }

    fn set_vertices(&self, q: VertexQuery, t: models::Type) -> Result<(), Error> {
        let mut sql_query_builder = CTEQueryBuilder::new();
        self.vertex_query_to_sql(q, &mut sql_query_builder);
        let (query, params) = sql_query_builder.to_query_payload(
            "UPDATE vertices SET type=%p WHERE id IN (SELECT id FROM %t WHERE owner_id=%p)",
            vec![Box::new(t.0), Box::new(self.account_id)]
        );
        let params_refs: Vec<&ToSql> = params.iter().map(|x| &**x).collect();
        self.trans.execute(&query[..], &params_refs[..])?;
        Ok(())
    }

    fn delete_vertices(&self, q: VertexQuery) -> Result<(), Error> {
        let mut sql_query_builder = CTEQueryBuilder::new();
        self.vertex_query_to_sql(q, &mut sql_query_builder);
        let (query, params) = sql_query_builder.to_query_payload("DELETE FROM vertices WHERE id IN (SELECT id FROM %t WHERE owner_id=%p)", vec![Box::new(self.account_id)]);
        let params_refs: Vec<&ToSql> = params.iter().map(|x| &**x).collect();
        self.trans.execute(&query[..], &params_refs[..])?;
        Ok(())
    }

    fn create_edge(&self, e: models::Edge) -> Result<(), Error> {
        let id = models::id();

        // Because this command could fail, we need to set a savepoint to roll
        // back to, rather than spoiling the entire transaction
        let results = {
            let trans = self.trans.savepoint("set_edge")?;
            let results = trans.query("
                INSERT INTO edges (id, outbound_id, type, inbound_id, weight, update_timestamp)
                VALUES ($1, (SELECT id FROM vertices WHERE id=$2 AND owner_id=$3), $4, $5, $6, CLOCK_TIMESTAMP())
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
                let v = self.get_vertices(VertexQuery::Vertex(e.outbound_id))?;
                if v.len() == 0 {
                    return Err(Error::VertexNotFound);
                } else {
                    return Err(Error::Unauthorized);
                }
            } else if err.code == pg_error::SqlState::ForeignKeyViolation {
                // This should only happen when there is no vertex with id=inbound_id
                return Err(Error::VertexNotFound);
            }
        }
        
        Ok(())
    }

    fn get_edges(&self, q: EdgeQuery) -> Result<Vec<models::Edge>, Error> {
        let mut sql_query_builder = CTEQueryBuilder::new();
        self.edge_query_to_sql(q, &mut sql_query_builder);
        let (query, params) = sql_query_builder.to_query_payload("SELECT outbound_id, type, inbound_id, weight, update_timestamp FROM %t", vec![]);
        let params_refs: Vec<&ToSql> = params.iter().map(|x| &**x).collect();

        let results = self.trans.query(&query[..], &params_refs[..])?;
        let mut edges: Vec<models::Edge> = Vec::new();

        for row in &results {
            let outbound_id: Uuid = row.get(0);
            let t_str: String = row.get(1);
            let inbound_id: Uuid = row.get(2);
            let weight_f32: f32 = row.get(3);
            let update_datetime: DateTime<UTC> = row.get(4);

            let e = models::Edge::new(
                outbound_id,
                models::Type::new(t_str).unwrap(),
                inbound_id,
                models::Weight::new(weight_f32).unwrap(),
                update_datetime
            );

            edges.push(e);
        }

        Ok(edges)
    }

    fn set_edges(&self, q: EdgeQuery, weight: models::Weight) -> Result<(), Error> {
        let mut sql_query_builder = CTEQueryBuilder::new();
        self.edge_query_to_sql(q, &mut sql_query_builder);
        self.ensure_edges_are_owned_by_account(&mut sql_query_builder);
        let (query, params) = sql_query_builder.to_query_payload("UPDATE edges SET weight=%p, update_timestamp=NOW() WHERE id IN (SELECT id FROM %t)", vec![Box::new(weight.0)]);
        let params_refs: Vec<&ToSql> = params.iter().map(|x| &**x).collect();
        self.trans.execute(&query[..], &params_refs[..])?;
        Ok(())
    }

    fn delete_edges(&self, q: EdgeQuery) -> Result<(), Error> {
        let mut sql_query_builder = CTEQueryBuilder::new();
        self.edge_query_to_sql(q, &mut sql_query_builder);
        self.ensure_edges_are_owned_by_account(&mut sql_query_builder);
        let (query, params) = sql_query_builder.to_query_payload("DELETE FROM edges WHERE id IN (SELECT id FROM %t)", vec![]);
        let params_refs: Vec<&ToSql> = params.iter().map(|x| &**x).collect();
        self.trans.execute(&query[..], &params_refs[..])?;
        Ok(())
    }

    fn get_edge_count(&self, q: EdgeQuery) -> Result<u64, Error> {
        let mut sql_query_builder = CTEQueryBuilder::new();
        self.edge_query_to_sql(q, &mut sql_query_builder);
        let (query, params) = sql_query_builder.to_query_payload("SELECT COUNT(id) FROM %t", vec![]);
        let params_refs: Vec<&ToSql> = params.iter().map(|x| &**x).collect();
        let results = self.trans.query(&query[..], &params_refs[..])?;

        for row in &results {
            let count: i64 = row.get(0);
            return Ok(count as u64);
        }

        panic!("Unreachable point hit");
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
