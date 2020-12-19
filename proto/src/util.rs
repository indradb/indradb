//! Converts between cnp and native IndraDB models

use std::convert::TryInto;
use std::fmt::Display;
use std::vec::IntoIter;

use capnp::Error as CapnpError;
use uuid::Uuid;

pub fn map_capnp_err<T, E: Display>(result: Result<T, E>) -> Result<T, capnp::Error> {
    result.map_err(|err| capnp::Error::failed(format!("{}", err)))
}

pub fn from_vertex<'a>(vertex: &indradb::Vertex, mut builder: crate::vertex::Builder<'a>) {
    builder.set_id(vertex.id.as_bytes());
    builder.set_t(&vertex.t.0);
}

pub fn to_vertex<'a>(reader: &crate::vertex::Reader<'a>) -> Result<indradb::Vertex, CapnpError> {
    let id = map_capnp_err(Uuid::from_slice(reader.get_id()?))?;
    let t = map_capnp_err(indradb::Type::new(reader.get_t()?))?;
    Ok(indradb::Vertex::with_id(id, t))
}

pub fn from_edge<'a>(edge: &indradb::Edge, mut builder: crate::edge::Builder<'a>) {
    builder.set_outbound_id(edge.outbound_id.as_bytes());
    builder.set_t(&edge.t.0);
    builder.set_inbound_id(edge.inbound_id.as_bytes());
}

pub fn to_edge<'a>(reader: &crate::edge::Reader<'a>) -> Result<indradb::Edge, CapnpError> {
    let outbound_id = map_capnp_err(Uuid::from_slice(reader.get_outbound_id()?))?;
    let t = map_capnp_err(indradb::Type::new(reader.get_t()?))?;
    let inbound_id = map_capnp_err(Uuid::from_slice(reader.get_inbound_id()?))?;
    Ok(indradb::Edge::new(outbound_id, t, inbound_id))
}

pub fn from_vertex_property<'a>(property: &indradb::VertexProperty, mut builder: crate::vertex_property::Builder<'a>) {
    builder.set_id(property.id.as_bytes());
    builder.set_value(&property.value.to_string());
}

pub fn to_vertex_property<'a>(
    reader: &crate::vertex_property::Reader<'a>,
) -> Result<indradb::VertexProperty, CapnpError> {
    let id = map_capnp_err(Uuid::from_slice(reader.get_id()?))?;
    let value = map_capnp_err(serde_json::from_str(reader.get_value()?))?;
    Ok(indradb::VertexProperty::new(id, value))
}

pub fn from_vertex_properties<'a>(
    properties: &indradb::VertexProperties,
    builder: &mut crate::vertex_properties::Builder<'a>,
) {
    from_vertex(&properties.vertex, builder.reborrow().init_vertex());
    let mut props_builder = builder.reborrow().init_props(properties.props.len() as u32);
    for (i, prop) in properties.props.iter().enumerate() {
        from_named_property(prop, props_builder.reborrow().get(i as u32));
    }
}

pub fn to_vertex_properties<'a>(
    reader: &crate::vertex_properties::Reader<'a>,
) -> Result<indradb::VertexProperties, CapnpError> {
    let vertex = map_capnp_err(to_vertex(&reader.get_vertex()?))?;
    let named_props: Result<Vec<indradb::NamedProperty>, CapnpError> =
        reader.get_props()?.into_iter().map(to_named_property).collect();
    Ok(indradb::VertexProperties::new(vertex, named_props?))
}

pub fn from_named_property<'a>(property: &indradb::NamedProperty, mut builder: crate::property::Builder<'a>) {
    builder.set_name(&property.name);
    builder.set_value(&property.value.to_string());
}

pub fn to_named_property(reader: crate::property::Reader) -> Result<indradb::NamedProperty, CapnpError> {
    let name = map_capnp_err(reader.get_name())?.to_string();
    let value = map_capnp_err(serde_json::from_str(reader.get_value()?))?;
    Ok(indradb::NamedProperty::new(name, value))
}

pub fn from_edge_properties<'a>(
    properties: &indradb::EdgeProperties,
    builder: &mut crate::edge_properties::Builder<'a>,
) {
    from_edge(&properties.edge, builder.reborrow().init_edge());
    let mut props_builder = builder.reborrow().init_props(properties.props.len() as u32);
    for (i, prop) in properties.props.iter().enumerate() {
        from_named_property(prop, props_builder.reborrow().get(i as u32));
    }
}

pub fn to_edge_properties<'a>(
    reader: &crate::edge_properties::Reader<'a>,
) -> Result<indradb::EdgeProperties, CapnpError> {
    let edge = map_capnp_err(to_edge(&reader.get_edge()?))?;
    let named_props: Result<Vec<indradb::NamedProperty>, CapnpError> =
        reader.get_props()?.into_iter().map(to_named_property).collect();
    Ok(indradb::EdgeProperties::new(edge, named_props?))
}

pub fn from_edge_property<'a>(property: &indradb::EdgeProperty, mut builder: crate::edge_property::Builder<'a>) {
    builder.set_value(&property.value.to_string());
    from_edge(&property.edge, builder.init_edge());
}

pub fn to_edge_property<'a>(reader: &crate::edge_property::Reader<'a>) -> Result<indradb::EdgeProperty, CapnpError> {
    let edge = to_edge(&reader.get_edge()?)?;
    let value = map_capnp_err(serde_json::from_str(reader.get_value()?))?;
    Ok(indradb::EdgeProperty::new(edge, value))
}

pub fn from_vertex_query<'a>(
    q: &indradb::VertexQuery,
    builder: crate::vertex_query::Builder<'a>,
) -> Result<(), CapnpError> {
    match q {
        indradb::VertexQuery::Range(q) => {
            let mut builder = builder.init_range();

            if let Some(start_id) = q.start_id {
                builder.set_start_id(start_id.as_bytes());
            }

            if let Some(ref t) = q.t {
                builder.set_t(&t.0);
            }

            builder.set_limit(match q.limit {
                Some(limit) => limit as u64,
                None => u64::max_value(),
            });
        }
        indradb::VertexQuery::Specific(q) => {
            let mut builder = builder.init_specific().init_ids(q.ids.len() as u32);

            for (i, id) in q.ids.iter().enumerate() {
                builder.set(i as u32, id.as_bytes());
            }
        }
        indradb::VertexQuery::Pipe(q) => {
            let mut builder = builder.init_pipe();
            builder.set_direction(from_edge_direction(q.direction));

            builder.set_limit(match q.limit {
                Some(limit) => limit as u64,
                None => u64::max_value(),
            });

            if let Some(ref t) = q.t {
                builder.set_t(&t.0);
            }

            from_edge_query(&q.inner, builder.init_inner())?;
        }
    }

    Ok(())
}

pub fn to_vertex_query<'a>(reader: &crate::vertex_query::Reader<'a>) -> Result<indradb::VertexQuery, CapnpError> {
    match reader.which()? {
        crate::vertex_query::Range(params) => {
            let start_id_bytes = params.get_start_id()?;
            let t_str = params.get_t()?;
            let mut range = indradb::RangeVertexQuery::default().limit(map_capnp_err(params.get_limit().try_into())?);

            if !start_id_bytes.is_empty() {
                range = range.start_id(map_capnp_err(Uuid::from_slice(start_id_bytes))?);
            }

            if !t_str.is_empty() {
                range = range.t(map_capnp_err(indradb::Type::new(t_str))?);
            }

            Ok(range.into())
        }
        crate::vertex_query::Specific(params) => {
            let ids: Result<Vec<Uuid>, CapnpError> = params
                .get_ids()?
                .into_iter()
                .map(|bytes| map_capnp_err(Uuid::from_slice(bytes?)))
                .collect();
            Ok(indradb::SpecificVertexQuery::new(ids?).into())
        }
        crate::vertex_query::Pipe(params) => {
            let inner = Box::new(to_edge_query(&params.get_inner()?)?);
            let direction = to_edge_direction(params.get_direction()?);
            let t_str = params.get_t()?;
            let mut pipe =
                indradb::PipeVertexQuery::new(inner, direction).limit(map_capnp_err(params.get_limit().try_into())?);

            if !t_str.is_empty() {
                pipe = pipe.t(map_capnp_err(indradb::Type::new(t_str))?);
            }

            Ok(pipe.into())
        }
    }
}

pub fn from_vertex_property_query<'a>(
    q: &indradb::VertexPropertyQuery,
    mut builder: crate::vertex_property_query::Builder<'a>,
) -> Result<(), CapnpError> {
    builder.set_name(&q.name);
    from_vertex_query(&q.inner, builder.init_inner())?;
    Ok(())
}

pub fn to_vertex_property_query<'a>(
    reader: &crate::vertex_property_query::Reader<'a>,
) -> Result<indradb::VertexPropertyQuery, CapnpError> {
    let inner = to_vertex_query(&reader.get_inner()?)?;
    let name = reader.get_name()?;
    Ok(indradb::VertexPropertyQuery::new(inner, name))
}

pub fn from_edge_query<'a>(q: &indradb::EdgeQuery, builder: crate::edge_query::Builder<'a>) -> Result<(), CapnpError> {
    match q {
        indradb::EdgeQuery::Specific(specific) => {
            let mut builder = builder.init_specific().init_edges(specific.edges.len() as u32);

            for (i, edge) in specific.edges.iter().enumerate() {
                from_edge(edge, builder.reborrow().get(i as u32));
            }
        }
        indradb::EdgeQuery::Pipe(pipe) => {
            let mut builder = builder.init_pipe();
            builder.set_direction(from_edge_direction(pipe.direction));

            if let Some(t) = &pipe.t {
                builder.set_t(&t.0);
            }

            builder.set_limit(match pipe.limit {
                Some(limit) => map_capnp_err(limit.try_into())?,
                None => u64::max_value(),
            });

            builder.set_offset(map_capnp_err(pipe.offset.try_into())?);

            from_vertex_query(&pipe.inner, builder.init_inner())?;
        }
    }

    Ok(())
}

pub fn to_edge_query<'a>(reader: &crate::edge_query::Reader<'a>) -> Result<indradb::EdgeQuery, CapnpError> {
    match reader.which()? {
        crate::edge_query::Specific(params) => {
            let edges: Result<Vec<indradb::Edge>, CapnpError> =
                params.get_edges()?.into_iter().map(|reader| to_edge(&reader)).collect();
            Ok(indradb::EdgeQuery::Specific(indradb::SpecificEdgeQuery::new(edges?)))
        }
        crate::edge_query::Pipe(params) => {
            let inner = Box::new(to_vertex_query(&params.get_inner()?)?);
            let direction = to_edge_direction(params.get_direction()?);
            let mut pipe = indradb::PipeEdgeQuery::new(inner, direction)
                .offset(map_capnp_err(params.get_offset().try_into())?)
                .limit(map_capnp_err(params.get_limit().try_into())?);

            let t = params.get_t()?;
            if !t.is_empty() {
                pipe = pipe.t(map_capnp_err(indradb::Type::new(t))?);
            }

            Ok(indradb::EdgeQuery::Pipe(pipe))
        }
    }
}

pub fn from_edge_property_query<'a>(
    q: &indradb::EdgePropertyQuery,
    mut builder: crate::edge_property_query::Builder<'a>,
) -> Result<(), CapnpError> {
    builder.set_name(&q.name);
    from_edge_query(&q.inner, builder.init_inner())?;
    Ok(())
}

pub fn to_edge_property_query<'a>(
    reader: &crate::edge_property_query::Reader<'a>,
) -> Result<indradb::EdgePropertyQuery, CapnpError> {
    let inner = to_edge_query(&reader.get_inner()?)?;
    let name = reader.get_name()?;
    Ok(indradb::EdgePropertyQuery::new(inner, name))
}

pub fn from_bulk_insert_items<'a>(
    items: &[indradb::BulkInsertItem],
    mut builder: capnp::struct_list::Builder<'a, crate::bulk_insert_item::Owned>,
) -> Result<(), CapnpError> {
    for (i, item) in items.iter().enumerate() {
        let builder = builder.reborrow().get(i as u32);

        match item {
            indradb::BulkInsertItem::Vertex(vertex) => {
                let builder = builder.init_vertex();
                from_vertex(vertex, builder.get_vertex()?);
            }
            indradb::BulkInsertItem::Edge(edge) => {
                let builder = builder.init_edge();
                from_edge(edge, builder.get_edge()?);
            }
            indradb::BulkInsertItem::VertexProperty(id, name, value) => {
                let mut builder = builder.init_vertex_property();
                builder.set_id(id.as_bytes());
                builder.set_name(name);
                builder.set_value(&value.to_string());
            }
            indradb::BulkInsertItem::EdgeProperty(edge, name, value) => {
                let mut builder = builder.init_edge_property();
                builder.set_name(name);
                builder.set_value(&value.to_string());
                from_edge(edge, builder.get_edge()?);
            }
        }
    }

    Ok(())
}

pub fn to_bulk_insert_items<'a>(
    reader: &capnp::struct_list::Reader<'a, crate::bulk_insert_item::Owned>,
) -> Result<IntoIter<indradb::BulkInsertItem>, CapnpError> {
    let items: Result<Vec<indradb::BulkInsertItem>, CapnpError> = reader
        .into_iter()
        .map(|item| match item.which()? {
            crate::bulk_insert_item::Vertex(params) => {
                let vertex = to_vertex(&params.get_vertex()?)?;
                Ok(indradb::BulkInsertItem::Vertex(vertex))
            }
            crate::bulk_insert_item::Edge(params) => {
                let edge = to_edge(&params.get_edge()?)?;
                Ok(indradb::BulkInsertItem::Edge(edge))
            }
            crate::bulk_insert_item::VertexProperty(params) => {
                let id = map_capnp_err(Uuid::from_slice(params.get_id()?))?;
                let name = params.get_name()?.to_string();
                let value = map_capnp_err(serde_json::from_str(params.get_value()?))?;
                Ok(indradb::BulkInsertItem::VertexProperty(id, name, value))
            }
            crate::bulk_insert_item::EdgeProperty(params) => {
                let edge = to_edge(&params.get_edge()?)?;
                let name = params.get_name()?.to_string();
                let value = map_capnp_err(serde_json::from_str(params.get_value()?))?;
                Ok(indradb::BulkInsertItem::EdgeProperty(edge, name, value))
            }
        })
        .collect();
    Ok(items?.into_iter())
}

pub fn from_edge_direction(direction: indradb::EdgeDirection) -> crate::EdgeDirection {
    match direction {
        indradb::EdgeDirection::Outbound => crate::EdgeDirection::Outbound,
        indradb::EdgeDirection::Inbound => crate::EdgeDirection::Inbound,
    }
}

pub fn to_edge_direction(direction: crate::EdgeDirection) -> indradb::EdgeDirection {
    match direction {
        crate::EdgeDirection::Outbound => indradb::EdgeDirection::Outbound,
        crate::EdgeDirection::Inbound => indradb::EdgeDirection::Inbound,
    }
}
