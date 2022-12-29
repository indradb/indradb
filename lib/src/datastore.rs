use crate::errors::{Error, Result};
use crate::models::{
    AllVertexQuery, BulkInsertItem, Edge, EdgeDirection, EdgeProperties, EdgeProperty, Identifier, Json, NamedProperty,
    PipePropertyQuery, Query, QueryOutputValue, SpecificVertexQuery, Vertex, VertexProperties, VertexProperty,
};
use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use uuid::Uuid;

type DynIter<'a, T> = Box<dyn Iterator<Item = T> + 'a>;

pub trait DatastoreBackend {
    fn vertex_count(&self) -> u64;
    fn all_vertices<'a>(&self) -> Result<DynIter<'a, Vertex>>;
    fn range_vertices<'a>(&self, offset: Uuid) -> Result<DynIter<'a, Vertex>>;
    fn specific_vertices<'a>(&self, ids: &Vec<Uuid>) -> Result<DynIter<'a, Vertex>>;
    fn vertex_ids_with_property<'a>(&self, name: &Identifier) -> Result<Option<DynIter<'a, Uuid>>>;
    fn vertex_ids_with_property_value<'a>(
        &self,
        name: &Identifier,
        value: &serde_json::Value,
    ) -> Result<Option<DynIter<'a, Uuid>>>;

    fn edge_count(&self) -> u64;
    fn all_edges<'a>(&self) -> Result<DynIter<'a, Edge>>;
    fn range_edges<'a>(&self, offset: Edge) -> Result<DynIter<'a, Edge>>;
    fn range_reversed_edges<'a>(&self, offset: Edge) -> Result<DynIter<'a, Edge>>;
    fn edges_with_property<'a>(&self, name: &Identifier) -> Result<Option<DynIter<'a, Edge>>>;
    fn edges_with_property_value<'a>(
        &self,
        name: &Identifier,
        value: &serde_json::Value,
    ) -> Result<Option<DynIter<'a, Edge>>>;

    fn vertex_property(&self, vertex: &Vertex, name: &Identifier) -> Result<Option<serde_json::Value>>;
    fn all_vertex_properties_for_vertex<'a>(
        &self,
        vertex: &Vertex,
    ) -> Result<DynIter<'a, (Identifier, serde_json::Value)>>;

    fn edge_property(&self, edge: &Edge, name: &Identifier) -> Result<Option<serde_json::Value>>;
    fn all_edge_properties_for_edge<'a>(&self, edge: &Edge) -> Result<DynIter<'a, (Identifier, serde_json::Value)>>;

    fn delete_vertices(&self, vertices: Vec<Vertex>) -> Result<()>;
    fn delete_edges(&self, edges: Vec<Edge>) -> Result<()>;
    fn delete_vertex_properties(&self, props: Vec<(Vertex, Identifier, serde_json::Value)>) -> Result<()>;
    fn delete_edge_properties(&self, props: Vec<(Edge, Identifier, serde_json::Value)>) -> Result<()>;

    fn sync(&self) -> Result<()>;
    fn create_vertex(&self, vertex: &Vertex) -> Result<bool>;
    fn create_edge(&self, edge: &Edge) -> Result<bool>;
    fn bulk_insert(&self, items: Vec<BulkInsertItem>) -> Result<()>;
    fn index_property(&self, name: Identifier) -> Result<()>;
}

/// Specifies a datastore implementation.
///
/// Note that this trait and its members purposefully do not employ any
/// generic arguments. While that would improve ergonomics, it would remove
/// object safety, which we need for plugins.
///
/// # Errors
/// All methods may return an error if something unexpected happens - e.g.
/// if there was a problem connecting to the underlying database.
pub struct Datastore<B: DatastoreBackend> {
    backend: B,
}

impl Datastore {
    fn query(&self, q: &Query, output: &mut Vec<QueryOutputValue>) -> Result<()> {
        // TODO: validate query

        let value = match q {
            Query::AllVertex(_) => {
                let iter = self.backend.all_vertices()?;
                QueryOutputValue::Vertices(iter.collect())
            }
            Query::RangeVertex(ref q) => {
                let mut iter: DynIter<(&Uuid, &Identifier)> = if let Some(start_id) = q.start_id {
                    Box::new(self.backend.range_vertices(start_id))
                } else {
                    Box::new(self.backend.all_vertices())
                };

                if let Some(ref t) = q.t {
                    iter = Box::new(iter.filter(move |v| v.t == &t));
                }

                iter = Box::new(iter.take(q.limit as usize));
                QueryOutputValue::Vertices(iter.collect())
            }
            Query::SpecificVertex(ref q) => {
                let iter = self.backend.specific_vertices(q.ids);
                QueryOutputValue::Vertices(iter.collect())
            }
            Query::Pipe(ref q) => {
                self.query(&*q.inner, output)?;
                let piped_values = output.pop().unwrap();

                let values = match piped_values {
                    QueryOutputValue::Edges(ref piped_edges) => {
                        let iter: DynIter<Uuid> = match q.direction {
                            EdgeDirection::Outbound => Box::new(piped_edges.iter().map(|e| e.outbound_id)),
                            EdgeDirection::Inbound => Box::new(piped_edges.iter().map(|e| e.inbound_id)),
                        };

                        let iter: DynIter<Vertex> =
                            Box::new(self.backend.specific_vertices(&iter.collect()).into_iter());

                        if let Some(ref t) = q.t {
                            iter = Box::new(iter.filter(move |v| v.t == &t));
                        }

                        iter = Box::new(iter.take(q.limit as usize));

                        QueryOutputValue::Vertices(iter.collect())
                    }
                    QueryOutputValue::Vertices(ref piped_vertices) => {
                        let mut iter: DynIter<&Edge> = Box::new(piped_vertices.iter().flat_map(move |v| {
                            let lower_bound = match &q.t {
                                Some(t) => Edge::new(v.id, t.clone(), Uuid::default()),
                                None => Edge::new(v.id, Identifier::default(), Uuid::default()),
                            };

                            let iter = if q.direction == EdgeDirection::Outbound {
                                self.backend.range_edges(lower_bound)
                            } else {
                                self.backend.range_reversed_edges(lower_bound)
                            };

                            iter.take_while(move |edge| edge.outbound_id == v.id)
                        }));

                        if let Some(ref t) = q.t {
                            iter = Box::new(iter.filter(move |edge| &edge.t == t));
                        }

                        let iter = iter.take(q.limit as usize);

                        let iter: DynIter<Edge> = if q.direction == EdgeDirection::Outbound {
                            Box::new(iter.cloned())
                        } else {
                            Box::new(iter.map(move |edge| edge.reversed()))
                        };

                        QueryOutputValue::Edges(iter.collect())
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
                self.query(&*q.inner, output)?;
                let piped_values = output.pop().unwrap();

                let values = match piped_values {
                    QueryOutputValue::Edges(ref piped_edges) => {
                        let mut edge_properties = Vec::new();
                        for edge in piped_edges.into_iter() {
                            if let Some(name) = &q.name {
                                if let Some(value) = self.backend.edge_property(&edge, &name)? {
                                    edge_properties.push((edge.clone(), name.clone(), value.0.clone()));
                                }
                            } else {
                                for (prop_name, prop_value) in self.backend.all_edge_properties_for_edge(&edge) {
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
                                if let Some(value) = self.backend.vertex_property(&vertex, &name)? {
                                    vertex_properties.push((vertex.clone(), name.clone(), value.0.clone()));
                                }
                            } else {
                                for (prop_name, prop_value) in self.backend.all_vertex_properties_for_vertex(&vertex) {
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
                if let Some(iter) = self.backend.vertex_ids_with_property(&q.name)? {
                    let iter = self.backend.specific_vertices(&iter.collect())?;
                    QueryOutputValue::Vertices(iter.collect())
                } else {
                    return Err(Error::NotIndexed);
                }
            }
            Query::VertexWithPropertyValue(ref q) => {
                if let Some(iter) = self.backend.vertex_ids_with_property_value(&q.name, &q.value)? {
                    let iter = self.backend.specific_vertices(&iter.collect())?;
                    QueryOutputValue::Vertices(iter.collect())
                } else {
                    return Err(Error::NotIndexed);
                }
            }
            Query::EdgeWithPropertyPresence(ref q) => {
                if let Some(iter) = self.backend.edges_with_property(&q.name)? {
                    QueryOutputValue::Edges(iter.collect())
                } else {
                    return Err(Error::NotIndexed);
                }
            }
            Query::EdgeWithPropertyValue(ref q) => {
                if let Some(iter) = self.backend.edges_with_property_value(&q.name, &q.value)? {
                    QueryOutputValue::Edges(iter.collect())
                } else {
                    return Err(Error::NotIndexed);
                }
            }
            Query::PipeWithPropertyPresence(ref q) => {
                self.query(&*q.inner, output)?;
                let piped_values = output.pop().unwrap();

                let values = match piped_values {
                    QueryOutputValue::Edges(ref piped_edges) => {
                        // TODO: should `None` trigger `Error::NotIndexed`?
                        let edges_with_property = match self.backend.edges_with_property(&q.name)? {
                            Some(iter) => iter.collect::<HashSet<Edge>>(),
                            None => HashSet::<Edge>::default(),
                        };
                        let iter = piped_edges.iter().filter(move |e| {
                            let contains = edges_with_property.contains(&e);
                            (q.exists && contains) || (!q.exists && !contains)
                        });
                        QueryOutputValue::Edges(iter.cloned().collect())
                    }
                    QueryOutputValue::Vertices(ref piped_vertices) => {
                        // TODO: should `None` trigger `Error::NotIndexed`?
                        let vertices_with_property = match self.backend.vertices_with_property(&q.name)? {
                            Some(iter) => iter.collect::<HashSet<Uuid>>(),
                            None => HashSet::<Uuid>::default(),
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
                self.query(&*q.inner, output)?;
                let piped_values = output.pop().unwrap();

                let values = match piped_values {
                    QueryOutputValue::Edges(ref piped_edges) => {
                        // TODO: should `None` trigger `Error::NotIndexed`?
                        let edges = match self.backend.edges_with_property_value(&q.name, &q.value)? {
                            Some(iter) => iter.collect::<HashSet<Edge>>(),
                            None => HashSet::<Edge>::default(),
                        };
                        let iter = piped_edges.iter().filter(move |e| {
                            let contains = edges.contains(&e);
                            (q.equal && contains) || (!q.equal && !contains)
                        });
                        QueryOutputValue::Edges(iter.cloned().collect())
                    }
                    QueryOutputValue::Vertices(ref piped_vertices) => {
                        // TODO: should `None` trigger `Error::NotIndexed`?
                        let vertex_ids = match self.backend.vertex_ids_with_property_value(&q.name, &q.value)? {
                            Some(iter) => iter.collect::<HashSet<Uuid>>(),
                            None => HashSet::<Uuid>::default(),
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
                let iter = self.backend.all_edges()?;
                QueryOutputValue::Edges(iter.collect())
            }
            Query::Include(ref q) => {
                self.query(&*q.inner, output)?;
                output.pop().unwrap()
            }
            Query::Count(ref q) => {
                let count = match &*q.inner {
                    // These paths are optimized
                    Query::AllVertex(_) => self.backend.vertex_count()?,
                    Query::AllEdge(_) => self.edges.edge_count()?,
                    q => {
                        self.query(q, output)?;
                        let piped_values = output.pop().unwrap();
                        match piped_values {
                            QueryOutputValue::Vertices(v) => v.len(),
                            QueryOutputValue::Edges(e) => e.len(),
                            QueryOutputValue::VertexProperties(p) => p.len(),
                            QueryOutputValue::EdgeProperties(p) => p.len(),
                            _ => return Err(Error::Unsupported),
                        }
                    }
                };
                QueryOutputValue::Count(count as u64)
            }
        };

        output.push(value);
        Ok(())
    }

    /// Syncs persisted content. Depending on the datastore implementation,
    /// this has different meanings - including potentially being a no-op.
    pub fn sync(&self) -> Result<()> {
        self.backend.sync()
    }

    /// Creates a new vertex. Returns whether the vertex was successfully
    /// created - if this is false, it's because a vertex with the same UUID
    /// already exists.
    ///
    /// # Arguments
    /// * `vertex`: The vertex to create.
    pub fn create_vertex(&self, vertex: &Vertex) -> Result<bool> {
        self.backend.create_vertex(vertex)
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

    /// Creates a new edge. If the edge already exists, this will update it
    /// with a new update datetime. Returns whether the edge was successfully
    /// created - if this is false, it's because one of the specified vertices
    /// is missing.
    ///
    /// # Arguments
    /// * `edge`: The edge to create.
    pub fn create_edge(&self, edge: &Edge) -> Result<bool> {
        self.backend.create_edge()
    }

    pub fn get(&self, q: Query) -> Result<Vec<QueryOutputValue>> {
        // TODO: use `Vec::with_capacity`.
        let mut output = Vec::new();
        let datastore = self.datastore.read().unwrap();
        datastore.query(&q, &mut output)?;
        Ok(output)
    }

    pub fn delete(&self, q: Query) -> Result<()> {
        let mut output = Vec::new();
        self.query(&q, &mut output)?;
        match output.pop().unwrap() {
            QueryOutputValue::Vertices(vertices) => {
                self.backend.delete_vertices(vertices)?;
            }
            QueryOutputValue::Edges(edges) => {
                self.backend.delete_edges(edges)?;
            }
            QueryOutputValue::VertexProperties(vertex_properties) => {
                self.backend.delete_vertex_properties(vertex_properties)?;
            }
            QueryOutputValue::EdgeProperties(edge_properties) => {
                self.backend.delete_edge_properties(edge_properties)?;
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
        let mut output = Vec::new();
        self.query(&q, &mut output)?;

        let wrapped_value = Json::new(value);

        match output.pop().unwrap() {
            QueryOutputValue::Vertices(vertices) => {
                self.backend.set_vertex_properties(vertices, name, wrapped_value)?;
            }
            QueryOutputValue::Edges(edges) => {
                self.backend.set_edge_properties(edges, name, wrapped_value)?;
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
        self.backend.bulk_insert(items)
    }

    // Enables indexing on a specified property. When indexing is enabled on a
    // property, it's possible to query on its presence and values.
    //
    // # Arguments
    // * `name`: The name of the property to index.
    pub fn index_property(&self, name: Identifier) -> Result<()> {
        self.backend.index_property(name);
    }

    /// Gets a range of vertices specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `get`")]
    fn get_vertices(&self, q: Query) -> Result<Vec<Vertex>> {
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
    fn delete_vertices(&self, q: Query) -> Result<()> {
        // NOTE: this runs the risk of deleting non-vertices
        self.delete(q)
    }

    /// Gets the number of vertices in the datastore.
    #[deprecated(since = "4.0.0", note = "use `get` with a count query")]
    fn get_vertex_count(&self) -> Result<u64> {
        expect_count(self.get(AllVertexQuery.count().into())?)
    }

    /// Gets a range of edges specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `get`")]
    fn get_edges(&self, q: Query) -> Result<Vec<Edge>> {
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
    fn delete_edges(&self, q: Query) -> Result<()> {
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
    fn get_edge_count(&self, id: Uuid, t: Option<&Identifier>, direction: EdgeDirection) -> Result<u64> {
        let q = SpecificVertexQuery::single(id);

        let q = match direction {
            EdgeDirection::Outbound => q.outbound(),
            EdgeDirection::Inbound => q.inbound(),
        };

        let q: Query = if let Some(t) = t {
            q.t(t.clone()).count().into()
        } else {
            q.count().into()
        };

        expect_count(self.get(q)?)
    }

    /// Gets vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `get`")]
    fn get_vertex_properties(&self, q: PipePropertyQuery) -> Result<Vec<VertexProperty>> {
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
    fn get_all_vertex_properties(&self, q: Query) -> Result<Vec<VertexProperties>> {
        let props_query = PipePropertyQuery::new(Box::new(q));
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
    fn set_vertex_properties(&self, q: PipePropertyQuery, value: serde_json::Value) -> Result<()> {
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
    fn delete_vertex_properties(&self, q: PipePropertyQuery) -> Result<()> {
        self.delete(q.into())
    }

    /// Gets edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `get`")]
    fn get_edge_properties(&self, q: PipePropertyQuery) -> Result<Vec<EdgeProperty>> {
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
    fn get_all_edge_properties(&self, q: Query) -> Result<Vec<EdgeProperties>> {
        let props_query = PipePropertyQuery::new(Box::new(q));
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
    fn set_edge_properties(&self, q: PipePropertyQuery, value: serde_json::Value) -> Result<()> {
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
    fn delete_edge_properties(&self, q: PipePropertyQuery) -> Result<()> {
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
