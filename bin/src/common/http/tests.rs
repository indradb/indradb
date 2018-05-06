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

fn create_optional<T, F>(v: &Option<T>, f: F) -> InputValue
where
    F: Fn(&T) -> InputValue,
{
    match v {
        Some(v) => f(v),
        None => InputValue::null(),
    }
}

fn create_edge_key(key: &EdgeKey) -> InputValue {
    InputValue::object(obj!(
        "outboundId" => InputValue::string(key.outbound_id.hyphenated().to_string()),
        "t" => InputValue::string(key.t.0.clone()),
        "inboundId" => InputValue::string(key.inbound_id.hyphenated().to_string())
    ))
}

type Container = OrderMap<String, InputValue>;

enum Query {
    Vertex(VertexQuery),
    Edge(EdgeQuery),
}

fn create_query(q: Query, mut manipulators: Vec<Box<FnOnce(&mut Container) -> Container>>) -> InputValue {
    let (root_key, root_obj) = match q {
        Query::Vertex(q) => {
            match q {
                VertexQuery::All { start_id, limit } => ("vertexRange", obj!(
                    "startId" => create_optional(&start_id, |i| InputValue::string(i.hyphenated().to_string())),
                    "limit" => InputValue::int(limit as i32)
                )),
                VertexQuery::Vertices { ids } => ("vertices", obj!(
                    "ids" => InputValue::list(ids.into_iter()
                        .map(|i| InputValue::string(i.hyphenated().to_string()))
                        .collect())
                )),
                VertexQuery::Pipe { edge_query, converter, limit } => {
                    unimplemented!()
                }
            }
        },
        Query::Edge(q) => {
            match q {
                EdgeQuery::Edges { keys } => ("edges", obj!(
                    "keys" => InputValue::list(keys.into_iter().map(create_edge_key).collect())
                )),
                EdgeQuery::Pipe { vertex_query, converter, type_filter, high_filter, low_filter, limit } => {
                    unimplemented!()
                }
            }
        }
    };

    let mut container = root_obj;

    while manipulators.len() > 0 {
        let manipulator = *manipulators.pop();
        container = manipulator(&mut container);
    }

    InputValue::object(obj!(root_key => InputValue::object(root_obj)))
}

fn create_vertex_query(q: VertexQuery) -> InputValue {
    create_query(Query::Vertex(q), vec![])
}

fn create_vertex_metadata_query(q: VertexQuery, name: &str) -> InputValue {
    let manipulator = move |mut container| {
        container.insert("metadata".to_string(), InputValue::list(vec![InputValue::string(name.to_string())]));
        container
    };
    create_query(Query::Vertex(q), vec![Box::new(manipulator)])
}

fn create_edge_query(q: EdgeQuery) -> InputValue {
    create_query(Query::Edge(q), vec![])
}

fn create_edge_metadata_query(q: EdgeQuery, name: &str) -> InputValue {
    let manipulator = move |mut container| {
        container.insert("metadata".to_string(), InputValue::list(vec![InputValue::string(name.to_string())]));
        container
    };
    create_query(Query::Edge(q), vec![Box::new(manipulator)])
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
            vars!("q" => create_vertex_query(q.clone())),
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
            vars!("q" => create_vertex_query(q.clone())),
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
            vars!("q" => create_edge_query(q.clone())),
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
            vars!("q" => create_edge_query(q.clone())),
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
                "typeFilter" => create_optional(&type_filter, |t| InputValue::string(t.0.clone())),
                "direction" => InputValue::enum_value(direction.to_string().to_uppercase())
            ),
            "edgeCount",
        )?;
        Ok(extract_u64(&res))
    }

    fn get_vertex_metadata(&self, q: &VertexQuery, name: &str) -> Result<Vec<VertexMetadata>, Error> {
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
            vars!("q" => create_vertex_metadata_query(q.clone(), name)),
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
        self.request(
            "
                mutation SetVertexMetadata($q: InputRootQuery!, $value: String!) {
                    setMetadata(q: $q, value: $value)
                }
            ",
            vars!(
                "q" => create_vertex_metadata_query(q.clone(), name),
                "value" => InputValue::string(value.to_string())
            ),
            "setMetadata",
        )?;

        Ok(())
    }

    fn delete_vertex_metadata(&self, q: &VertexQuery, name: &str) -> Result<(), Error> {
        self.request(
            "
                mutation DeleteVertexMetadata($q: InputRootQuery!) {
                    delete(q: $q)
                }
            ",
            vars!("q" => create_vertex_metadata_query(q.clone(), name)),
            "delete",
        )?;

        Ok(())
    }

    fn get_edge_metadata(&self, q: &EdgeQuery, name: &str) -> Result<Vec<EdgeMetadata>, Error> {
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
            vars!("q" => create_edge_metadata_query(q.clone(), name)),
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
        self.request(
            "
                mutation SetEdgeMetadata($q: InputRootQuery!, $value: String!) {
                    setMetadata(q: $q, value: $value)
                }
            ",
            vars!(
                "q" => create_edge_metadata_query(q.clone(), name),
                "value" => InputValue::string(value.to_string())
            ),
            "setMetadata",
        )?;

        Ok(())
    }

    fn delete_edge_metadata(&self, q: &EdgeQuery, name: &str) -> Result<(), Error> {
        self.request(
            "
                mutation DeleteEdgeMetadata($q: InputRootQuery!) {
                    delete(q: $q)
                }
            ",
            vars!("q" => create_edge_metadata_query(q.clone(), name)),
            "delete",
        )?;

        Ok(())
    }
}

// This is a copy/paste of the tests ued in `indradb_full_test_impl`, with
// certain tests commented out, either because:
// 1) It relies on nested queries. Nested query -> GraphQL query serialization
//    has not yet been implemented.
// 2) It uses a `limit` value greater than that supported by GraphQL (i.e.
//    greater than `i32::MAX`.)

// Vertex queries
indradb_test!(should_create_vertex_from_type, ClientDatastore::default());
// indradb_test!(should_get_all_vertices, ClientDatastore::default());
indradb_test!(
    should_get_all_vertices_with_zero_limit,
    ClientDatastore::default()
);
// indradb_test!(should_get_all_vertices_out_of_range, ClientDatastore::default());
indradb_test!(should_get_single_vertices, ClientDatastore::default());
indradb_test!(
    should_get_single_vertices_nonexisting,
    ClientDatastore::default()
);
indradb_test!(should_get_vertices, ClientDatastore::default());
// indradb_test!(should_get_vertices_piped, ClientDatastore::default());
indradb_test!(should_get_a_vertex_count, ClientDatastore::default());

// Vertex updates
indradb_test!(should_delete_a_valid_vertex, ClientDatastore::default());
indradb_test!(
    should_not_delete_an_invalid_vertex,
    ClientDatastore::default()
);

// Edges
indradb_test!(should_get_a_valid_edge, ClientDatastore::default());
indradb_test!(should_not_get_an_invalid_edge, ClientDatastore::default());
// indradb_test!(should_create_a_valid_edge, ClientDatastore::default());
indradb_test!(
    should_not_create_an_invalid_edge,
    ClientDatastore::default()
);
indradb_test!(should_delete_a_valid_edge, ClientDatastore::default());
indradb_test!(
    should_not_delete_an_invalid_edge,
    ClientDatastore::default()
);
indradb_test!(should_get_an_edge_count, ClientDatastore::default());
indradb_test!(
    should_get_an_edge_count_with_no_type,
    ClientDatastore::default()
);
indradb_test!(
    should_get_an_edge_count_for_an_invalid_edge,
    ClientDatastore::default()
);
indradb_test!(should_get_an_inbound_edge_count, ClientDatastore::default());
// indradb_test!(should_get_an_edge_range, ClientDatastore::default());
// indradb_test!(should_get_edges_with_no_type, ClientDatastore::default());
// indradb_test!(should_get_no_edges_for_an_invalid_range, ClientDatastore::default());
// indradb_test!(should_get_edges_with_no_high, ClientDatastore::default());
// indradb_test!(should_get_edges_with_no_low, ClientDatastore::default());
// indradb_test!(should_get_edges_with_no_time, ClientDatastore::default());
// indradb_test!(should_get_no_edges_for_reversed_time, ClientDatastore::default());
indradb_test!(should_get_edges, ClientDatastore::default());

// Metadata
indradb_test!(should_handle_vertex_metadata, ClientDatastore::default());
indradb_test!(
    should_not_set_invalid_vertex_metadata,
    ClientDatastore::default()
);
indradb_test!(
    should_not_delete_invalid_vertex_metadata,
    ClientDatastore::default()
);
indradb_test!(should_handle_edge_metadata, ClientDatastore::default());
indradb_test!(
    should_not_set_invalid_edge_metadata,
    ClientDatastore::default()
);
indradb_test!(
    should_not_delete_invalid_edge_metadata,
    ClientDatastore::default()
);
