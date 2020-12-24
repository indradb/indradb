//! Converts between protobuf and native IndraDB models.

use std::convert::TryInto;

use chrono::TimeZone;
use chrono::{DateTime, Utc};
use indradb::ValidationError;
use serde_json::Error as JsonError;
use serde_json::Value as JsonValue;
use uuid::Error as UuidError;
use uuid::Uuid;

#[derive(Debug, Fail)]
pub enum ConversionError {
    #[fail(display = "json error: {}", inner)]
    Json { inner: JsonError },
    #[fail(display = "uuid error: {}", inner)]
    Uuid { inner: UuidError },
    #[fail(display = "validation error: {}", inner)]
    Validation { inner: ValidationError },
    #[fail(display = "proto field '{}' should not be none", name)]
    NoneField { name: String },
    #[fail(display = "unexpected response type")]
    UnexpectedResponseType,
}

impl From<JsonError> for ConversionError {
    fn from(err: JsonError) -> Self {
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

impl From<indradb::Type> for crate::Type {
    fn from(t: indradb::Type) -> Self {
        crate::Type { value: t.0 }
    }
}

impl TryInto<indradb::Type> for crate::Type {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::Type, Self::Error> {
        Ok(indradb::Type::new(self.value)?)
    }
}

impl From<JsonValue> for crate::Json {
    fn from(value: JsonValue) -> Self {
        crate::Json {
            value: value.to_string(),
        }
    }
}

impl TryInto<JsonValue> for crate::Json {
    type Error = ConversionError;

    fn try_into(self) -> Result<JsonValue, Self::Error> {
        Ok(serde_json::from_str(&self.value)?)
    }
}

impl From<indradb::Edge> for crate::Edge {
    fn from(edge: indradb::Edge) -> Self {
        crate::Edge {
            key: Some(edge.key.into()),
            created_datetime: Some(to_proto_time(&edge.created_datetime)),
        }
    }
}

impl TryInto<indradb::Edge> for crate::Edge {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::Edge, Self::Error> {
        Ok(indradb::Edge::new(
            required_field("key", self.key)?.try_into()?,
            to_chrono_time(required_field("created_datetime", self.created_datetime)?),
        ))
    }
}

impl From<indradb::EdgeKey> for crate::EdgeKey {
    fn from(key: indradb::EdgeKey) -> Self {
        crate::EdgeKey {
            outbound_id: Some(key.outbound_id.into()),
            t: Some(key.t.into()),
            inbound_id: Some(key.inbound_id.into()),
        }
    }
}

impl TryInto<indradb::EdgeKey> for crate::EdgeKey {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::EdgeKey, Self::Error> {
        Ok(indradb::EdgeKey::new(
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

impl From<indradb::VertexQuery> for crate::VertexQuery {
    fn from(q: indradb::VertexQuery) -> Self {
        crate::VertexQuery {
            query: Some(match q {
                indradb::VertexQuery::Range(q) => crate::VertexQueryVariant::Range(crate::RangeVertexQuery {
                    limit: q.limit,
                    t: q.t.map(|t| t.into()),
                    start_id: q.start_id.map(|start_id| start_id.into()),
                }),
                indradb::VertexQuery::Specific(q) => crate::VertexQueryVariant::Specific(crate::SpecificVertexQuery {
                    ids: q.ids.into_iter().map(|id| id.into()).collect(),
                }),
                indradb::VertexQuery::Pipe(q) => {
                    let mut proto_q = crate::PipeVertexQuery {
                        inner: Some(Box::new((*q.inner).into())),
                        direction: 0,
                        limit: q.limit,
                        t: q.t.map(|t| t.into()),
                    };
                    proto_q.set_direction(q.direction.into());
                    crate::VertexQueryVariant::Pipe(Box::new(proto_q))
                }
            }),
        }
    }
}

impl TryInto<indradb::VertexQuery> for crate::VertexQuery {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::VertexQuery, Self::Error> {
        Ok(match required_field("query", self.query)? {
            crate::VertexQueryVariant::Range(q) => indradb::VertexQuery::Range(indradb::RangeVertexQuery {
                limit: q.limit,
                t: q.t.map(|t| t.try_into()).transpose()?,
                start_id: q.start_id.map(|start_id| start_id.try_into()).transpose()?,
            }),
            crate::VertexQueryVariant::Specific(q) => {
                let ids: Result<Vec<Uuid>, ConversionError> = q.ids.into_iter().map(|id| id.try_into()).collect();
                indradb::VertexQuery::Specific(indradb::SpecificVertexQuery { ids: ids? })
            }
            crate::VertexQueryVariant::Pipe(q) => {
                let direction = q.direction().into();
                let limit = q.limit;
                let t = q.t.map(|t| t.try_into()).transpose()?;
                let inner = required_field("inner", q.inner)?;
                indradb::VertexQuery::Pipe(indradb::PipeVertexQuery {
                    direction,
                    limit,
                    t,
                    inner: Box::new((*inner).try_into()?),
                })
            }
        })
    }
}

impl From<indradb::VertexPropertyQuery> for crate::VertexPropertyQuery {
    fn from(q: indradb::VertexPropertyQuery) -> Self {
        crate::VertexPropertyQuery {
            inner: Some(q.inner.into()),
            name: q.name,
        }
    }
}

impl TryInto<indradb::VertexPropertyQuery> for crate::VertexPropertyQuery {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::VertexPropertyQuery, Self::Error> {
        Ok(indradb::VertexPropertyQuery::new(
            required_field("inner", self.inner)?.try_into()?,
            self.name,
        ))
    }
}

impl From<indradb::EdgeQuery> for crate::EdgeQuery {
    fn from(q: indradb::EdgeQuery) -> Self {
        crate::EdgeQuery {
            query: Some(match q {
                indradb::EdgeQuery::Specific(q) => crate::EdgeQueryVariant::Specific(crate::SpecificEdgeQuery {
                    keys: q.keys.into_iter().map(|key| key.into()).collect(),
                }),
                indradb::EdgeQuery::Pipe(q) => {
                    let mut proto_q = crate::PipeEdgeQuery {
                        inner: Some(Box::new((*q.inner).into())),
                        direction: 0,
                        t: q.t.map(|t| t.into()),
                        high: q.high.map(|t| to_proto_time(&t)),
                        low: q.low.map(|t| to_proto_time(&t)),
                        limit: q.limit,
                    };
                    proto_q.set_direction(q.direction.into());
                    crate::EdgeQueryVariant::Pipe(Box::new(proto_q))
                }
            }),
        }
    }
}

impl TryInto<indradb::EdgeQuery> for crate::EdgeQuery {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::EdgeQuery, Self::Error> {
        Ok(match required_field("query", self.query)? {
            crate::EdgeQueryVariant::Specific(q) => {
                let keys: Result<Vec<indradb::EdgeKey>, ConversionError> =
                    q.keys.into_iter().map(|id| id.try_into()).collect();
                indradb::EdgeQuery::Specific(indradb::SpecificEdgeQuery { keys: keys? })
            }
            crate::EdgeQueryVariant::Pipe(q) => {
                let direction = q.direction().into();
                let t = q.t.map(|t| t.try_into()).transpose()?;
                let high = q.high.map(to_chrono_time);
                let low = q.low.map(to_chrono_time);
                let limit = q.limit;
                let inner: Box<crate::VertexQuery> = required_field("inner", q.inner)?;
                indradb::EdgeQuery::Pipe(indradb::PipeEdgeQuery {
                    direction,
                    t,
                    high,
                    low,
                    limit,
                    inner: Box::new((*inner).try_into()?),
                })
            }
        })
    }
}

impl From<indradb::EdgePropertyQuery> for crate::EdgePropertyQuery {
    fn from(q: indradb::EdgePropertyQuery) -> Self {
        crate::EdgePropertyQuery {
            inner: Some(q.inner.into()),
            name: q.name,
        }
    }
}

impl TryInto<indradb::EdgePropertyQuery> for crate::EdgePropertyQuery {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::EdgePropertyQuery, Self::Error> {
        Ok(indradb::EdgePropertyQuery::new(
            required_field("inner", self.inner)?.try_into()?,
            self.name,
        ))
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

impl Into<indradb::EdgeDirection> for crate::EdgeDirection {
    fn into(self) -> indradb::EdgeDirection {
        match self {
            crate::EdgeDirection::Outbound => indradb::EdgeDirection::Outbound,
            crate::EdgeDirection::Inbound => indradb::EdgeDirection::Inbound,
        }
    }
}

impl From<indradb::NamedProperty> for crate::NamedProperty {
    fn from(prop: indradb::NamedProperty) -> Self {
        crate::NamedProperty {
            name: prop.name,
            value: Some(prop.value.into()),
        }
    }
}

impl TryInto<indradb::NamedProperty> for crate::NamedProperty {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::NamedProperty, Self::Error> {
        Ok(indradb::NamedProperty::new(
            self.name,
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
            key: Some(prop.key.into()),
            value: Some(prop.value.into()),
        }
    }
}

impl TryInto<indradb::EdgeProperty> for crate::EdgeProperty {
    type Error = ConversionError;

    fn try_into(self) -> Result<indradb::EdgeProperty, Self::Error> {
        Ok(indradb::EdgeProperty::new(
            required_field("key", self.key)?.try_into()?,
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
                indradb::BulkInsertItem::Edge(key) => crate::BulkInsertItemVariant::Edge(key.into()),
                indradb::BulkInsertItem::VertexProperty(id, name, value) => {
                    crate::BulkInsertItemVariant::VertexProperty(crate::VertexPropertyBulkInsertItem {
                        id: Some(id.into()),
                        name,
                        value: Some(value.into()),
                    })
                }
                indradb::BulkInsertItem::EdgeProperty(key, name, value) => {
                    crate::BulkInsertItemVariant::EdgeProperty(crate::EdgePropertyBulkInsertItem {
                        key: Some(key.into()),
                        name,
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
            crate::BulkInsertItemVariant::Edge(key) => indradb::BulkInsertItem::Edge(key.try_into()?),
            crate::BulkInsertItemVariant::VertexProperty(item) => indradb::BulkInsertItem::VertexProperty(
                required_field("id", item.id)?.try_into()?,
                item.name,
                required_field("value", item.value)?.try_into()?,
            ),
            crate::BulkInsertItemVariant::EdgeProperty(item) => indradb::BulkInsertItem::EdgeProperty(
                required_field("key", item.key)?.try_into()?,
                item.name,
                required_field("value", item.value)?.try_into()?,
            ),
        })
    }
}

impl TryInto<(Uuid, Option<indradb::Type>, indradb::EdgeDirection)> for crate::GetEdgeCountRequest {
    type Error = ConversionError;

    fn try_into(self) -> Result<(Uuid, Option<indradb::Type>, indradb::EdgeDirection), Self::Error> {
        let direction = self.direction().into();
        let id = required_field("id", self.id)?.try_into()?;
        let t = self.t.map(|t| t.try_into()).transpose()?;
        Ok((id, t, direction))
    }
}

impl From<(Uuid, Option<indradb::Type>, indradb::EdgeDirection)> for crate::GetEdgeCountRequest {
    fn from(value: (Uuid, Option<indradb::Type>, indradb::EdgeDirection)) -> Self {
        let direction: crate::EdgeDirection = value.2.into();
        crate::GetEdgeCountRequest {
            id: Some(value.0.into()),
            t: value.1.map(|t| t.into()),
            direction: direction as i32,
        }
    }
}

impl TryInto<(indradb::VertexPropertyQuery, JsonValue)> for crate::SetVertexPropertiesRequest {
    type Error = ConversionError;

    fn try_into(self) -> Result<(indradb::VertexPropertyQuery, JsonValue), Self::Error> {
        let q = required_field("q", self.q)?.try_into()?;
        let value = required_field("value", self.value)?.try_into()?;
        Ok((q, value))
    }
}

impl From<(indradb::VertexPropertyQuery, JsonValue)> for crate::SetVertexPropertiesRequest {
    fn from(value: (indradb::VertexPropertyQuery, JsonValue)) -> Self {
        crate::SetVertexPropertiesRequest {
            q: Some(value.0.into()),
            value: Some(value.1.into()),
        }
    }
}

impl TryInto<(indradb::EdgePropertyQuery, JsonValue)> for crate::SetEdgePropertiesRequest {
    type Error = ConversionError;

    fn try_into(self) -> Result<(indradb::EdgePropertyQuery, JsonValue), Self::Error> {
        let q = required_field("q", self.q)?.try_into()?;
        let value = required_field("value", self.value)?.try_into()?;
        Ok((q, value))
    }
}

impl From<(indradb::EdgePropertyQuery, JsonValue)> for crate::SetEdgePropertiesRequest {
    fn from(value: (indradb::EdgePropertyQuery, JsonValue)) -> Self {
        crate::SetEdgePropertiesRequest {
            q: Some(value.0.into()),
            value: Some(value.1.into()),
        }
    }
}

macro_rules! convert_transaction_response {
    ($variant:ident, $ty:ty) => {
        impl TryInto<$ty> for crate::TransactionResponseVariant {
            type Error = ConversionError;

            fn try_into(self) -> Result<$ty, Self::Error> {
                if let crate::TransactionResponseVariant::$variant(value) = self {
                    Ok(value)
                } else {
                    Err(Self::Error::UnexpectedResponseType)
                }
            }
        }
    };
}

macro_rules! convert_errorable_transaction_response {
    ($variant:ident, $ty:ty) => {
        impl TryInto<$ty> for crate::TransactionResponseVariant {
            type Error = ConversionError;

            fn try_into(self) -> Result<$ty, Self::Error> {
                if let crate::TransactionResponseVariant::$variant(value) = self {
                    value.try_into()
                } else {
                    Err(Self::Error::UnexpectedResponseType)
                }
            }
        }
    };
}

convert_transaction_response!(Empty, ());
convert_transaction_response!(Ok, bool);
convert_transaction_response!(Count, u64);
convert_errorable_transaction_response!(Id, Uuid);
convert_errorable_transaction_response!(Vertex, indradb::Vertex);
convert_errorable_transaction_response!(Edge, indradb::Edge);
convert_errorable_transaction_response!(VertexProperty, indradb::VertexProperty);
convert_errorable_transaction_response!(VertexProperties, indradb::VertexProperties);
convert_errorable_transaction_response!(EdgeProperty, indradb::EdgeProperty);
convert_errorable_transaction_response!(EdgeProperties, indradb::EdgeProperties);

fn to_chrono_time(ts: prost_types::Timestamp) -> DateTime<Utc> {
    Utc.timestamp(ts.seconds, ts.nanos as u32)
}

fn to_proto_time(dt: &DateTime<Utc>) -> prost_types::Timestamp {
    prost_types::Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as i32,
    }
}
