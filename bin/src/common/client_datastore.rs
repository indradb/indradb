use super::http;
use super::http::models;
use super::statics;
use indradb::{Datastore, Edge, EdgeDirection, EdgeKey, EdgeMetadata, EdgeQuery, Error, Transaction, Type, Vertex, VertexMetadata, VertexQuery};
use serde_json::value::Value as JsonValue;
use uuid::Uuid;
use juniper::{InputValue, Variables, Value, execute};
use std::str::FromStr;
use ordermap::OrderMap;
use chrono::{DateTime, FixedOffset, Utc};

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
    extract_string(v).parse::<u64>().expect("Expected to be able to parse the string into a u64")
}

fn extract_vertex(v: &Value) -> Vertex {
    let obj = v.as_object_value().expect("Expected an object");
    let id_str = extract_string(&obj["id"]);
    let id = Uuid::from_str(&id_str).expect("Expected a UUID");
    let t_str = extract_string(&obj["t"]);
    let t = Type::new(t_str).expect("Expected a valid type");
    Vertex::with_id(id, t)
}

fn extract_edge_key(v: &Value) -> EdgeKey {
    let obj = v.as_object_value().expect("Expected an object");
    let outbound_id_str = extract_string(&obj["outboundId"]);
    let outbound_id = Uuid::from_str(&outbound_id_str).expect("Expected a UUID");
    let t_str = extract_string(&obj["t"]);
    let t = Type::new(t_str).expect("Expected a valid type");
    let inbound_id_str = extract_string(&obj["inboundId"]);
    let inbound_id = Uuid::from_str(&inbound_id_str).expect("Expected a UUID");
    EdgeKey::new(outbound_id, t, inbound_id)
}

fn extract_edge(v: &Value) -> Edge {
    let obj = v.as_object_value().expect("Expected an object");
    let key = extract_edge_key(&obj["key"]);
    let created_datetime_str = extract_string(&obj["createdDatetime"]);
    let created_datetime = DateTime::<FixedOffset>::parse_from_rfc3339(&created_datetime_str);
    let created_datetime = created_datetime.expect("Expected an RFC3339 formatted datetime").with_timezone(&Utc);
    Edge::new(key, created_datetime)
}

fn create_optional<T, F>(v: &Option<T>, f: F) -> InputValue
where F: Fn(&T) -> InputValue {
    match v {
        Some(v) => f(v),
        None => InputValue::null()
    }
}

fn create_vertex_query(q: &VertexQuery) -> OrderMap<String, InputValue> {
    match q {
        VertexQuery::All { start_id, limit } => {
            obj!(
                "vertexRange" => InputValue::object(obj!(
                    "startId" => create_optional(start_id, |i| InputValue::string(i.hyphenated().to_string())),
                    "limit" => InputValue::int(*limit as i32)
                ))
            )
        },
        VertexQuery::Vertices { ids } => {
            let ids = ids.into_iter().map(|i| InputValue::string(i.hyphenated().to_string())).collect();
            obj!("vertices" => InputValue::object(obj!("ids" => InputValue::list(ids))))
        },
        VertexQuery::Pipe { edge_query, converter, limit } => {
            let mut o = create_edge_query(edge_query);

            o.insert(converter.to_string(), InputValue::object(obj!(
                "limit" => InputValue::int(*limit as i32)
            )));

            o
        }
    }
}

fn create_edge_query(q: &EdgeQuery) -> OrderMap<String, InputValue> {
    match q {
        EdgeQuery::Edges { keys } => {
            let keys = keys.into_iter().map(|key| {
                InputValue::object(obj!(
                    "outboundId" => InputValue::string(key.outbound_id.hyphenated().to_string()),
                    "t" => InputValue::string(key.t.0.clone()),
                    "inboundId" => InputValue::string(key.inbound_id.hyphenated().to_string())
                ))
            }).collect();

            obj!("edges" => InputValue::object(obj!("keys" => InputValue::list(keys))))
        },
        EdgeQuery::Pipe { vertex_query, converter, type_filter, high_filter, low_filter, limit } => {
            let mut o = create_vertex_query(vertex_query);

            o.insert(converter.to_string(), InputValue::object(obj!(
                "typeFilter" => create_optional(type_filter, |t| InputValue::string(t.0.clone())),
                "highFilter" => create_optional(high_filter, |h| InputValue::string(h.to_rfc3339())),
                "lowFilter" => create_optional(low_filter, |l| InputValue::string(l.to_rfc3339())),
                "limit" => InputValue::int(*limit as i32)
            )));

            o
        }
    }
}

#[derive(Debug)]
pub struct ClientDatastore;

impl ClientDatastore {
    fn default() -> Self {
        Self{}
    }
}

impl Datastore<ClientTransaction> for ClientDatastore {
    fn transaction(&self) -> Result<ClientTransaction, Error> {
        Ok(ClientTransaction::default())
    }
}

pub struct ClientTransaction {
    context: http::Context
}

impl ClientTransaction {
    fn default() -> Self {
        let trans = statics::DATASTORE.transaction().unwrap();

        Self {
            context: http::Context::new(trans)
        }
    }
}

impl ClientTransaction {
    fn request(&self, body: &str, variables: Variables, key: &str) -> Result<Value, Error> {
        let (mut value, errors) = execute(
            body,
            None,
            &http::Schema::new(http::RootQuery, http::RootMutation),
            &variables,
            &self.context,
        ).unwrap();

        if errors.len() > 0 {
            let description = format!("{:?}", errors[0]);
            return Err(Error::from(description));
        }

        let obj = value.as_mut_object_value().expect("Response is not an object");
        let inner_value = obj.remove(key).expect(&format!("Response does not have the expected key `{}`: {:?}", key, variables));
        Ok(inner_value)
    }
}

impl Transaction for ClientTransaction {
    fn create_vertex(&self, v: &Vertex) -> Result<bool, Error> {
        Ok(extract_bool(&self.request("
            mutation CreateVertex($id: ID!, $t: String!) {
                createVertex(vertex: {
                    id: $id,
                    t: $t
                })
            }
        ", vars!(
            "id" => InputValue::String(v.id.hyphenated().to_string()),
            "t" => InputValue::String(v.t.0.clone())
        ), "createVertex")?))
    }

    fn create_vertex_from_type(&self, t: Type) -> Result<Uuid, Error> {
        let s = extract_string(&self.request("
            mutation CreateVertexFromType($t: String!) {
                createVertexFromType(t: $t)
            }
        ", vars!(
            "t" => InputValue::String(t.0.clone())
        ), "createVertexFromType")?);

        Ok(Uuid::from_str(&s).unwrap())
    }

    fn get_vertices(&self, q: &VertexQuery) -> Result<Vec<Vertex>, Error> {
        let res = self.request("
            query GetVertices($q: InputRootQuery!) {
                query(q: $q) {
                    ... on OutputVertex {
                        id
                        t
                    }
                }
            }
        ", vars!(
            "q" => InputValue::object(create_vertex_query(q))
        ), "query")?;

        let values = res.as_list_value().expect("Expected a list");
        Ok(values.into_iter().map(extract_vertex).collect())
    }

    fn delete_vertices(&self, q: &VertexQuery) -> Result<(), Error> {
        let res = self.request("
            mutation DeleteVertices($q: InputRootQuery!) {
                delete(q: $q)
            }
        ", vars!(
            "q" => InputValue::object(create_vertex_query(q))
        ), "delete")?;

        Ok(())
    }

    fn get_vertex_count(&self) -> Result<u64, Error> {
        let res = self.request("query { vertexCount }", vars!(), "vertexCount")?;
        Ok(extract_u64(&res))
    }

    fn create_edge(&self, e: &EdgeKey) -> Result<bool, Error> {
        Ok(extract_bool(&self.request("
            mutation CreateEdge($outboundId: ID!, $t: String!, $inboundId: ID!) {
                createEdge(key: {
                    outboundId: $outboundId,
                    t: $t,
                    inboundId: $inboundId
                })
            }
        ", vars!(
            "outboundId" => InputValue::String(e.outbound_id.hyphenated().to_string()),
            "t" => InputValue::String(e.t.0.clone()),
            "inboundId" => InputValue::String(e.inbound_id.hyphenated().to_string())
        ), "createEdge")?))
    }

    fn get_edges(&self, q: &EdgeQuery) -> Result<Vec<Edge>, Error> {
        let res = self.request("
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
        ", vars!(
            "q" => InputValue::object(create_edge_query(q))
        ), "query")?;

        let values = res.as_list_value().expect("Expected a list");
        Ok(values.into_iter().map(extract_edge).collect())
    }

    fn delete_edges(&self, q: &EdgeQuery) -> Result<(), Error> {
        self.request("
            mutation DeleteEdges($q: InputRootQuery!) {
                delete(q: $q)
            }
        ", vars!(
            "q" => InputValue::object(create_edge_query(q))
        ), "delete")?;

        Ok(())
    }

    fn get_edge_count(&self, id: Uuid, type_filter: Option<&Type>, direction: EdgeDirection) -> Result<u64, Error> {
        // let res = self.request("
        //     query GetEdgeCount($id: ID!, typeFilter: String, direction: InputEdgeDirection) {
        //         edgeCount(id: $id, typeFilter: $typeFilter, direction: $direction)
        //     }", vars!(
        //         "id" => InputValue::String(id.hyphenated().to_string()),
        //         "t" => create_optional(&type_filter, |t| InputValue::String(t.0.clone())),
        //         "direction" => InputValue::String(direction.to_string())
        //     ), "getEdgeCount")?;
        // Ok(extract_u64(&res))
        unimplemented!();
    }

    fn get_vertex_metadata(&self, q: &VertexQuery, name: &str) -> Result<Vec<VertexMetadata>, Error> {
        unimplemented!();
    }

    fn set_vertex_metadata(&self, q: &VertexQuery, name: &str, value: &JsonValue) -> Result<(), Error> {
        unimplemented!();
    }

    fn delete_vertex_metadata(&self, q: &VertexQuery, name: &str) -> Result<(), Error> {
        unimplemented!();
    }

    fn get_edge_metadata(&self, q: &EdgeQuery, name: &str) -> Result<Vec<EdgeMetadata>, Error> {
        // let res = self.request("
        //     query GetEdgeMetadata($q: InputRootQuery!) {
        //         query(q: $q)
        //     }
        // ", vars!(
        //     "q" => InputValue::object(create_edge_query(q))
        // ), "query")?;

        // let values = res.as_list_value().expect("Expected a list");
        // Ok(values.into_iter().map(extract_edge_key).collect())
        unimplemented!();
    }

    fn set_edge_metadata(&self, q: &EdgeQuery, name: &str, value: &JsonValue) -> Result<(), Error> {
        unimplemented!();
    }

    fn delete_edge_metadata(&self, q: &EdgeQuery, name: &str) -> Result<(), Error> {
        unimplemented!();
    }
}

#[cfg(feature = "test-suite")]
full_test_impl!(ClientDatastore::default());
