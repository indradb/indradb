//! Trait implementations for conveniently converting between protobuf and
//! native IndraDB models.

use std::convert::TryInto;
use std::error::Error as StdError;
use std::fmt;

use chrono::TimeZone;
use chrono::{DateTime, Utc};
use indradb::ValidationError;
use serde_json::Error as SerdeJsonError;
use uuid::Error as UuidError;
use uuid::Uuid;

/// The error returned if a try into operation fails.
#[derive(Debug)]
pub enum ConversionError {
    Json { inner: SerdeJsonError },
    Uuid { inner: UuidError },
    Validation { inner: ValidationError },
    NoneField { name: String },
    UnexpectedResponseType,
}

impl StdError for ConversionError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            ConversionError::Json { ref inner } => Some(inner),
            ConversionError::Uuid { ref inner } => Some(inner),
            ConversionError::Validation { ref inner } => Some(inner),
            _ => None,
        }
    }
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConversionError::Json { ref inner } => write!(f, "json conversion failed: {}", inner),
            ConversionError::Uuid { ref inner } => write!(f, "uuid conversion failed: {}", inner),
            ConversionError::Validation { ref inner } => write!(f, "validation conversion failed: {}", inner),
            ConversionError::NoneField { ref name } => write!(f, "proto field '{}' should not be none", name),
            ConversionError::UnexpectedResponseType => write!(f, "unexpected response type"),
        }
    }
}

impl From<SerdeJsonError> for ConversionError {
    fn from(err: SerdeJsonError) -> Self {
        ConversionError::Json { inner: err }
    }
}

impl From<UuidError> for ConversionError {
    fn from(err: UuidError) -> Self {
        ConversionError::Uuid { inner: err }
    }
}

impl From<ValidationError> for ConversionError {
    fn from(err: ValidationError) -> Self {
        ConversionError::Validation { inner: err }
    }
}

fn required_field<T>(field_name: &str, value: Option<T>) -> Result<T, ConversionError> {
    value.ok_or_else(|| ConversionError::NoneField {
        name: field_name.to_string(),
    })
}

impl From<Uuid> for crate::Uuid {
    fn from(uuid: Uuid) -> Self {
        crate::Uuid {
            value: uuid.as_bytes().to_vec(),
        }
    }
}

impl TryInto<Uuid> for crate::Uuid {
    type Error = ConversionError;

    fn try_into(self) -> Result<Uuid, Self::Error> {
        Ok(Uuid::from_slice(&self.value)?)
    }
}

impl From<indradb::Identifier> for crate::Identifier {
    fn from(t: indradb::Identifier) -> Self {
        crate::Identifier { value: t.into_string() }
    }
}

impl TryInto<indradb::Identifier> for crate::Identifier {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::Identifier, Self::Error> {
        Ok(indradb::Identifier::new(self.value)?)
    }
}

impl From<serde_json::Value> for crate::Json {
    fn from(value: serde_json::Value) -> Self {
        crate::Json {
            value: value.to_string(),
        }
    }
}

impl TryInto<serde_json::Value> for crate::Json {
    type Error = ConversionError;

    fn try_into(self) -> Result<serde_json::Value, Self::Error> {
        Ok(serde_json::from_str(&self.value)?)
    }
}

impl From<indradb::Edge> for crate::Edge {
    fn from(edge: indradb::Edge) -> Self {
        crate::Edge {
            outbound_id: Some(edge.outbound_id.into()),
            t: Some(edge.t.into()),
            inbound_id: Some(edge.inbound_id.into()),
        }
    }
}

impl TryInto<indradb::Edge> for crate::Edge {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::Edge, Self::Error> {
        Ok(indradb::Edge::new(
            required_field("outbound_id", self.outbound_id)?.try_into()?,
            required_field("t", self.t)?.try_into()?,
            required_field("inbound_id", self.inbound_id)?.try_into()?,
        ))
    }
}

impl From<indradb::Vertex> for crate::Vertex {
    fn from(vertex: indradb::Vertex) -> Self {
        crate::Vertex {
            id: Some(vertex.id.into()),
            t: Some(vertex.t.into()),
        }
    }
}

impl TryInto<indradb::Vertex> for crate::Vertex {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::Vertex, Self::Error> {
        Ok(indradb::Vertex::with_id(
            required_field("id", self.id)?.try_into()?,
            required_field("t", self.t)?.try_into()?,
        ))
    }
}

impl From<indradb::Query> for crate::Query {
    fn from(q: indradb::Query) -> Self {
        crate::Query {
            query: Some(match q {
                indradb::Query::AllVertex => crate::QueryVariant::AllVertex(()),
                indradb::Query::RangeVertex(q) => crate::QueryVariant::RangeVertex(crate::RangeVertexQuery {
                    limit: q.limit,
                    t: q.t.map(|t| t.into()),
                    start_id: q.start_id.map(|start_id| start_id.into()),
                }),
                indradb::Query::SpecificVertex(q) => crate::QueryVariant::SpecificVertex(crate::SpecificVertexQuery {
                    ids: q.ids.into_iter().map(|id| id.into()).collect(),
                }),
                indradb::Query::VertexWithPropertyPresence(q) => {
                    crate::QueryVariant::VertexWithPropertyPresence(crate::VertexWithPropertyPresenceQuery {
                        name: Some(q.name.into()),
                    })
                }
                indradb::Query::VertexWithPropertyValue(q) => {
                    crate::QueryVariant::VertexWithPropertyValue(crate::VertexWithPropertyValueQuery {
                        name: Some(q.name.into()),
                        value: Some(q.value.into()),
                    })
                }

                indradb::Query::AllEdge => crate::QueryVariant::AllEdge(()),
                indradb::Query::SpecificEdge(q) => crate::QueryVariant::SpecificEdge(crate::SpecificEdgeQuery {
                    edges: q.edges.into_iter().map(|id| id.into()).collect(),
                }),
                indradb::Query::EdgeWithPropertyPresence(q) => {
                    crate::QueryVariant::EdgeWithPropertyPresence(crate::EdgeWithPropertyPresenceQuery {
                        name: Some(q.name.into()),
                    })
                }
                indradb::Query::EdgeWithPropertyValue(q) => {
                    crate::QueryVariant::EdgeWithPropertyValue(crate::EdgeWithPropertyValueQuery {
                        name: Some(q.name.into()),
                        value: Some(q.value.into()),
                    })
                }

                indradb::Query::Pipe(q) => {
                    let mut proto_q = crate::PipeQuery {
                        inner: Some(Box::new((*q.inner).into())),
                        direction: 0,
                        limit: q.limit,
                        t: q.t.map(|t| t.into()),
                    };
                    proto_q.set_direction(q.direction.into());
                    crate::QueryVariant::Pipe(Box::new(proto_q))
                }
                indradb::Query::PipeProperty(q) => {
                    let proto_q = crate::PipePropertyQuery {
                        inner: Some(Box::new((*q.inner).into())),
                        name: q.name.map(|name| name.into()),
                    };
                    crate::QueryVariant::PipeProperty(Box::new(proto_q))
                }
                indradb::Query::PipeWithPropertyPresence(q) => {
                    let proto_q = crate::PipeWithPropertyPresenceQuery {
                        inner: Some(Box::new((*q.inner).into())),
                        name: Some(q.name.into()),
                        exists: q.exists,
                    };
                    crate::QueryVariant::PipeWithPropertyPresence(Box::new(proto_q))
                }
                indradb::Query::PipeWithPropertyValue(q) => {
                    let proto_q = crate::PipeWithPropertyValueQuery {
                        inner: Some(Box::new((*q.inner).into())),
                        name: Some(q.name.into()),
                        value: Some(q.value.into()),
                        equal: q.equal,
                    };
                    crate::QueryVariant::PipeWithPropertyValue(Box::new(proto_q))
                }

                indradb::Query::Include(q) => {
                    let proto_q = crate::IncludeQuery {
                        inner: Some(Box::new((*q.inner).into())),
                    };
                    crate::QueryVariant::Include(Box::new(proto_q))
                }
                indradb::Query::Count(q) => {
                    let proto_q = crate::CountQuery {
                        inner: Some(Box::new((*q.inner).into())),
                    };
                    crate::QueryVariant::Count(Box::new(proto_q))
                }
            }),
        }
    }
}

impl TryInto<indradb::Query> for crate::Query {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::Query, Self::Error> {
        Ok(match required_field("query", self.query)? {
            crate::QueryVariant::AllVertex(_q) => indradb::Query::AllVertex,
            crate::QueryVariant::RangeVertex(q) => indradb::Query::RangeVertex(indradb::RangeVertexQuery {
                limit: q.limit,
                t: q.t.map(|t| t.try_into()).transpose()?,
                start_id: q.start_id.map(|start_id| start_id.try_into()).transpose()?,
            }),
            crate::QueryVariant::SpecificVertex(q) => {
                let ids: Result<Vec<Uuid>, ConversionError> = q.ids.into_iter().map(|id| id.try_into()).collect();
                indradb::Query::SpecificVertex(indradb::SpecificVertexQuery { ids: ids? })
            }
            crate::QueryVariant::VertexWithPropertyPresence(q) => {
                let name = required_field("name", q.name)?;
                indradb::Query::VertexWithPropertyPresence(indradb::VertexWithPropertyPresenceQuery {
                    name: name.try_into()?,
                })
            }
            crate::QueryVariant::VertexWithPropertyValue(q) => {
                let name = required_field("name", q.name)?;
                let value = required_field("value", q.value)?;
                indradb::Query::VertexWithPropertyValue(indradb::VertexWithPropertyValueQuery {
                    name: name.try_into()?,
                    value: value.try_into()?,
                })
            }

            crate::QueryVariant::AllEdge(_q) => indradb::Query::AllEdge,
            crate::QueryVariant::SpecificEdge(q) => {
                let edges: Result<Vec<indradb::Edge>, ConversionError> =
                    q.edges.into_iter().map(|id| id.try_into()).collect();
                indradb::Query::SpecificEdge(indradb::SpecificEdgeQuery { edges: edges? })
            }
            crate::QueryVariant::EdgeWithPropertyPresence(q) => {
                let name = required_field("name", q.name)?;
                indradb::Query::EdgeWithPropertyPresence(indradb::EdgeWithPropertyPresenceQuery {
                    name: name.try_into()?,
                })
            }
            crate::QueryVariant::EdgeWithPropertyValue(q) => {
                let name = required_field("name", q.name)?;
                let value = required_field("value", q.value)?;
                indradb::Query::EdgeWithPropertyValue(indradb::EdgeWithPropertyValueQuery {
                    name: name.try_into()?,
                    value: value.try_into()?,
                })
            }

            crate::QueryVariant::Pipe(q) => {
                let direction = q.direction().into();
                let limit = q.limit;
                let t = q.t.map(|t| t.try_into()).transpose()?;
                let inner = required_field("inner", q.inner)?;
                indradb::Query::Pipe(indradb::PipeQuery {
                    direction,
                    limit,
                    t,
                    inner: Box::new((*inner).try_into()?),
                })
            }
            crate::QueryVariant::PipeWithPropertyPresence(q) => {
                let inner = required_field("inner", q.inner)?;
                let name = required_field("name", q.name)?;
                indradb::Query::PipeWithPropertyPresence(indradb::PipeWithPropertyPresenceQuery {
                    inner: Box::new((*inner).try_into()?),
                    name: name.try_into()?,
                    exists: q.exists,
                })
            }
            crate::QueryVariant::PipeWithPropertyValue(q) => {
                let inner = required_field("inner", q.inner)?;
                let name = required_field("name", q.name)?;
                let value = required_field("value", q.value)?;
                indradb::Query::PipeWithPropertyValue(indradb::PipeWithPropertyValueQuery {
                    inner: Box::new((*inner).try_into()?),
                    name: name.try_into()?,
                    value: value.try_into()?,
                    equal: q.equal,
                })
            }
        })
    }
}

impl From<indradb::EdgeDirection> for crate::EdgeDirection {
    fn from(direction: indradb::EdgeDirection) -> Self {
        match direction {
            indradb::EdgeDirection::Outbound => crate::EdgeDirection::Outbound,
            indradb::EdgeDirection::Inbound => crate::EdgeDirection::Inbound,
        }
    }
}

impl From<crate::EdgeDirection> for indradb::EdgeDirection {
    fn from(direction: crate::EdgeDirection) -> Self {
        match direction {
            crate::EdgeDirection::Outbound => indradb::EdgeDirection::Outbound,
            crate::EdgeDirection::Inbound => indradb::EdgeDirection::Inbound,
        }
    }
}

impl From<indradb::NamedProperty> for crate::NamedProperty {
    fn from(prop: indradb::NamedProperty) -> Self {
        crate::NamedProperty {
            name: Some(prop.name.into()),
            value: Some(prop.value.into()),
        }
    }
}

impl TryInto<indradb::NamedProperty> for crate::NamedProperty {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::NamedProperty, Self::Error> {
        Ok(indradb::NamedProperty::new(
            required_field("name", self.name)?.try_into()?,
            required_field("value", self.value)?.try_into()?,
        ))
    }
}

impl From<indradb::VertexProperty> for crate::VertexProperty {
    fn from(prop: indradb::VertexProperty) -> Self {
        crate::VertexProperty {
            id: Some(prop.id.into()),
            value: Some(prop.value.into()),
        }
    }
}

impl TryInto<indradb::VertexProperty> for crate::VertexProperty {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::VertexProperty, Self::Error> {
        Ok(indradb::VertexProperty::new(
            required_field("id", self.id)?.try_into()?,
            required_field("value", self.value)?.try_into()?,
        ))
    }
}

impl From<indradb::VertexProperties> for crate::VertexProperties {
    fn from(props: indradb::VertexProperties) -> Self {
        crate::VertexProperties {
            vertex: Some(props.vertex.into()),
            props: props.props.into_iter().map(|prop| prop.into()).collect(),
        }
    }
}

impl TryInto<indradb::VertexProperties> for crate::VertexProperties {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::VertexProperties, Self::Error> {
        let props: Result<Vec<indradb::NamedProperty>, ConversionError> =
            self.props.into_iter().map(|prop| prop.try_into()).collect();
        Ok(indradb::VertexProperties::new(
            required_field("vertex", self.vertex)?.try_into()?,
            props?,
        ))
    }
}

impl From<indradb::EdgeProperty> for crate::EdgeProperty {
    fn from(prop: indradb::EdgeProperty) -> Self {
        crate::EdgeProperty {
            edge: Some(prop.edge.into()),
            value: Some(prop.value.into()),
        }
    }
}

impl TryInto<indradb::EdgeProperty> for crate::EdgeProperty {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::EdgeProperty, Self::Error> {
        Ok(indradb::EdgeProperty::new(
            required_field("edge", self.edge)?.try_into()?,
            required_field("value", self.value)?.try_into()?,
        ))
    }
}

impl From<indradb::EdgeProperties> for crate::EdgeProperties {
    fn from(props: indradb::EdgeProperties) -> Self {
        crate::EdgeProperties {
            edge: Some(props.edge.into()),
            props: props.props.into_iter().map(|prop| prop.into()).collect(),
        }
    }
}

impl TryInto<indradb::EdgeProperties> for crate::EdgeProperties {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::EdgeProperties, Self::Error> {
        let props: Result<Vec<indradb::NamedProperty>, ConversionError> =
            self.props.into_iter().map(|prop| prop.try_into()).collect();
        Ok(indradb::EdgeProperties::new(
            required_field("edge", self.edge)?.try_into()?,
            props?,
        ))
    }
}

impl From<indradb::BulkInsertItem> for crate::BulkInsertItem {
    fn from(item: indradb::BulkInsertItem) -> Self {
        crate::BulkInsertItem {
            item: Some(match item {
                indradb::BulkInsertItem::Vertex(vertex) => crate::BulkInsertItemVariant::Vertex(vertex.into()),
                indradb::BulkInsertItem::Edge(edge) => crate::BulkInsertItemVariant::Edge(edge.into()),
                indradb::BulkInsertItem::VertexProperty(id, name, value) => {
                    crate::BulkInsertItemVariant::VertexProperty(crate::VertexPropertyBulkInsertItem {
                        id: Some(id.into()),
                        name: Some(name.into()),
                        value: Some(value.into()),
                    })
                }
                indradb::BulkInsertItem::EdgeProperty(edge, name, value) => {
                    crate::BulkInsertItemVariant::EdgeProperty(crate::EdgePropertyBulkInsertItem {
                        edge: Some(edge.into()),
                        name: Some(name.into()),
                        value: Some(value.into()),
                    })
                }
            }),
        }
    }
}

impl TryInto<indradb::BulkInsertItem> for crate::BulkInsertItem {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::BulkInsertItem, Self::Error> {
        Ok(match required_field("item", self.item)? {
            crate::BulkInsertItemVariant::Vertex(vertex) => indradb::BulkInsertItem::Vertex(vertex.try_into()?),
            crate::BulkInsertItemVariant::Edge(edge) => indradb::BulkInsertItem::Edge(edge.try_into()?),
            crate::BulkInsertItemVariant::VertexProperty(item) => indradb::BulkInsertItem::VertexProperty(
                required_field("id", item.id)?.try_into()?,
                required_field("name", item.name)?.try_into()?,
                required_field("value", item.value)?.try_into()?,
            ),
            crate::BulkInsertItemVariant::EdgeProperty(item) => indradb::BulkInsertItem::EdgeProperty(
                required_field("edge", item.edge)?.try_into()?,
                required_field("name", item.name)?.try_into()?,
                required_field("value", item.value)?.try_into()?,
            ),
        })
    }
}

impl TryInto<indradb::Identifier> for crate::IndexPropertyRequest {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::Identifier, Self::Error> {
        let name = required_field("name", self.name)?.try_into()?;
        Ok(name)
    }
}

impl TryInto<(indradb::Query, indradb::Identifier, serde_json::Value)> for crate::SetPropertiesRequest {
    type Error = ConversionError;

    fn try_into(self) -> Result<(indradb::Query, indradb::Identifier, serde_json::Value), Self::Error> {
        let q = required_field("q", self.q)?.try_into()?;
        let name = required_field("name", self.name)?.try_into()?;
        let value = required_field("value", self.value)?.try_into()?;
        Ok((q, name, value))
    }
}

impl From<(indradb::Query, indradb::Identifier, serde_json::Value)> for crate::SetPropertiesRequest {
    fn from(value: (indradb::Query, indradb::Identifier, serde_json::Value)) -> Self {
        crate::SetPropertiesRequest {
            q: Some(value.0.into()),
            name: Some(value.1.into()),
            value: Some(value.2.into()),
        }
    }
}

fn to_chrono_time(ts: prost_types::Timestamp) -> DateTime<Utc> {
    Utc.timestamp_opt(ts.seconds, ts.nanos as u32).unwrap()
}

fn to_proto_time(dt: &DateTime<Utc>) -> prost_types::Timestamp {
    prost_types::Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as i32,
    }
}
