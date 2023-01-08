use crate::errors::{Error, Result};
use crate::models::{
    AllVertexQuery, BulkInsertItem, Edge, EdgeDirection, EdgeProperties, EdgeProperty, Identifier, NamedProperty,
    PipePropertyQuery, Query, QueryExt, QueryOutputValue, SpecificVertexQuery, Vertex, VertexProperties,
    VertexProperty,
};
use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use uuid::Uuid;

/// A dynamic iterator over results, which are commonly employed as
/// transaction return types.
pub type DynIter<'a, T> = Box<dyn Iterator<Item = Result<T>> + 'a>;

/// Specifies a datastore, which contains all implementation-specific logic.
/// Database instances call into datastores. This separation allows us to
/// minimize the amount of redundant and boilerplate logic within datastores.
///
/// Note that this trait and its members purposefully do not employ any
/// generic arguments. While that would improve ergonomics, it would remove
/// object safety, which we need for plugins.
///
/// # Errors
/// Nearly all methods may return an error if something unexpected happens -
/// e.g. if there was a problem connecting to the underlying database.
pub trait Transaction<'a> {
    /// Gets the number of vertices.
    fn vertex_count(&self) -> u64;
    /// Returns all vertices.
    fn all_vertices(&'a self) -> Result<DynIter<'a, Vertex>>;
    /// Returns all vertices with `id >= offset`.
    ///
    /// # Arguments
    /// * `offset` - Only fetch vertices with an offset greater than or equal
    /// to this value.
    fn range_vertices(&'a self, offset: Uuid) -> Result<DynIter<'a, Vertex>>;
    /// Gets a specific set of vertices with the given IDs.
    fn specific_vertices(&'a self, ids: Vec<Uuid>) -> Result<DynIter<'a, Vertex>>;
    /// Get all vertices with a given property.
    ///
    /// # Arguments
    /// * `name` - The property name.
    fn vertex_ids_with_property(&'a self, name: &Identifier) -> Result<Option<DynIter<'a, Uuid>>>;
    /// Get all vertices with a given property value.
    ///
    /// # Arguments
    /// * `name` - The property name.
    /// * `value` - The property value.
    fn vertex_ids_with_property_value(
        &'a self,
        name: &Identifier,
        value: &serde_json::Value,
    ) -> Result<Option<DynIter<'a, Uuid>>>;

    /// Gets the number of edges.
    fn edge_count(&self) -> u64;
    /// Returns all edges.
    fn all_edges(&'a self) -> Result<DynIter<'a, Edge>>;
    /// Returns all edges with that are greater than or equal to `offset`.
    ///
    /// # Arguments
    /// * `offset` - Only fetch edges greater than or equal to this value.
    fn range_edges(&'a self, offset: Edge) -> Result<DynIter<'a, Edge>>;
    /// Returns all reversed edges (where the outbound and inbound IDs are
    /// reversed from their actual values) with that are greater than or equal
    /// to `offset`.
    ///
    /// # Arguments
    /// * `offset` - Only fetch edges greater than or equal to this value.
    fn range_reversed_edges(&'a self, offset: Edge) -> Result<DynIter<'a, Edge>>;
    /// Gets a specific set of edges.
    fn specific_edges(&'a self, edges: Vec<Edge>) -> Result<DynIter<'a, Edge>>;
    /// Get all edges with a given property.
    ///
    /// # Arguments
    /// * `name` - The property name.
    fn edges_with_property(&'a self, name: &Identifier) -> Result<Option<DynIter<'a, Edge>>>;
    /// Get all edges with a given property value.
    ///
    /// # Arguments
    /// * `name` - The property name.
    /// * `value` - The property value.
    fn edges_with_property_value(
        &'a self,
        name: &Identifier,
        value: &serde_json::Value,
    ) -> Result<Option<DynIter<'a, Edge>>>;

    /// Gets the value of a vertex property if it exists, or `None` otherwise.
    ///
    /// # Arguments
    /// * `vertex` - The vertex.
    /// * `name` - The property name.
    fn vertex_property(&self, vertex: &Vertex, name: &Identifier) -> Result<Option<serde_json::Value>>;
    fn all_vertex_properties_for_vertex(
        &'a self,
        vertex: &Vertex,
    ) -> Result<DynIter<'a, (Identifier, serde_json::Value)>>;

    /// Gets the value of an edge property if it exists, or `None` otherwise.
    ///
    /// # Arguments
    /// * `edge` - The edge.
    /// * `name` - The property name.
    fn edge_property(&self, edge: &Edge, name: &Identifier) -> Result<Option<serde_json::Value>>;
    fn all_edge_properties_for_edge(&'a self, edge: &Edge) -> Result<DynIter<'a, (Identifier, serde_json::Value)>>;

    fn delete_vertices(&mut self, vertices: Vec<Vertex>) -> Result<()>;
    fn delete_edges(&mut self, edges: Vec<Edge>) -> Result<()>;
    fn delete_vertex_properties(&mut self, props: Vec<(Uuid, Identifier)>) -> Result<()>;
    fn delete_edge_properties(&mut self, props: Vec<(Edge, Identifier)>) -> Result<()>;

    fn sync(&self) -> Result<()> {
        Err(Error::Unsupported)
    }

    fn create_vertex(&mut self, vertex: &Vertex) -> Result<bool>;
    fn create_edge(&mut self, edge: &Edge) -> Result<bool>;

    fn bulk_insert(&mut self, items: Vec<BulkInsertItem>) -> Result<()> {
        for item in items {
            match item {
                BulkInsertItem::Vertex(vertex) => {
                    self.create_vertex(&vertex)?;
                }
                BulkInsertItem::Edge(edge_key) => {
                    self.create_edge(&edge_key)?;
                }
                BulkInsertItem::VertexProperty(id, name, value) => {
                    self.set_vertex_properties(vec![id], name, value)?;
                }
                BulkInsertItem::EdgeProperty(edge_key, name, value) => {
                    self.set_edge_properties(vec![edge_key], name, value)?;
                }
            }
        }

        Ok(())
    }

    fn index_property(&mut self, name: Identifier) -> Result<()>;

    fn set_vertex_properties(&mut self, vertices: Vec<Uuid>, name: Identifier, value: serde_json::Value) -> Result<()>;
    fn set_edge_properties(&mut self, edges: Vec<Edge>, name: Identifier, value: serde_json::Value) -> Result<()>;
}

pub trait Datastore {
    type Transaction<'a>: Transaction<'a>
    where
        Self: 'a;
    fn transaction<'a>(&'a self) -> Self::Transaction<'a>;
}

/// The IndraDB database.
///
/// This contains all of the logic shared across implementations, e.g. query
/// handling. Underlying it (as a generic argument) are datastores, which
/// contain implementation-specific logic.
pub struct Database<D: Datastore> {
    datastore: D,
}

impl<D: Datastore> Database<D> {
    /// Creates a new database.
    ///
    /// # Arguments
    /// * `datastore`: The underlying datastore to use.
    pub fn new(datastore: D) -> Database<D> {
        Self { datastore }
    }

    /// Syncs persisted content. Depending on the datastore implementation,
    /// this has different meanings - including potentially being a no-op.
    pub fn sync(&self) -> Result<()> {
        let txn = self.datastore.transaction();
        txn.sync()
    }

    /// Creates a new vertex. Returns whether the vertex was successfully
    /// created - if this is false, it's because a vertex with the same UUID
    /// already exists.
    ///
    /// # Arguments
    /// * `vertex`: The vertex to create.
    pub fn create_vertex(&self, vertex: &Vertex) -> Result<bool> {
        let mut txn = self.datastore.transaction();
        txn.create_vertex(vertex)
    }

    /// Creates a new vertex with just a type specification. As opposed to
    /// `create_vertex`, this is used when you do not want to manually specify
    /// the vertex's UUID. Returns the new vertex's UUID.
    ///
    /// # Arguments
    /// * `t`: The type of the vertex to create.
    pub fn create_vertex_from_type(&self, t: Identifier) -> Result<Uuid> {
        let v = Vertex::new(t);

        if !self.create_vertex(&v)? {
            Err(Error::UuidTaken)
        } else {
            Ok(v.id)
        }
    }

    /// Creates a new edge. Returns whether the edge was successfully
    /// created - if this is false, it's because one of the specified vertices
    /// is missing.
    ///
    /// # Arguments
    /// * `edge`: The edge to create.
    pub fn create_edge(&self, edge: &Edge) -> Result<bool> {
        let mut txn = self.datastore.transaction();
        txn.create_edge(edge)
    }

    /// Gets values specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub fn get(&self, q: Query) -> Result<Vec<QueryOutputValue>> {
        let txn = self.datastore.transaction();
        let mut output = Vec::with_capacity(q.output_len());
        unsafe {
            query(&txn as *const D::Transaction<'_>, &q, &mut output)?;
        }
        Ok(output)
    }

    /// Deletes values specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub fn delete(&self, q: Query) -> Result<()> {
        let mut txn = self.datastore.transaction();
        let mut output = Vec::with_capacity(q.output_len());
        unsafe {
            query(&txn as *const D::Transaction<'_>, &q, &mut output)?;
        }
        match output.pop().unwrap() {
            QueryOutputValue::Vertices(vertices) => {
                txn.delete_vertices(vertices)?;
            }
            QueryOutputValue::Edges(edges) => {
                txn.delete_edges(edges)?;
            }
            QueryOutputValue::VertexProperties(vertex_properties) => {
                txn.delete_vertex_properties(
                    vertex_properties
                        .into_iter()
                        .map(|(vertex, prop_name, _prop_value)| (vertex.id, prop_name))
                        .collect(),
                )?;
            }
            QueryOutputValue::EdgeProperties(edge_properties) => {
                txn.delete_edge_properties(
                    edge_properties
                        .into_iter()
                        .map(|(edge, prop_name, _prop_value)| (edge, prop_name))
                        .collect(),
                )?;
            }
            QueryOutputValue::Count(_) => return Err(Error::Unsupported),
        }
        Ok(())
    }

    /// Sets properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    /// * `name`: The property name.
    /// * `value`: The property value.
    pub fn set_properties(&self, q: Query, name: Identifier, value: serde_json::Value) -> Result<()> {
        let mut txn = self.datastore.transaction();
        let mut output = Vec::with_capacity(q.output_len());
        unsafe {
            query(&txn as *const D::Transaction<'_>, &q, &mut output)?;
        }

        match output.pop().unwrap() {
            QueryOutputValue::Vertices(vertices) => {
                txn.set_vertex_properties(vertices.into_iter().map(|v| v.id).collect(), name, value)?;
            }
            QueryOutputValue::Edges(edges) => {
                txn.set_edge_properties(edges, name, value)?;
            }
            _ => return Err(Error::Unsupported),
        }
        Ok(())
    }

    /// Bulk inserts many vertices, edges, and/or properties.
    ///
    /// # Arguments
    /// * `items`: The items to insert.
    pub fn bulk_insert(&self, items: Vec<BulkInsertItem>) -> Result<()> {
        let mut txn = self.datastore.transaction();
        txn.bulk_insert(items)
    }

    /// Enables indexing on a specified property. When indexing is enabled on a
    /// property, it's possible to query on its presence and values.
    ///
    /// # Arguments
    /// * `name`: The name of the property to index.
    pub fn index_property(&self, name: Identifier) -> Result<()> {
        let mut txn = self.datastore.transaction();
        txn.index_property(name)
    }

    // All functions after this are deprecated. They provide an interface that
    // is close to (but not the same as) IndraDB versions below 4. At some
    // point these will be removed, likely in version 5.

    /// Gets a range of vertices specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `get`")]
    pub fn get_vertices(&self, q: Query) -> Result<Vec<Vertex>> {
        if let Some(QueryOutputValue::Vertices(vertices)) = self.get(q)?.pop() {
            Ok(vertices)
        } else {
            Err(Error::Unsupported)
        }
    }

    /// Deletes existing vertices specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `delete`")]
    pub fn delete_vertices(&self, q: Query) -> Result<()> {
        // NOTE: this runs the risk of deleting non-vertices
        self.delete(q)
    }

    /// Gets the number of vertices in the datastore.
    #[deprecated(since = "4.0.0", note = "use `get` with a count query")]
    pub fn get_vertex_count(&self) -> Result<u64> {
        expect_count(self.get(AllVertexQuery.count().unwrap().into())?)
    }

    /// Gets a range of edges specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `get`")]
    pub fn get_edges(&self, q: Query) -> Result<Vec<Edge>> {
        if let Some(QueryOutputValue::Edges(edges)) = self.get(q)?.pop() {
            Ok(edges)
        } else {
            Err(Error::Unsupported)
        }
    }

    /// Deletes a set of edges specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `delete`")]
    pub fn delete_edges(&self, q: Query) -> Result<()> {
        // NOTE: this runs the risk of deleting non-edges
        self.delete(q)
    }

    /// Gets the number of edges associated with a vertex.
    ///
    /// # Arguments
    /// * `id`: The id of the vertex.
    /// * `t`: Only get the count for a specified edge type.
    /// * `direction`: The direction of edges to get.
    #[deprecated(since = "4.0.0", note = "use `get` with a count query")]
    pub fn get_edge_count(&self, id: Uuid, t: Option<&Identifier>, direction: EdgeDirection) -> Result<u64> {
        let q = SpecificVertexQuery::single(id);

        let q = match direction {
            EdgeDirection::Outbound => q.outbound().unwrap(),
            EdgeDirection::Inbound => q.inbound().unwrap(),
        };

        let q: Query = if let Some(t) = t {
            q.t(t.clone()).count().unwrap().into()
        } else {
            q.count().unwrap().into()
        };

        expect_count(self.get(q)?)
    }

    /// Gets vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `get`")]
    pub fn get_vertex_properties(&self, q: PipePropertyQuery) -> Result<Vec<VertexProperty>> {
        if let Some(QueryOutputValue::VertexProperties(props)) = self.get(q.into())?.pop() {
            let iter = props
                .into_iter()
                .map(|(vertex, _prop_name, prop_value)| VertexProperty::new(vertex.id, prop_value));
            Ok(iter.collect())
        } else {
            Err(Error::Unsupported)
        }
    }

    /// Gets all vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `get`")]
    pub fn get_all_vertex_properties(&self, q: Query) -> Result<Vec<VertexProperties>> {
        // `QueryExt::properties()` not used here because this function is not
        // generic in order to keep this object safe.
        let props_query = PipePropertyQuery::new(Box::new(q))?;
        if let Some(QueryOutputValue::VertexProperties(props)) = self.get(props_query.into())?.pop() {
            let mut props_by_vertex = HashMap::new();
            for (vertex, prop_name, prop_value) in props.into_iter() {
                props_by_vertex
                    .entry(vertex)
                    .or_insert_with(Vec::new)
                    .push(NamedProperty::new(prop_name, prop_value));
            }
            let mut grouped_properties = Vec::with_capacity(props_by_vertex.len());
            for (vertex, named_properties) in props_by_vertex.drain() {
                grouped_properties.push(VertexProperties::new(vertex, named_properties));
            }
            Ok(grouped_properties)
        } else {
            Err(Error::Unsupported)
        }
    }

    /// Sets a vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    /// * `value`: The property value.
    #[deprecated(since = "4.0.0", note = "use `set_properties`")]
    pub fn set_vertex_properties(&self, q: PipePropertyQuery, value: serde_json::Value) -> Result<()> {
        if let Some(name) = q.name {
            self.set_properties(*q.inner, name, value)
        } else {
            // Name must be specified for this compat fn to work
            Err(Error::Unsupported)
        }
    }

    /// Deletes vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `delete`")]
    pub fn delete_vertex_properties(&self, q: PipePropertyQuery) -> Result<()> {
        self.delete(q.into())
    }

    /// Gets edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `get`")]
    pub fn get_edge_properties(&self, q: PipePropertyQuery) -> Result<Vec<EdgeProperty>> {
        if let Some(QueryOutputValue::EdgeProperties(props)) = self.get(q.into())?.pop() {
            let iter = props
                .into_iter()
                .map(|(edge, _prop_name, prop_value)| EdgeProperty::new(edge, prop_value));
            Ok(iter.collect())
        } else {
            Err(Error::Unsupported)
        }
    }

    /// Gets all edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `get`")]
    pub fn get_all_edge_properties(&self, q: Query) -> Result<Vec<EdgeProperties>> {
        // `QueryExt::properties()` not used here because this function is not
        // generic in order to keep this object safe.
        let props_query = PipePropertyQuery::new(Box::new(q))?;
        if let Some(QueryOutputValue::EdgeProperties(props)) = self.get(props_query.into())?.pop() {
            let mut props_by_edge = HashMap::new();
            for (edge, prop_name, prop_value) in props.into_iter() {
                props_by_edge
                    .entry(edge.clone())
                    .or_insert_with(Vec::new)
                    .push(NamedProperty::new(prop_name, prop_value));
            }
            let mut grouped_properties = Vec::with_capacity(props_by_edge.len());
            for (edge, named_properties) in props_by_edge.drain() {
                grouped_properties.push(EdgeProperties::new(edge, named_properties));
            }
            Ok(grouped_properties)
        } else {
            Err(Error::Unsupported)
        }
    }

    /// Sets edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    /// * `value`: The property value.
    #[deprecated(since = "4.0.0", note = "use `set_properties`")]
    pub fn set_edge_properties(&self, q: PipePropertyQuery, value: serde_json::Value) -> Result<()> {
        if let Some(name) = q.name {
            self.set_properties(*q.inner, name, value)
        } else {
            // Name must be specified for this compat fn to work
            Err(Error::Unsupported)
        }
    }

    /// Deletes edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `delete`")]
    pub fn delete_edge_properties(&self, q: PipePropertyQuery) -> Result<()> {
        self.delete(q.into())
    }
}

fn expect_count(mut output: Vec<QueryOutputValue>) -> Result<u64> {
    if let Some(QueryOutputValue::Count(count)) = output.pop() {
        Ok(count)
    } else {
        unreachable!()
    }
}

unsafe fn query<'a, T: Transaction<'a> + 'a>(
    txn: *const T,
    q: &Query,
    output: &mut Vec<QueryOutputValue>,
) -> Result<()> {
    // TODO: validate query
    let value = match q {
        Query::AllVertex(_) => {
            let iter = (*txn).all_vertices()?;
            QueryOutputValue::Vertices(iter.collect::<Result<Vec<Vertex>>>()?)
        }
        Query::RangeVertex(ref q) => {
            let mut iter: DynIter<Vertex> = if let Some(start_id) = q.start_id {
                (*txn).range_vertices(start_id)?
            } else {
                (*txn).all_vertices()?
            };

            if let Some(ref t) = q.t {
                iter = Box::new(iter.filter(move |r| match r {
                    Ok(v) => &v.t == t,
                    Err(_) => true,
                }));
            }

            iter = Box::new(iter.take(q.limit as usize));
            QueryOutputValue::Vertices(iter.collect::<Result<Vec<Vertex>>>()?)
        }
        Query::SpecificVertex(ref q) => {
            let iter = (*txn).specific_vertices(q.ids.clone())?;
            QueryOutputValue::Vertices(iter.collect::<Result<Vec<Vertex>>>()?)
        }
        Query::Pipe(ref q) => {
            query(txn, &*q.inner, output)?;
            let piped_values = output.pop().unwrap();

            let values = match piped_values {
                QueryOutputValue::Edges(ref piped_edges) => {
                    let iter: Box<dyn Iterator<Item = Uuid>> = match q.direction {
                        EdgeDirection::Outbound => Box::new(piped_edges.iter().map(|e| e.outbound_id)),
                        EdgeDirection::Inbound => Box::new(piped_edges.iter().map(|e| e.inbound_id)),
                    };

                    let mut iter: DynIter<Vertex> = (*txn).specific_vertices(iter.collect())?;

                    if let Some(ref t) = q.t {
                        iter = Box::new(iter.filter(move |r| match r {
                            Ok(v) => &v.t == t,
                            Err(_) => true,
                        }));
                    }

                    iter = Box::new(iter.take(q.limit as usize));

                    QueryOutputValue::Vertices(iter.collect::<Result<Vec<Vertex>>>()?)
                }
                QueryOutputValue::Vertices(ref piped_vertices) => {
                    let mut edges = Vec::new();

                    for vertex in piped_vertices {
                        let lower_bound = match &q.t {
                            Some(t) => Edge::new(vertex.id, t.clone(), Uuid::default()),
                            None => Edge::new(vertex.id, Identifier::default(), Uuid::default()),
                        };

                        let mut iter = if q.direction == EdgeDirection::Outbound {
                            (*txn).range_edges(lower_bound)?
                        } else {
                            (*txn).range_reversed_edges(lower_bound)?
                        };

                        iter = Box::new(iter.take_while(move |r| match r {
                            Ok(e) => e.outbound_id == vertex.id,
                            Err(_) => true,
                        }));

                        if let Some(ref t) = q.t {
                            iter = Box::new(iter.filter(move |r| match r {
                                Ok(e) => &e.t == t,
                                Err(_) => true,
                            }));
                        }

                        if q.direction == EdgeDirection::Inbound {
                            iter = Box::new(iter.map(move |r| Ok(r?.reversed())));
                        }

                        iter = Box::new(iter.take((q.limit as usize) - edges.len()));

                        for result in iter {
                            edges.push(result?);
                        }

                        if edges.len() >= (q.limit as usize) {
                            break;
                        }
                    }

                    QueryOutputValue::Edges(edges)
                }
                _ => {
                    return Err(Error::Unsupported);
                }
            };

            if let Query::Include(_) = *q.inner {
                // keep the value exported
                output.push(piped_values);
            }

            values
        }
        Query::PipeProperty(ref q) => {
            query(txn, &*q.inner, output)?;
            let piped_values = output.pop().unwrap();

            let values = match piped_values {
                QueryOutputValue::Edges(ref piped_edges) => {
                    let mut edge_properties = Vec::new();
                    for edge in piped_edges.into_iter() {
                        if let Some(name) = &q.name {
                            if let Some(value) = (*txn).edge_property(&edge, &name)? {
                                edge_properties.push((edge.clone(), name.clone(), value.clone()));
                            }
                        } else {
                            for result in (*txn).all_edge_properties_for_edge(&edge)? {
                                let (prop_name, prop_value) = result?;
                                edge_properties.push((edge.clone(), prop_name, prop_value));
                            }
                        }
                    }

                    QueryOutputValue::EdgeProperties(edge_properties)
                }
                QueryOutputValue::Vertices(ref piped_vertices) => {
                    let mut vertex_properties = Vec::with_capacity(piped_vertices.len());
                    for vertex in piped_vertices.into_iter() {
                        if let Some(name) = &q.name {
                            if let Some(value) = (*txn).vertex_property(&vertex, &name)? {
                                vertex_properties.push((vertex.clone(), name.clone(), value.clone()));
                            }
                        } else {
                            for result in (*txn).all_vertex_properties_for_vertex(&vertex)? {
                                let (prop_name, prop_value) = result?;
                                vertex_properties.push((vertex.clone(), prop_name, prop_value));
                            }
                        }
                    }

                    QueryOutputValue::VertexProperties(vertex_properties)
                }
                _ => {
                    return Err(Error::Unsupported);
                }
            };

            if let Query::Include(_) = *q.inner {
                // keep the value exported
                output.push(piped_values);
            }

            values
        }
        Query::VertexWithPropertyPresence(ref q) => {
            if let Some(iter) = (*txn).vertex_ids_with_property(&q.name)? {
                let iter = (*txn).specific_vertices(iter.collect::<Result<Vec<Uuid>>>()?)?;
                QueryOutputValue::Vertices(iter.collect::<Result<Vec<Vertex>>>()?)
            } else {
                return Err(Error::NotIndexed);
            }
        }
        Query::VertexWithPropertyValue(ref q) => {
            if let Some(iter) = (*txn).vertex_ids_with_property_value(&q.name, &q.value)? {
                let iter = (*txn).specific_vertices(iter.collect::<Result<Vec<Uuid>>>()?)?;
                QueryOutputValue::Vertices(iter.collect::<Result<Vec<Vertex>>>()?)
            } else {
                return Err(Error::NotIndexed);
            }
        }
        Query::EdgeWithPropertyPresence(ref q) => {
            if let Some(iter) = (*txn).edges_with_property(&q.name)? {
                QueryOutputValue::Edges(iter.collect::<Result<Vec<Edge>>>()?)
            } else {
                return Err(Error::NotIndexed);
            }
        }
        Query::EdgeWithPropertyValue(ref q) => {
            if let Some(iter) = (*txn).edges_with_property_value(&q.name, &q.value)? {
                QueryOutputValue::Edges(iter.collect::<Result<Vec<Edge>>>()?)
            } else {
                return Err(Error::NotIndexed);
            }
        }
        Query::PipeWithPropertyPresence(ref q) => {
            query(txn, &*q.inner, output)?;
            let piped_values = output.pop().unwrap();

            let values = match piped_values {
                QueryOutputValue::Edges(ref piped_edges) => {
                    let edges_with_property = match (*txn).edges_with_property(&q.name)? {
                        Some(iter) => iter.collect::<Result<HashSet<Edge>>>()?,
                        None => return Err(Error::NotIndexed),
                    };
                    let iter = piped_edges.iter().filter(move |e| {
                        let contains = edges_with_property.contains(&e);
                        (q.exists && contains) || (!q.exists && !contains)
                    });
                    QueryOutputValue::Edges(iter.cloned().collect())
                }
                QueryOutputValue::Vertices(ref piped_vertices) => {
                    let vertices_with_property = match (*txn).vertex_ids_with_property(&q.name)? {
                        Some(iter) => iter.collect::<Result<HashSet<Uuid>>>()?,
                        None => return Err(Error::NotIndexed),
                    };
                    let iter = piped_vertices.iter().filter(move |v| {
                        let contains = vertices_with_property.contains(&v.id);
                        (q.exists && contains) || (!q.exists && !contains)
                    });
                    QueryOutputValue::Vertices(iter.cloned().collect())
                }
                _ => {
                    return Err(Error::Unsupported);
                }
            };

            if let Query::Include(_) = *q.inner {
                // keep the value exported
                output.push(piped_values);
            }

            values
        }
        Query::PipeWithPropertyValue(ref q) => {
            query(txn, &*q.inner, output)?;
            let piped_values = output.pop().unwrap();

            let values = match piped_values {
                QueryOutputValue::Edges(ref piped_edges) => {
                    let edges = match (*txn).edges_with_property_value(&q.name, &q.value)? {
                        Some(iter) => iter.collect::<Result<HashSet<Edge>>>()?,
                        None => return Err(Error::NotIndexed),
                    };
                    let iter = piped_edges.iter().filter(move |e| {
                        let contains = edges.contains(&e);
                        (q.equal && contains) || (!q.equal && !contains)
                    });
                    QueryOutputValue::Edges(iter.cloned().collect())
                }
                QueryOutputValue::Vertices(ref piped_vertices) => {
                    let vertex_ids = match (*txn).vertex_ids_with_property_value(&q.name, &q.value)? {
                        Some(iter) => iter.collect::<Result<HashSet<Uuid>>>()?,
                        None => return Err(Error::NotIndexed),
                    };
                    let iter = piped_vertices.iter().filter(move |v| {
                        let contains = vertex_ids.contains(&v.id);
                        (q.equal && contains) || (!q.equal && !contains)
                    });
                    QueryOutputValue::Vertices(iter.cloned().collect())
                }
                _ => {
                    return Err(Error::Unsupported);
                }
            };

            if let Query::Include(_) = *q.inner {
                // keep the value exported
                output.push(piped_values);
            }

            values
        }
        Query::AllEdge(_) => {
            let iter = (*txn).all_edges()?;
            QueryOutputValue::Edges(iter.collect::<Result<Vec<Edge>>>()?)
        }
        Query::SpecificEdge(ref q) => {
            let iter = (*txn).specific_edges(q.edges.clone())?;
            QueryOutputValue::Edges(iter.collect::<Result<Vec<Edge>>>()?)
        }
        Query::Include(ref q) => {
            query(txn, &*q.inner, output)?;
            output.pop().unwrap()
        }
        Query::Count(ref q) => {
            let count = match &*q.inner {
                // These paths are optimized
                Query::AllVertex(_) => (*txn).vertex_count(),
                Query::AllEdge(_) => (*txn).edge_count(),
                q => {
                    query(txn, q, output)?;
                    let piped_values = output.pop().unwrap();
                    let len = match piped_values {
                        QueryOutputValue::Vertices(v) => v.len(),
                        QueryOutputValue::Edges(e) => e.len(),
                        QueryOutputValue::VertexProperties(p) => p.len(),
                        QueryOutputValue::EdgeProperties(p) => p.len(),
                        _ => return Err(Error::Unsupported),
                    };
                    len as u64
                }
            };
            QueryOutputValue::Count(count as u64)
        }
    };

    output.push(value);
    Ok(())
}
