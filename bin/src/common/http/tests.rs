//! Tests for IndradB's HTTP interface. Most of the tests are implemented by
//! simply reusing the standard test suite made available by the IndraDB lib.
//! To achieve this, we umplement a faux datastore that proxies requests to
//! the GraphQL interface.

use super::{Context, RootMutation, RootQuery, Schema};
use chrono::{DateTime, FixedOffset, Utc};
use indradb::{Datastore, Edge, EdgeDirection, EdgeKey, EdgeMetadata, EdgeQuery, Error, Transaction, Type, Vertex,
              VertexMetadata, VertexQuery};
use juniper::{execute, InputValue, Value, Variables};
use ordermap::OrderMap;
use serde_json;
use serde_json::value::Value as JsonValue;
use statics;
use std::str::FromStr;
use uuid::Uuid;
use std::iter::FromIterator;

macro_rules! vars(
    { $($key:expr => $value:expr),* } => {
        {
            let mut m = Variables::new();
            $(
                m.insert($key.to_string(), $value);
            )*
            m
        }
     };
);

macro_rules! obj(
    { $($key:expr => $value:expr),* } => {
        {
            let mut m = OrderMap::new();
            $(
                m.insert($key.to_string(), $value);
            )*
            m
        }
     };
);

fn extract_bool(v: &Value) -> bool {
    if let Value::Boolean(v) = v {
        *v
    } else {
        panic!("Unexpected value: {:?}", v);
    }
}

fn extract_string(v: &Value) -> String {
    if let Value::String(v) = v {
        v.clone()
    } else {
        panic!("Unexpected value: {:?}", v);
    }
}

fn extract_u64(v: &Value) -> u64 {
    extract_string(v)
        .parse::<u64>()
        .expect("Expected to be able to parse the string into a u64")
}

fn extract_uuid(v: &Value) -> Uuid {
    Uuid::from_str(&extract_string(v)).expect("Expected a UUID")
}

fn extract_type(v: &Value) -> Type {
    Type::new(extract_string(v)).expect("Expected a valid type")
}

fn extract_json(v: &Value) -> serde_json::Value {
    serde_json::from_str(&extract_string(v)).expect("Expected valid JSON")
}

fn extract_vertex(v: &Value) -> Vertex {
    let obj = v.as_object_value().expect("Expected an object");
    let id = extract_uuid(&obj["id"]);
    let t = extract_type(&obj["t"]);
    Vertex::with_id(id, t)
}

fn extract_edge_key(v: &Value) -> EdgeKey {
    let obj = v.as_object_value().expect("Expected an object");
    let outbound_id = extract_uuid(&obj["outboundId"]);
    let t = extract_type(&obj["t"]);
    let inbound_id = extract_uuid(&obj["inboundId"]);
    EdgeKey::new(outbound_id, t, inbound_id)
}

fn extract_edge(v: &Value) -> Edge {
    let obj = v.as_object_value().expect("Expected an object");
    let key = extract_edge_key(&obj["key"]);
    let created_datetime_str = extract_string(&obj["createdDatetime"]);
    let created_datetime = DateTime::<FixedOffset>::parse_from_rfc3339(&created_datetime_str);
    let created_datetime = created_datetime
        .expect("Expected an RFC3339 formatted datetime")
        .with_timezone(&Utc);
    Edge::new(key, created_datetime)
}

// This is basically a copy of the subset of `juniper::InputValue` that we
// need in order to construct queries. It's necessary because
// `juniper::InputValue` is not amenable to iterative changes, which we need
// in order to convert the `indradb::Query`. There might be a cleaner way to
// do this though.
enum QueryInputValueBuilder {
    Object(OrderMap<String, QueryInputValueBuilder>),
    String(String),
    Integer(i32),
    List(Vec<QueryInputValueBuilder>),
    Null
}

impl QueryInputValueBuilder {
    fn to_input_value(self) -> InputValue {
        match self {
            QueryInputValueBuilder::Object(o) => InputValue::object(OrderMap::from_iter(
                o.into_iter().map(|(k, v)| (k, v.to_input_value()))
            )),
            QueryInputValueBuilder::String(s) => InputValue::string(s),
            QueryInputValueBuilder::Integer(i) => InputValue::int(i),
            QueryInputValueBuilder::List(l) => InputValue::list(
                l.into_iter().map(|v| v.to_input_value()).collect()
            ),
            QueryInputValueBuilder::Null => InputValue::null()
        }
    }

    fn add_to_innermost_object(&mut self, name: &str, builder: QueryInputValueBuilder) {
        if let QueryInputValueBuilder::Object(root) = self {
            for value in root.values_mut() {
                if let QueryInputValueBuilder::Object(_) = value {
                    return value.add_to_innermost_object(name, builder);
                }
            }

            root.insert(name.to_string(), builder);
        } else {
            panic!("Expected an object `QueryInputValueBuilder`");
        }
    }

    fn add_metadata(&mut self, name: &str) {
        self.add_to_innermost_object("metadata", QueryInputValueBuilder::List(vec![
            QueryInputValueBuilder::String(name.to_string())
        ]));
    }

    fn from_optional<T, F>(v: &Option<T>, f: F) -> Self
    where F: Fn(&T) -> Self {
        match v {
            Some(v) => f(v),
            None => QueryInputValueBuilder::Null
        }
    }

    fn from_vertex_query(q: &VertexQuery) -> Self {
        match q {
            VertexQuery::All { start_id, limit } => QueryInputValueBuilder::Object(obj!(
                "vertexRange" => QueryInputValueBuilder::Object(obj!(
                    "startId" => QueryInputValueBuilder::from_optional(start_id, |i| {
                        QueryInputValueBuilder::String(i.hyphenated().to_string())
                    }),
                    "limit" => QueryInputValueBuilder::Integer(*limit as i32)
                ))
            )),
            VertexQuery::Vertices { ids } => QueryInputValueBuilder::Object(obj!(
                "vertices" => QueryInputValueBuilder::Object(obj!(
                    "ids" => QueryInputValueBuilder::List(ids.into_iter()
                        .map(|i| QueryInputValueBuilder::String(i.hyphenated().to_string()))
                        .collect())
                ))
            )),
            VertexQuery::Pipe { edge_query, converter, limit } => {
                let mut builder = Self::from_edge_query(edge_query);

                builder.add_to_innermost_object(&converter.to_string(), QueryInputValueBuilder::Object(obj!(
                    "limit" => QueryInputValueBuilder::Integer(*limit as i32)
                )));

                builder
            }
        }
    }

    fn from_edge_query(q: &EdgeQuery) -> Self {
        match q {
            EdgeQuery::Edges { keys } => QueryInputValueBuilder::Object(obj!(
                "edges" => QueryInputValueBuilder::Object(obj!(
                    "keys" => QueryInputValueBuilder::List(keys.iter()
                        .map(|k| QueryInputValueBuilder::Object(obj!(
                            "outboundId" => QueryInputValueBuilder::String(k.outbound_id.hyphenated().to_string()),
                            "t" => QueryInputValueBuilder::String(k.t.0.clone()),
                            "inboundId" => QueryInputValueBuilder::String(k.inbound_id.hyphenated().to_string())
                        )))
                        .collect())
                ))
            )),
            EdgeQuery::Pipe { vertex_query, converter, type_filter, high_filter, low_filter, limit } => {
                let mut builder = Self::from_vertex_query(vertex_query);

                builder.add_to_innermost_object(&converter.to_string(), QueryInputValueBuilder::Object(obj!(
                    "typeFilter" => QueryInputValueBuilder::from_optional(type_filter, |t| {
                        QueryInputValueBuilder::String(t.0.clone())
                    }),
                    "highFilter" => QueryInputValueBuilder::from_optional(high_filter, |h| {
                        QueryInputValueBuilder::String(h.to_rfc3339())
                    }),
                    "lowFilter" => QueryInputValueBuilder::from_optional(low_filter, |l| {
                        QueryInputValueBuilder::String(l.to_rfc3339())
                    }),
                    "limit" => QueryInputValueBuilder::Integer(*limit as i32)
                )));

                builder
            }
        }
    }
}

#[derive(Debug)]
pub struct ClientDatastore;

impl ClientDatastore {
    pub fn default() -> Self {
        Self {}
    }
}

impl Datastore<ClientTransaction> for ClientDatastore {
    fn transaction(&self) -> Result<ClientTransaction, Error> {
        Ok(ClientTransaction::default())
    }
}

pub struct ClientTransaction {
    context: Context,
}

impl ClientTransaction {
    fn default() -> Self {
        let trans = statics::DATASTORE.transaction().unwrap();

        Self {
            context: Context::new(trans),
        }
    }
}

impl ClientTransaction {
    fn request(&self, body: &str, variables: Variables, key: &str) -> Result<Value, Error> {
        let (mut value, errors) = execute(
            body,
            None,
            &Schema::new(RootQuery, RootMutation),
            &variables,
            &self.context,
        ).map_err(|err| Error::from(format!("{:?}", err)))?;

        assert_eq!(errors, vec![]);
        let obj = value
            .as_mut_object_value()
            .expect("Response is not an object");
        let inner_value = obj.remove(key).expect(&format!(
            "Response does not have the expected key `{}`: {:?}",
            key, variables
        ));
        Ok(inner_value)
    }
}

impl Transaction for ClientTransaction {
    fn create_vertex(&self, v: &Vertex) -> Result<bool, Error> {
        Ok(extract_bool(
            &self.request(
                "
                    mutation CreateVertex($id: ID!, $t: String!) {
                        createVertex(vertex: {
                            id: $id,
                            t: $t
                        })
                    }
                ",
                vars!(
                    "id" => InputValue::string(v.id.hyphenated().to_string()),
                    "t" => InputValue::string(v.t.0.clone())
                ),
                "createVertex",
            )?,
        ))
    }

    fn create_vertex_from_type(&self, t: Type) -> Result<Uuid, Error> {
        Ok(extract_uuid(
            &self.request(
                "
                    mutation CreateVertexFromType($t: String!) {
                        createVertexFromType(t: $t)
                    }
                ",
                vars!("t" => InputValue::string(t.0.clone())),
                "createVertexFromType",
            )?,
        ))
    }

    fn get_vertices(&self, q: &VertexQuery) -> Result<Vec<Vertex>, Error> {
        let res = self.request(
            "
                query GetVertices($q: InputRootQuery!) {
                    query(q: $q) {
                        ... on OutputVertex {
                            id
                            t
                        }
                    }
                }
            ",
            vars!("q" => QueryInputValueBuilder::from_vertex_query(q).to_input_value()),
            "query",
        )?;

        let values = res.as_list_value().expect("Expected a list");
        Ok(values.into_iter().map(extract_vertex).collect())
    }

    fn delete_vertices(&self, q: &VertexQuery) -> Result<(), Error> {
        self.request(
            "
                mutation DeleteVertices($q: InputRootQuery!) {
                    delete(q: $q)
                }
            ",
            vars!("q" => QueryInputValueBuilder::from_vertex_query(q).to_input_value()),
            "delete",
        )?;

        Ok(())
    }

    fn get_vertex_count(&self) -> Result<u64, Error> {
        let res = self.request("query { vertexCount }", vars!(), "vertexCount")?;
        Ok(extract_u64(&res))
    }

    fn create_edge(&self, e: &EdgeKey) -> Result<bool, Error> {
        Ok(extract_bool(
            &self.request(
                "
                mutation CreateEdge($outboundId: ID!, $t: String!, $inboundId: ID!) {
                    createEdge(key: {
                        outboundId: $outboundId,
                        t: $t,
                        inboundId: $inboundId
                    })
                }
            ",
                vars!(
                "outboundId" => InputValue::string(e.outbound_id.hyphenated().to_string()),
                "t" => InputValue::string(e.t.0.clone()),
                "inboundId" => InputValue::string(e.inbound_id.hyphenated().to_string())
            ),
                "createEdge",
            )?,
        ))
    }

    fn get_edges(&self, q: &EdgeQuery) -> Result<Vec<Edge>, Error> {
        let res = self.request(
            "
                query GetEdges($q: InputRootQuery!) {
                    query(q: $q) {
                        ... on OutputEdge {
                            key {
                                outboundId
                                t
                                inboundId
                            }
                            createdDatetime
                        }
                    }
                }
            ",
            vars!("q" => QueryInputValueBuilder::from_edge_query(q).to_input_value()),
            "query",
        )?;

        let values = res.as_list_value().expect("Expected a list");
        Ok(values.into_iter().map(extract_edge).collect())
    }

    fn delete_edges(&self, q: &EdgeQuery) -> Result<(), Error> {
        self.request(
            "
                mutation DeleteEdges($q: InputRootQuery!) {
                    delete(q: $q)
                }
            ",
            vars!("q" => QueryInputValueBuilder::from_edge_query(q).to_input_value()),
            "delete",
        )?;

        Ok(())
    }

    fn get_edge_count(&self, id: Uuid, type_filter: Option<&Type>, direction: EdgeDirection) -> Result<u64, Error> {
        let res = self.request(
            "
                query GetEdgeCount($id: ID!, $typeFilter: String, $direction: InputEdgeDirection!) {
                    edgeCount(id: $id, typeFilter: $typeFilter, direction: $direction)
                }
            ",
            vars!(
                "id" => InputValue::string(id.hyphenated().to_string()),
                "typeFilter" => match type_filter {
                    Some(t) => InputValue::string(t.0.clone()),
                    None => InputValue::null(),
                },
                "direction" => InputValue::enum_value(direction.to_string().to_uppercase())
            ),
            "edgeCount",
        )?;
        Ok(extract_u64(&res))
    }

    fn get_vertex_metadata(&self, q: &VertexQuery, name: &str) -> Result<Vec<VertexMetadata>, Error> {
        let mut q = QueryInputValueBuilder::from_vertex_query(q);
        q.add_metadata(name);

        let res = self.request(
            "
                query GetVertexMetadata($q: InputRootQuery!) {
                    query(q: $q) {
                        ... on OutputVertexMetadata {
                            id
                            value
                        }
                    }
                }
            ",
            vars!("q" => q.to_input_value()),
            "query",
        )?;

        let values = res.as_list_value().expect("Expected a list");

        Ok(values
            .into_iter()
            .map(|v| {
                let obj = v.as_object_value().expect("Expected an object");
                let id = extract_uuid(&obj["id"]);
                let value = extract_json(&obj["value"]);
                VertexMetadata::new(id, value)
            })
            .collect())
    }

    fn set_vertex_metadata(&self, q: &VertexQuery, name: &str, value: &JsonValue) -> Result<(), Error> {
        let mut q = QueryInputValueBuilder::from_vertex_query(q);
        q.add_metadata(name);

        self.request(
            "
                mutation SetVertexMetadata($q: InputRootQuery!, $value: String!) {
                    setMetadata(q: $q, value: $value)
                }
            ",
            vars!(
                "q" => q.to_input_value(),
                "value" => InputValue::string(value.to_string())
            ),
            "setMetadata",
        )?;

        Ok(())
    }

    fn delete_vertex_metadata(&self, q: &VertexQuery, name: &str) -> Result<(), Error> {
        let mut q = QueryInputValueBuilder::from_vertex_query(q);
        q.add_metadata(name);

        self.request(
            "
                mutation DeleteVertexMetadata($q: InputRootQuery!) {
                    delete(q: $q)
                }
            ",
            vars!("q" => q.to_input_value()),
            "delete",
        )?;

        Ok(())
    }

    fn get_edge_metadata(&self, q: &EdgeQuery, name: &str) -> Result<Vec<EdgeMetadata>, Error> {
        let mut q = QueryInputValueBuilder::from_edge_query(q);
        q.add_metadata(name);

        let res = self.request(
            "
                query GetEdgeMetadata($q: InputRootQuery!) {
                    query(q: $q) {
                        ... on OutputEdgeMetadata {
                            key {
                                outboundId
                                t
                                inboundId
                            }
                            value
                        }
                    }
                }
            ",
            vars!("q" => q.to_input_value()),
            "query",
        )?;

        let values = res.as_list_value().expect("Expected a list");

        Ok(values
            .into_iter()
            .map(|v| {
                let obj = v.as_object_value().expect("Expected an object");
                let key = extract_edge_key(&obj["key"]);
                let value = extract_json(&obj["value"]);
                EdgeMetadata::new(key, value)
            })
            .collect())
    }

    fn set_edge_metadata(&self, q: &EdgeQuery, name: &str, value: &JsonValue) -> Result<(), Error> {
        let mut q = QueryInputValueBuilder::from_edge_query(q);
        q.add_metadata(name);

        self.request(
            "
                mutation SetEdgeMetadata($q: InputRootQuery!, $value: String!) {
                    setMetadata(q: $q, value: $value)
                }
            ",
            vars!(
                "q" => q.to_input_value(),
                "value" => InputValue::string(value.to_string())
            ),
            "setMetadata",
        )?;

        Ok(())
    }

    fn delete_edge_metadata(&self, q: &EdgeQuery, name: &str) -> Result<(), Error> {
        let mut q = QueryInputValueBuilder::from_edge_query(q);
        q.add_metadata(name);

        self.request(
            "
                mutation DeleteEdgeMetadata($q: InputRootQuery!) {
                    delete(q: $q)
                }
            ",
            vars!("q" => q.to_input_value()),
            "delete",
        )?;

        Ok(())
    }
}

// Standard test suite
indradb_full_test_impl!(ClientDatastore::default());
