use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use r2d2::{Pool, PooledConnection};
use std::mem;
use super::super::{Datastore, EdgeQuery, QueryTypeConverter, Transaction, VertexQuery};
use models;
use errors::Error;
use postgres;
use postgres::rows::Rows;
use chrono::DateTime;
use chrono::offset::Utc;
use serde_json::Value as JsonValue;
use num_cpus;
use uuid::Uuid;
use std::i64;
use postgres::error as pg_error;
use super::util::CTEQueryBuilder;
use postgres::types::ToSql;
use super::schema;
use util::UuidGenerator;
use std::sync::Arc;

/// A datastore that is backed by a postgres database.
#[derive(Clone, Debug)]
pub struct PostgresDatastore {
    pool: Pool<PostgresConnectionManager>,
    uuid_generator: Arc<UuidGenerator>,
}

impl PostgresDatastore {
    /// Creates a new postgres-backed datastore.
    ///
    /// # Arguments
    /// * `pool_size` - The maximum number of connections to maintain to
    ///   postgres. If `None`, it defaults to twice the number of CPUs.
    /// * `connetion_string` - The postgres database connection string.
    /// * `secure_uuids` - If true, UUIDv4 will be used, which will result in
    ///   difficult to guess UUIDs at the detriment of a more index-optimized
    ///   (and thus faster) variant.
    pub fn new(
        pool_size: Option<u32>,
        connection_string: String,
        secure_uuids: bool,
    ) -> Result<PostgresDatastore, Error> {
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

        let manager = PostgresConnectionManager::new(&*connection_string, TlsMode::None)?;
        let pool = Pool::builder()
            .max_size(unwrapped_pool_size)
            .build(manager)?;

        Ok(PostgresDatastore {
            pool: pool,
            uuid_generator: Arc::new(UuidGenerator::new(secure_uuids)),
        })
    }

    /// Creates a new postgres-backed datastore.
    ///
    /// # Arguments
    /// * `connetion_string` - The postgres database connection string.
    pub fn create_schema(connection_string: String) -> Result<(), Error> {
        let conn = postgres::Connection::connect(connection_string, postgres::TlsMode::None)
            .map_err(|err| {
                let message = format!("Could not connect to the postgres database: {}", err);
                Error::Unexpected(message)
            })?;

        for statement in schema::SCHEMA.split(";") {
            conn.execute(statement, &vec![])?;
        }

        Ok(())
    }
}

impl Datastore<PostgresTransaction> for PostgresDatastore {
    fn transaction(&self) -> Result<PostgresTransaction, Error> {
        let conn = self.pool.get()?;
        let trans = PostgresTransaction::new(conn, self.uuid_generator.clone())?;
        Ok(trans)
    }
}

/// A postgres-backed datastore transaction.
#[derive(Debug)]
pub struct PostgresTransaction {
    trans: postgres::transaction::Transaction<'static>,
    conn: Box<PooledConnection<PostgresConnectionManager>>,
    uuid_generator: Arc<UuidGenerator>,
}

impl PostgresTransaction {
    fn new(
        conn: PooledConnection<PostgresConnectionManager>,
        uuid_generator: Arc<UuidGenerator>,
    ) -> Result<Self, Error> {
        let conn = Box::new(conn);

        let trans = unsafe {
            mem::transmute(conn.transaction().map_err(|err| {
                Error::Unexpected(format!("Could not create transaction: {}", err))
            })?)
        };

        Ok(PostgresTransaction {
            conn: conn,
            trans: trans,
            uuid_generator: uuid_generator,
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
        if results.is_empty() {
            Err(Error::MetadataNotFound)
        } else {
            Ok(())
        }
    }

    fn handle_set_metadata_error(&self, err: pg_error::Error, foreign_key_err: Error) -> Error {
        if let Some(state) = err.code() {
            if state == &pg_error::FOREIGN_KEY_VIOLATION || state == &pg_error::NOT_NULL_VIOLATION {
                // This should only happen when we couldn't get the
                // "owning" resource for the metadata
                return foreign_key_err;
            }
        }

        Error::from(err)
    }

    fn vertex_query_to_sql(&self, q: VertexQuery, sql_query_builder: &mut CTEQueryBuilder) {
        match q {
            VertexQuery::All { start_id, limit } => match start_id {
                Some(start_id) => {
                    let query_template =
                        "SELECT id, type FROM %t WHERE id > %p ORDER BY id LIMIT %p";
                    let params: Vec<Box<ToSql>> = vec![Box::new(start_id), Box::new(limit as i64)];
                    sql_query_builder.push(query_template, "vertices", params);
                }
                None => {
                    let query_template = "SELECT id, type FROM %t ORDER BY id LIMIT %p";
                    let params: Vec<Box<ToSql>> = vec![Box::new(limit as i64)];
                    sql_query_builder.push(query_template, "vertices", params);
                }
            },
            VertexQuery::Vertices { ids } => {
                let mut params_template_builder = vec![];
                let mut params: Vec<Box<ToSql>> = vec![];

                for id in ids {
                    params_template_builder.push("%p");
                    params.push(Box::new(id));
                }

                let query_template = format!(
                    "SELECT id, type FROM %t WHERE id IN ({}) ORDER BY id",
                    params_template_builder.join(", ")
                );
                sql_query_builder.push(&query_template[..], "vertices", params);
            }
            VertexQuery::Pipe {
                edge_query,
                converter,
                limit,
            } => {
                self.edge_query_to_sql(*edge_query, sql_query_builder);
                let params: Vec<Box<ToSql>> = vec![Box::new(limit as i64)];

                let query_template = match converter {
                    QueryTypeConverter::Outbound => {
                        "SELECT id, type FROM vertices WHERE id IN (SELECT outbound_id FROM %t) ORDER BY id LIMIT %p"
                    }
                    QueryTypeConverter::Inbound => {
                        "SELECT id, type FROM vertices WHERE id IN (SELECT inbound_id FROM %t) ORDER BY id LIMIT %p"
                    }
                };

                sql_query_builder.push(query_template, "", params);
            }
        }
    }

    fn edge_query_to_sql(&self, q: EdgeQuery, sql_query_builder: &mut CTEQueryBuilder) {
        match q {
            EdgeQuery::Edges { keys } => {
                let mut params_template_builder = vec![];
                let mut params: Vec<Box<ToSql>> = vec![];

                for key in keys {
                    params_template_builder.push("(%p, %p, %p)");
                    params.push(Box::new(key.outbound_id));
                    params.push(Box::new(key.t.0));
                    params.push(Box::new(key.inbound_id));
                }

                let query_template = format!(
                    "SELECT id, outbound_id, type, inbound_id, update_timestamp FROM %t WHERE (outbound_id, type, inbound_id) IN ({})",
                    params_template_builder.join(", ")
                );
                sql_query_builder.push(&query_template[..], "edges", params);
            }
            EdgeQuery::Pipe {
                vertex_query,
                converter,
                type_filter,
                high_filter,
                low_filter,
                limit,
            } => {
                self.vertex_query_to_sql(*vertex_query, sql_query_builder);

                let mut where_clause_template_builder = vec![];
                let mut params: Vec<Box<ToSql>> = vec![];

                if let Some(type_filter) = type_filter {
                    where_clause_template_builder.push("type = %p");
                    params.push(Box::new(type_filter.0));
                }

                if let Some(high_filter) = high_filter {
                    where_clause_template_builder.push("update_timestamp <= %p");
                    params.push(Box::new(high_filter));
                }

                if let Some(low_filter) = low_filter {
                    where_clause_template_builder.push("update_timestamp >= %p");
                    params.push(Box::new(low_filter));
                }

                params.push(Box::new(limit as i64));
                let where_clause = where_clause_template_builder.join(" AND ");

                let query_template = match (converter, where_clause.len()) {
                    (QueryTypeConverter::Outbound, 0) => {
                        "SELECT id, outbound_id, type, inbound_id, update_timestamp FROM edges WHERE outbound_id IN (SELECT id FROM %t) ORDER BY update_timestamp DESC LIMIT %p".to_string()
                    }
                    (QueryTypeConverter::Outbound, _) => {
                        format!(
                            "SELECT id, outbound_id, type, inbound_id, update_timestamp FROM edges WHERE outbound_id IN (SELECT id FROM %t) AND {} ORDER BY update_timestamp DESC LIMIT %p",
                            where_clause
                        )
                    }
                    (QueryTypeConverter::Inbound, 0) => {
                        "SELECT id, outbound_id, type, inbound_id, update_timestamp FROM edges WHERE inbound_id IN (SELECT id FROM %t) ORDER BY update_timestamp DESC LIMIT %p".to_string()
                    }
                    (QueryTypeConverter::Inbound, _) => {
                        format!(
                            "SELECT id, outbound_id, type, inbound_id, update_timestamp FROM edges WHERE inbound_id IN (SELECT id FROM %t) AND {} ORDER BY update_timestamp DESC LIMIT %p",
                            where_clause
                        )
                    }
                };

                sql_query_builder.push(&query_template[..], "", params);
            }
        }
    }
}

impl Transaction for PostgresTransaction {
    fn create_vertex(&self, t: models::Type) -> Result<Uuid, Error> {
        let id = self.uuid_generator.next();
        self.trans.execute(
            "INSERT INTO vertices (id, type) VALUES ($1, $2)",
            &[&id, &t.0],
        )?;
        Ok(id)
    }

    fn get_vertices(&self, q: VertexQuery) -> Result<Vec<models::Vertex>, Error> {
        let mut sql_query_builder = CTEQueryBuilder::new();
        self.vertex_query_to_sql(q, &mut sql_query_builder);
        let (query, params) =
            sql_query_builder.into_query_payload("SELECT id, type FROM %t", vec![]);
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

    fn delete_vertices(&self, q: VertexQuery) -> Result<(), Error> {
        let mut sql_query_builder = CTEQueryBuilder::new();
        self.vertex_query_to_sql(q, &mut sql_query_builder);
        let (query, params) = sql_query_builder.into_query_payload(
            "DELETE FROM vertices WHERE id IN (SELECT id FROM %t)",
            vec![],
        );
        let params_refs: Vec<&ToSql> = params.iter().map(|x| &**x).collect();
        self.trans.execute(&query[..], &params_refs[..])?;
        Ok(())
    }

    fn create_edge(&self, key: models::EdgeKey) -> Result<(), Error> {
        let id = self.uuid_generator.next();

        // Because this command could fail, we need to set a savepoint to roll
        // back to, rather than spoiling the entire transaction
        let results = {
            let trans = self.trans.savepoint("set_edge")?;
            let results = trans.query(
                "
                INSERT INTO edges (id, outbound_id, type, inbound_id, update_timestamp)
                VALUES ($1, $2, $3, $4, CLOCK_TIMESTAMP())
                ON CONFLICT ON CONSTRAINT edges_outbound_id_type_inbound_id_ukey
                DO UPDATE SET update_timestamp=CLOCK_TIMESTAMP()
            ",
                &[&id, &key.outbound_id, &key.t.0, &key.inbound_id],
            );

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

        if let Err(err) = results {
            if let Some(state) = err.code() {
                if state == &pg_error::FOREIGN_KEY_VIOLATION {
                    // This should only happen when there is no vertex with id=inbound_id
                    return Err(Error::VertexNotFound);
                }
            }
        }

        Ok(())
    }

    fn get_edges(&self, q: EdgeQuery) -> Result<Vec<models::Edge>, Error> {
        let mut sql_query_builder = CTEQueryBuilder::new();
        self.edge_query_to_sql(q, &mut sql_query_builder);
        let (query, params) = sql_query_builder.into_query_payload(
            "SELECT outbound_id, type, inbound_id, update_timestamp FROM %t",
            vec![],
        );
        let params_refs: Vec<&ToSql> = params.iter().map(|x| &**x).collect();

        let results = self.trans.query(&query[..], &params_refs[..])?;
        let mut edges: Vec<models::Edge> = Vec::new();

        for row in &results {
            let outbound_id: Uuid = row.get(0);
            let t_str: String = row.get(1);
            let inbound_id: Uuid = row.get(2);
            let update_datetime: DateTime<Utc> = row.get(3);
            let t = models::Type::new(t_str).unwrap();
            let key = models::EdgeKey::new(outbound_id, t, inbound_id);
            let edge = models::Edge::new(key, update_datetime);
            edges.push(edge);
        }

        Ok(edges)
    }

    fn delete_edges(&self, q: EdgeQuery) -> Result<(), Error> {
        let mut sql_query_builder = CTEQueryBuilder::new();
        self.edge_query_to_sql(q, &mut sql_query_builder);
        let (query, params) = sql_query_builder
            .into_query_payload("DELETE FROM edges WHERE id IN (SELECT id FROM %t)", vec![]);
        let params_refs: Vec<&ToSql> = params.iter().map(|x| &**x).collect();
        self.trans.execute(&query[..], &params_refs[..])?;
        Ok(())
    }

    fn get_edge_count(&self, q: EdgeQuery) -> Result<u64, Error> {
        let mut sql_query_builder = CTEQueryBuilder::new();
        self.edge_query_to_sql(q, &mut sql_query_builder);
        let (query, params) =
            sql_query_builder.into_query_payload("SELECT COUNT(id) FROM %t", vec![]);
        let params_refs: Vec<&ToSql> = params.iter().map(|x| &**x).collect();
        let results = self.trans.query(&query[..], &params_refs[..])?;

        for row in &results {
            let count: i64 = row.get(0);
            return Ok(count as u64);
        }

        unreachable!();
    }

    fn get_global_metadata(&self, name: String) -> Result<JsonValue, Error> {
        let results = self.trans
            .query("SELECT value FROM global_metadata WHERE name=$1", &[&name])?;
        self.handle_get_metadata(results)
    }

    fn set_global_metadata(&self, name: String, value: JsonValue) -> Result<(), Error> {
        // Because this command could fail, we need to set a savepoint to roll
        // back to, rather than spoiling the entire transaction
        let trans = self.trans.savepoint("set_global_metadata")?;

        let results = trans.query(
            "
            INSERT INTO global_metadata (name, value)
            VALUES ($1, $2)
            ON CONFLICT ON CONSTRAINT global_metadata_pkey
            DO UPDATE SET value=$2
            RETURNING 1
            ",
            &[&name, &value],
        );

        match results {
            Err(err) => {
                trans.set_rollback();
                let foreign_key_err =
                    Error::Unexpected("Unexpected error when setting global metadata".to_string());
                Err(self.handle_set_metadata_error(err, foreign_key_err))
            }
            Ok(_) => {
                trans.set_commit();
                Ok(())
            }
        }
    }

    fn delete_global_metadata(&self, name: String) -> Result<(), Error> {
        let results = self.trans.query(
            "DELETE FROM global_metadata WHERE name=$1 RETURNING 1",
            &[&name],
        )?;

        self.handle_delete_metadata(results)
    }

    fn get_vertex_metadata(
        &self,
        q: VertexQuery,
        name: String,
    ) -> Result<Vec<models::VertexMetadata>, Error> {
        let mut sql_query_builder = CTEQueryBuilder::new();
        self.vertex_query_to_sql(q, &mut sql_query_builder);
        let (query, params) = sql_query_builder.into_query_payload("SELECT owner_id, value FROM vertex_metadata WHERE owner_id IN (SELECT id FROM %t) AND name=%p", vec![Box::new(name)]);
        let params_refs: Vec<&ToSql> = params.iter().map(|x| &**x).collect();
        let results = self.trans.query(&query[..], &params_refs[..])?;
        let mut metadata = Vec::new();

        for row in &results {
            let id: Uuid = row.get(0);
            let value: JsonValue = row.get(1);
            metadata.push(models::VertexMetadata::new(id, value));
        }

        Ok(metadata)
    }

    fn set_vertex_metadata(
        &self,
        q: VertexQuery,
        name: String,
        value: JsonValue,
    ) -> Result<(), Error> {
        let mut sql_query_builder = CTEQueryBuilder::new();
        self.vertex_query_to_sql(q, &mut sql_query_builder);
        let (query, params) = sql_query_builder.into_query_payload(
            "
            INSERT INTO vertex_metadata (owner_id, name, value)
            SELECT id, %p, %p FROM %t
            ON CONFLICT ON CONSTRAINT vertex_metadata_pkey
            DO UPDATE SET value=%p
            ",
            vec![Box::new(name), Box::new(value.clone()), Box::new(value)],
        );
        let params_refs: Vec<&ToSql> = params.iter().map(|x| &**x).collect();
        self.trans.execute(&query[..], &params_refs[..])?;
        Ok(())
    }

    fn delete_vertex_metadata(&self, q: VertexQuery, name: String) -> Result<(), Error> {
        let mut sql_query_builder = CTEQueryBuilder::new();
        self.vertex_query_to_sql(q, &mut sql_query_builder);
        let (query, params) = sql_query_builder.into_query_payload(
            "DELETE FROM vertex_metadata WHERE owner_id IN (SELECT id FROM %t) AND name=%p",
            vec![Box::new(name)],
        );
        let params_refs: Vec<&ToSql> = params.iter().map(|x| &**x).collect();
        self.trans.execute(&query[..], &params_refs[..])?;
        Ok(())
    }

    fn get_edge_metadata(
        &self,
        q: EdgeQuery,
        name: String,
    ) -> Result<Vec<models::EdgeMetadata>, Error> {
        let mut sql_query_builder = CTEQueryBuilder::new();
        self.edge_query_to_sql(q, &mut sql_query_builder);

        let (query, params) = sql_query_builder.into_query_payload(
            "
            SELECT edges.outbound_id, edges.type, edges.inbound_id, edge_metadata.value
            FROM edge_metadata JOIN edges ON edge_metadata.owner_id=edges.id
            WHERE owner_id IN (SELECT id FROM %t) AND name=%p
            ",
            vec![Box::new(name)],
        );

        let params_refs: Vec<&ToSql> = params.iter().map(|x| &**x).collect();
        let results = self.trans.query(&query[..], &params_refs[..])?;
        let mut metadata = Vec::new();

        for row in &results {
            let outbound_id: Uuid = row.get(0);
            let t_str: String = row.get(1);
            let inbound_id: Uuid = row.get(2);
            let value: JsonValue = row.get(3);
            let t = models::Type::new(t_str).unwrap();
            let key = models::EdgeKey::new(outbound_id, t, inbound_id);
            metadata.push(models::EdgeMetadata::new(key, value));
        }

        Ok(metadata)
    }

    fn set_edge_metadata(&self, q: EdgeQuery, name: String, value: JsonValue) -> Result<(), Error> {
        let mut sql_query_builder = CTEQueryBuilder::new();
        self.edge_query_to_sql(q, &mut sql_query_builder);
        let (query, params) = sql_query_builder.into_query_payload(
            "
            INSERT INTO edge_metadata (owner_id, name, value)
            SELECT id, %p, %p FROM %t
            ON CONFLICT ON CONSTRAINT edge_metadata_pkey
            DO UPDATE SET value=%p
            ",
            vec![Box::new(name), Box::new(value.clone()), Box::new(value)],
        );
        let params_refs: Vec<&ToSql> = params.iter().map(|x| &**x).collect();
        self.trans.execute(&query[..], &params_refs[..])?;
        Ok(())
    }

    fn delete_edge_metadata(&self, q: EdgeQuery, name: String) -> Result<(), Error> {
        let mut sql_query_builder = CTEQueryBuilder::new();
        self.edge_query_to_sql(q, &mut sql_query_builder);
        let (query, params) = sql_query_builder.into_query_payload(
            "DELETE FROM edge_metadata WHERE owner_id IN (SELECT id FROM %t) AND name=%p",
            vec![Box::new(name)],
        );
        let params_refs: Vec<&ToSql> = params.iter().map(|x| &**x).collect();
        self.trans.execute(&query[..], &params_refs[..])?;
        Ok(())
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
