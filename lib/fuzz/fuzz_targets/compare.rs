#![no_main]

use std::collections::HashMap;

use arbitrary::{Arbitrary, Unstructured};
use indradb::{MemoryDatastore, RocksdbDatastore};
use libfuzzer_sys::fuzz_target;
use tempfile::tempdir;

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub enum Op {
    BulkInsert(Vec<BulkInsertItem>),
    CreateVertex(Vertex),
    Get(Query),
    Delete(Query),
    CreateEdge(Edge),
    SetProperties(Query, Identifier, Json),
    IndexProperty(Identifier),
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub enum BulkInsertItem {
    Vertex(Vertex),
    Edge(Edge),
    VertexProperty(u64, Identifier, Json),
    EdgeProperty(Edge, Identifier, Json),
}

impl Into<indradb::BulkInsertItem> for BulkInsertItem {
    fn into(self) -> indradb::BulkInsertItem {
        match self {
            BulkInsertItem::Vertex(vertex) => indradb::BulkInsertItem::Vertex(vertex.into()),
            BulkInsertItem::Edge(edge) => indradb::BulkInsertItem::Edge(edge.into()),
            BulkInsertItem::VertexProperty(id, name, value) => {
                indradb::BulkInsertItem::VertexProperty(id.into(), name.into(), value.into())
            }
            BulkInsertItem::EdgeProperty(edge, name, value) => {
                indradb::BulkInsertItem::EdgeProperty(edge.into(), name.into(), value.into())
            }
        }
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub struct Vertex {
    pub id: u64,
    pub t: Identifier,
}

impl Into<indradb::Vertex> for Vertex {
    fn into(self) -> indradb::Vertex {
        indradb::Vertex::with_id(self.id.into(), self.t.into())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier(indradb::Identifier);

impl<'a> Arbitrary<'a> for Identifier {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let s: String = u.arbitrary()?;

        if s.is_empty() {
            return Err(arbitrary::Error::NotEnoughData);
        }

        let t = indradb::Identifier::new(s).map_err(|_| arbitrary::Error::IncorrectFormat)?;
        Ok(Self { 0: t })
    }
}

impl Into<indradb::Identifier> for Identifier {
    fn into(self) -> indradb::Identifier {
        self.0
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub enum EdgeDirection {
    Outbound,
    Inbound,
}

impl Into<indradb::EdgeDirection> for EdgeDirection {
    fn into(self) -> indradb::EdgeDirection {
        match self {
            EdgeDirection::Outbound => indradb::EdgeDirection::Outbound,
            EdgeDirection::Inbound => indradb::EdgeDirection::Inbound,
        }
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub enum Query {
    AllVertex,
    RangeVertex(RangeVertexQuery),
    SpecificVertex(SpecificVertexQuery),
    VertexWithPropertyPresence(VertexWithPropertyPresenceQuery),
    VertexWithPropertyValue(VertexWithPropertyValueQuery),

    AllEdge,
    SpecificEdge(SpecificEdgeQuery),
    EdgeWithPropertyPresence(EdgeWithPropertyPresenceQuery),
    EdgeWithPropertyValue(EdgeWithPropertyValueQuery),

    Pipe(PipeQuery),
    PipeProperty(PipePropertyQuery),
    PipeWithPropertyPresence(PipeWithPropertyPresenceQuery),
    PipeWithPropertyValue(PipeWithPropertyValueQuery),

    Include(IncludeQuery),
    Count(CountQuery),
}

impl Into<indradb::Query> for Query {
    fn into(self) -> indradb::Query {
        match self {
            Query::AllVertex => indradb::Query::AllVertex,
            Query::RangeVertex(q) => indradb::Query::RangeVertex(q.into()),
            Query::SpecificVertex(q) => indradb::Query::SpecificVertex(q.into()),
            Query::VertexWithPropertyPresence(q) => indradb::Query::VertexWithPropertyPresence(q.into()),
            Query::VertexWithPropertyValue(q) => indradb::Query::VertexWithPropertyValue(q.into()),

            Query::AllEdge => indradb::Query::AllEdge,
            Query::SpecificEdge(specific) => indradb::Query::SpecificEdge(specific.into()),
            Query::EdgeWithPropertyPresence(q) => indradb::Query::EdgeWithPropertyPresence(q.into()),
            Query::EdgeWithPropertyValue(q) => indradb::Query::EdgeWithPropertyValue(q.into()),

            Query::Pipe(q) => indradb::Query::Pipe(q.into()),
            Query::PipeProperty(q) => indradb::Query::PipeProperty(q.into()),
            Query::PipeWithPropertyPresence(q) => indradb::Query::PipeWithPropertyPresence(q.into()),
            Query::PipeWithPropertyValue(q) => indradb::Query::PipeWithPropertyValue(q.into()),

            Query::Include(q) => indradb::Query::Include(q.into()),
            Query::Count(q) => indradb::Query::Count(q.into()),
        }
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub struct RangeVertexQuery {
    pub limit: u32,
    pub t: Option<Identifier>,
    pub start_id: Option<u64>,
}

impl Into<indradb::RangeVertexQuery> for RangeVertexQuery {
    fn into(self) -> indradb::RangeVertexQuery {
        indradb::RangeVertexQuery {
            limit: self.limit,
            t: self.t.map(|t| t.into()),
            start_id: self.start_id.map(|id| id.into()),
        }
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub struct SpecificVertexQuery {
    pub ids: Vec<u64>,
}

impl Into<indradb::SpecificVertexQuery> for SpecificVertexQuery {
    fn into(self) -> indradb::SpecificVertexQuery {
        indradb::SpecificVertexQuery {
            ids: self.ids.into_iter().map(|i| i.into()).collect(),
        }
    }
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub struct VertexWithPropertyPresenceQuery {
    pub name: Identifier,
}

impl Into<indradb::VertexWithPropertyPresenceQuery> for VertexWithPropertyPresenceQuery {
    fn into(self) -> indradb::VertexWithPropertyPresenceQuery {
        indradb::VertexWithPropertyPresenceQuery {
            name: self.name.into(),
        }
    }
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub struct VertexWithPropertyValueQuery {
    pub name: Identifier,
    pub value: Json,
}

impl Into<indradb::VertexWithPropertyValueQuery> for VertexWithPropertyValueQuery {
    fn into(self) -> indradb::VertexWithPropertyValueQuery {
        indradb::VertexWithPropertyValueQuery {
            name: self.name.into(),
            value: self.value.into(),
        }
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub struct SpecificEdgeQuery {
    pub edges: Vec<Edge>,
}

impl Into<indradb::SpecificEdgeQuery> for SpecificEdgeQuery {
    fn into(self) -> indradb::SpecificEdgeQuery {
        indradb::SpecificEdgeQuery {
            edges: self.edges.into_iter().map(|e| e.into()).collect(),
        }
    }
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub struct EdgeWithPropertyPresenceQuery {
    pub name: Identifier,
}

impl Into<indradb::EdgeWithPropertyPresenceQuery> for EdgeWithPropertyPresenceQuery {
    fn into(self) -> indradb::EdgeWithPropertyPresenceQuery {
        indradb::EdgeWithPropertyPresenceQuery {
            name: self.name.into(),
        }
    }
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub struct EdgeWithPropertyValueQuery {
    pub name: Identifier,
    pub value: Json,
}

impl Into<indradb::EdgeWithPropertyValueQuery> for EdgeWithPropertyValueQuery {
    fn into(self) -> indradb::EdgeWithPropertyValueQuery {
        indradb::EdgeWithPropertyValueQuery {
            name: self.name.into(),
            value: self.value.into(),
        }
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub struct PipeQuery {
    pub inner: Box<Query>,
    pub direction: EdgeDirection,
    pub limit: u32,
    pub t: Option<Identifier>,
}

impl Into<indradb::PipeQuery> for PipeQuery {
    fn into(self) -> indradb::PipeQuery {
        indradb::PipeQuery {
            inner: Box::new((*self.inner).into()),
            direction: self.direction.into(),
            limit: self.limit,
            t: self.t.map(|t| t.into()),
        }
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub struct PipePropertyQuery {
    pub inner: Box<Query>,
    pub name: Option<Identifier>,
}

impl Into<indradb::PipePropertyQuery> for PipePropertyQuery {
    fn into(self) -> indradb::PipePropertyQuery {
        indradb::PipePropertyQuery {
            inner: Box::new((*self.inner).into()),
            name: self.name.map(|n| n.into()),
        }
    }
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub struct PipeWithPropertyPresenceQuery {
    pub inner: Box<Query>,
    pub name: Identifier,
    pub exists: bool,
}

impl Into<indradb::PipeWithPropertyPresenceQuery> for PipeWithPropertyPresenceQuery {
    fn into(self) -> indradb::PipeWithPropertyPresenceQuery {
        indradb::PipeWithPropertyPresenceQuery {
            inner: Box::new((*self.inner).into()),
            name: self.name.into(),
            exists: self.exists,
        }
    }
}

#[derive(Arbitrary, PartialEq, Clone, Debug)]
pub struct PipeWithPropertyValueQuery {
    pub inner: Box<Query>,
    pub name: Identifier,
    pub value: Json,
    pub equal: bool,
}

impl Into<indradb::PipeWithPropertyValueQuery> for PipeWithPropertyValueQuery {
    fn into(self) -> indradb::PipeWithPropertyValueQuery {
        indradb::PipeWithPropertyValueQuery {
            inner: Box::new((*self.inner).into()),
            name: self.name.into(),
            value: self.value.into(),
            equal: self.equal,
        }
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub struct IncludeQuery {
    pub inner: Box<Query>,
}

impl Into<indradb::IncludeQuery> for IncludeQuery {
    fn into(self) -> indradb::IncludeQuery {
        indradb::IncludeQuery {
            inner: Box::new((*self.inner).into()),
        }
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub struct CountQuery {
    pub inner: Box<Query>,
}

impl Into<indradb::CountQuery> for CountQuery {
    fn into(self) -> indradb::CountQuery {
        indradb::CountQuery {
            inner: Box::new((*self.inner).into()),
        }
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub struct VertexProperty {
    pub id: u64,
    pub value: Json,
}

impl Into<indradb::VertexProperty> for VertexProperty {
    fn into(self) -> indradb::VertexProperty {
        indradb::VertexProperty {
            id: self.id.into(),
            value: self.value.into(),
        }
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub struct NamedProperty {
    pub name: Identifier,
    pub value: Json,
}

impl Into<indradb::NamedProperty> for NamedProperty {
    fn into(self) -> indradb::NamedProperty {
        indradb::NamedProperty {
            name: self.name.into(),
            value: self.value.into(),
        }
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub struct VertexProperties {
    pub vertex: Vertex,
    pub props: Vec<NamedProperty>,
}

impl Into<indradb::VertexProperties> for VertexProperties {
    fn into(self) -> indradb::VertexProperties {
        indradb::VertexProperties {
            vertex: self.vertex.into(),
            props: self.props.into_iter().map(|p| p.into()).collect(),
        }
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub struct EdgeProperties {
    pub edge: Edge,
    pub props: Vec<NamedProperty>,
}

impl Into<indradb::EdgeProperties> for EdgeProperties {
    fn into(self) -> indradb::EdgeProperties {
        indradb::EdgeProperties {
            edge: self.edge.into(),
            props: self.props.into_iter().map(|p| p.into()).collect(),
        }
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub struct EdgeProperty {
    pub edge: Edge,
    pub value: Json,
}

impl Into<indradb::EdgeProperty> for EdgeProperty {
    fn into(self) -> indradb::EdgeProperty {
        indradb::EdgeProperty {
            edge: self.edge.into(),
            value: self.value.into(),
        }
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub struct Edge {
    pub outbound_id: u64,
    pub t: Identifier,
    pub inbound_id: u64,
}

impl Into<indradb::Edge> for Edge {
    fn into(self) -> indradb::Edge {
        indradb::Edge {
            outbound_id: self.outbound_id.into(),
            t: self.t.into(),
            inbound_id: self.inbound_id.into(),
        }
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Number(JsonNumber),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

impl Into<serde_json::Value> for Json {
    fn into(self) -> serde_json::Value {
        match self {
            Json::Null => serde_json::Value::Null,
            Json::Bool(b) => serde_json::Value::Bool(b),
            Json::Number(n) => serde_json::Value::Number(n.into()),
            Json::String(s) => serde_json::Value::String(s),
            Json::Array(v) => serde_json::Value::Array(v.into_iter().map(|i| i.into()).collect()),
            Json::Object(o) => {
                let mut m = serde_json::Map::new();

                for (k, v) in o.into_iter() {
                    m.insert(k, v.into());
                }

                serde_json::Value::Object(m)
            }
        }
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
pub enum JsonNumber {
    PosInt(u64),
    NegInt(i64),
    Float(FiniteFloat),
}

impl Into<serde_json::Number> for JsonNumber {
    fn into(self) -> serde_json::Number {
        match self {
            JsonNumber::PosInt(n) => n.into(),
            JsonNumber::NegInt(n) => n.into(),
            JsonNumber::Float(n) => serde_json::Number::from_f64(n.into()).unwrap(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FiniteFloat(f64);

impl<'a> Arbitrary<'a> for FiniteFloat {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let f: f64 = u.arbitrary()?;

        if f.is_finite() {
            Ok(Self { 0: f })
        } else {
            Err(arbitrary::Error::IncorrectFormat)
        }
    }
}

impl Into<f64> for FiniteFloat {
    fn into(self) -> f64 {
        self.0
    }
}

macro_rules! cmp {
    ($v1:expr, $v2:expr) => {
        match ($v1, $v2) {
            (Ok(v1), Ok(v2)) => {
                assert_eq!(v1, v2);
            }
            (v1, v2) => {
                assert_eq!(format!("{:?}", v1), format!("{:?}", v2));
            }
        }
    };
}

fuzz_target!(|ops: Vec<Op>| {
    let d1 = MemoryDatastore::new_db();
    let rocksdb_dir = tempdir().unwrap();
    let d2 = RocksdbDatastore::new_db(rocksdb_dir.path(), Some(1)).unwrap();

    for op in ops {
        match op {
            Op::BulkInsert(items) => {
                let items: Vec<indradb::BulkInsertItem> = items.into_iter().map(|i| i.into()).collect();
                let v1 = d1.bulk_insert(items.clone());
                let v2 = d2.bulk_insert(items);
                cmp!(v1, v2);
            }
            Op::CreateVertex(vertex) => {
                let vertex = vertex.into();
                let v1 = d1.create_vertex(&vertex);
                let v2 = d2.create_vertex(&vertex);
                cmp!(v1, v2);
            }
            Op::Get(q) => {
                let q: indradb::Query = q.into();
                let o1 = d1.get(q.clone());
                let o2 = d2.get(q);
                cmp!(o1, o2);
            }
            Op::Delete(q) => {
                let q: indradb::Query = q.into();
                let o1 = d1.delete(q.clone());
                let o2 = d2.delete(q);
                cmp!(o1, o2);
            }
            Op::CreateEdge(edge) => {
                let edge: indradb::Edge = edge.into();
                let v1 = d1.create_edge(&edge);
                let v2 = d2.create_edge(&edge);
                cmp!(v1, v2);
            }
            Op::SetProperties(q, name, value) => {
                let q: indradb::Query = q.into();
                let name: indradb::Identifier = name.into();
                let value: serde_json::Value = value.into();
                let v1 = d1.set_properties(q.clone(), name.clone(), value.clone());
                let v2 = d2.set_properties(q, name.clone(), value);
                cmp!(v1, v2);
            }
            Op::IndexProperty(t) => {
                let v1 = d1.index_property(t.clone().into());
                let v2 = d2.index_property(t.into());
                cmp!(v1, v2);
            }
        }
    }
});
