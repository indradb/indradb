/// Most implementations of IndraDB will rely tree-like data structures. This
/// module implements transaction trait for those implementations, such that a
/// smaller set of methods just need to be implemented.
pub struct TreelikeTransaction<T: UnderlyingTreelikeTransaction> {
    underlying: T
}

impl<T: UnderlyingTreelikeTransaction> Transaction for TreelikeTransaction<T> {
    type VertexIterator = IntoIter<models::Vertex>;
    type EdgeIterator = IntoIter<models::Edge>;
    type VertexPropertyIterator = IntoIter<models::VertexProperty>;
    type EdgePropertyIterator = IntoIter<models::EdgeProperty>;
    
    fn create_vertex(&self, vertex: &models::Vertex) -> Result<bool> {
        self.underlying.create_vertex(vertex)
    }

    fn get_vertices<Q: Into<models::VertexQuery>>(&self, q: Q) -> Result<Vec<models::Vertex>> {
        match q {
            models::VertexQuery::Range(range) => {
                let mut iter: Box<dyn Iterator> = if let Some(start_id) = range.start_id {
                    Box::new(self.underlying.get_vertices_range_with_offset(start_id)?)
                } else {
                    Box::new(self.underlying.get_vertices_range()?)
                };

                if let Some(ref t) = range.t {
                    iter = Box::new(iter.filter(move |v| v.t == &t));
                }

                Ok(iter.take(range.limit as usize).collect())
            }
            models::VertexQuery::Specific(specific) => {
                let mut results = Vec::new();

                for id in specific.ids {
                    if let Some(vertex) = self.underlying.get_vertex(id) {
                        results.push(vertex);
                    }
                }

                Ok(results)
            }
            models::VertexQuery::Pipe(pipe) => {
                let iter = self.get_edges(pipe.inner).into_iter();

                let iter: Box<dyn Iterator> = match pipe.direction {
                    models::EdgeDirection::Outbound => Box::new(iter.map(|e| e.key.outbound_id)),
                    models::EdgeDirection::Inbound => Box::new(iter.map(|e| e.key.inbound_id)),
                };

                let mut results = Vec::new();

                for id in iter {
                    if let Some(vertex) = self.underlying.get_vertex(id) {
                        if let Some(ref t) = pipe.t {
                            if t == vertex.t.{
                                results.push(vertex);
                            }
                        } else {
                            results.push(vertex);
                        }
                    }
                }

                //

                let mut iter: Box<dyn Iterator<Item = (Uuid, &models::Type)>> = Box::new(
                    iter.map(|id| (id, self.vertices.get(&id)))
                        .filter_map(|(k, v)| Some((k, v?))),
                );

                if let Some(ref t) = pipe.t {
                    iter = Box::new(iter.filter(move |(_, v)| v == &t));
                }

                Ok(iter.take(pipe.limit as usize).map(|(k, v)| (k, v.clone())).collect())
            }
        }
    }

    fn delete_vertices<Q: Into<models::VertexQuery>>(&self, q: Q) -> Result<()> {
        self.underlying.delete_vertices(q)
    }

    fn get_vertex_count(&self) -> Result<u64> {
        self.underlying.get_vertex_count()
    }

    fn create_edge(&self, key: &models::EdgeKey) -> Result<bool> {
        self.underlying.create_edge(key)
    }

    fn get_edges<Q: Into<models::EdgeQuery>>(&self, q: Q) -> Result<Vec<models::Edge>> {
        self.underlying.get_edges(q)
    }

    fn delete_edges<Q: Into<models::EdgeQuery>>(&self, q: Q) -> Result<()> {
        self.underlying.delete_edges(q)
    }

    fn get_edge_count(&self, id: Uuid, t: Option<&models::Type>, direction: models::EdgeDirection) -> Result<u64> {
        self.underlying.get_edge_count(id, t, direction)
    }

    fn get_vertex_properties(&self, q: models::VertexPropertyQuery) -> Result<Vec<models::VertexProperty>> {
        self.underlying.get_vertex_properties(q)
    }

    fn set_vertex_properties(&self, q: models::VertexPropertyQuery, value: &JsonValue) -> Result<()> {
        self.underlying.set_vertex_properties(q, value)
    }

    fn delete_vertex_properties(&self, q: models::VertexPropertyQuery) -> Result<()> {
        self.underlying.delete_vertex_properties(q)
    }

    fn get_edge_properties(&self, q: models::EdgePropertyQuery) -> Result<Vec<models::EdgeProperty>> {
        self.underlying.get_edge_properties(q)
    }

    fn set_edge_properties(&self, q: models::EdgePropertyQuery, value: &JsonValue) -> Result<()> {
        self.underlying.set_edge_properties(q, value)
    }

    fn delete_edge_properties(&self, q: models::EdgePropertyQuery) -> Result<()> {
        self.underlying.delete_edge_properties(q)
    }
}

pub trait UnderlyingTreelikeTransaction {
    type VertexRangeWithOffsetIterator: Iterator<Item=models::Vertex>;
    type VertexRangeIterator: Iterator<Item=models::Vertex>;

    fn create_vertex(&self, vertex: &models::Vertex) -> Result<bool>;

    fn get_vertices_range_with_offset(&self, start_id: Uuid) -> Result<Self::VertexRangeWithOffsetIterator>;

    fn get_vertices_range(&self) -> Result<Self::RangeVertexIterator>;

    fn get_vertex(&self, id: Uuid) -> Result<Option<models::Vertex>>;

    fn delete_vertices<Q: Into<models::VertexQuery>>(&self, q: Q) -> Result<()>;

    fn get_vertex_count(&self) -> Result<u64>;

    fn create_edge(&self, key: &models::EdgeKey) -> Result<bool>;

    fn get_edges<Q: Into<models::EdgeQuery>>(&self, q: Q) -> Result<Vec<models::Edge>>;

    fn delete_edges<Q: Into<models::EdgeQuery>>(&self, q: Q) -> Result<()>;

    fn get_edge_count(&self, id: Uuid, t: Option<&models::Type>, direction: models::EdgeDirection) -> Result<u64>;

    fn get_vertex_properties(&self, q: models::VertexPropertyQuery) -> Result<Vec<models::VertexProperty>>;

    fn set_vertex_properties(&self, q: models::VertexPropertyQuery, value: &JsonValue) -> Result<()>;

    fn delete_vertex_properties(&self, q: models::VertexPropertyQuery) -> Result<()>;

    fn get_edge_properties(&self, q: models::EdgePropertyQuery) -> Result<Vec<models::EdgeProperty>>;

    fn set_edge_properties(&self, q: models::EdgePropertyQuery, value: &JsonValue) -> Result<()>;

    fn delete_edge_properties(&self, q: models::EdgePropertyQuery) -> Result<()>;
}

