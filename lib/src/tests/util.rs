use crate::errors::{Error, Result};
use crate::util::{extract_count, extract_edge_properties, extract_edges, extract_vertex_properties, extract_vertices};
use crate::{models, Database, Datastore};

use uuid::Uuid;

/// A wrapper around a database used for testing.
///
/// IndraDB versions < 4 had a very different interface, which the fairly
/// extensive test suite was written against. Parts of the test suite was
/// updated for the new interface, but not all of it. To decrease the amount
/// of refactoring needed, tests now get passed an instance of `TestDatabase`
/// instead, which more closely mirrors the old interface.
pub struct TestDatabase<D: Datastore> {
    db: Database<D>,
}

impl<D: Datastore> TestDatabase<D> {
    /// Gets a range of vertices specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub fn get_vertices(&self, q: models::Query) -> Result<Vec<models::Vertex>> {
        extract_vertices(self.db.get(q)?).ok_or(Error::Unsupported)
    }

    /// Deletes existing vertices specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub fn delete_vertices(&self, q: models::Query) -> Result<()> {
        // NOTE: this runs the risk of deleting non-vertices
        self.db.delete(q)
    }

    /// Gets the number of vertices in the datastore.
    pub fn get_vertex_count(&self) -> Result<u64> {
        extract_count(self.db.get(models::AllVertexQuery.count().unwrap().into())?).map_err(Error::Unsupported)
    }

    /// Gets a range of edges specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub fn get_edges(&self, q: models::Query) -> Result<Vec<models::Edge>> {
        extract_edges(self.db.get(q)?).map_err(Error::Unsupported)
    }

    /// Deletes a set of edges specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub fn delete_edges(&self, q: models::Query) -> Result<()> {
        // NOTE: this runs the risk of deleting non-edges
        self.db.delete(q)
    }

    /// Gets the number of edges associated with a vertex.
    ///
    /// # Arguments
    /// * `id`: The id of the vertex.
    /// * `t`: Only get the count for a specified edge type.
    /// * `direction`: The direction of edges to get.
    pub fn get_edge_count(
        &self,
        id: Uuid,
        t: Option<&models::Identifier>,
        direction: models::EdgeDirection,
    ) -> Result<u64> {
        let q = models::SpecificVertexQuery::single(id);

        let q = match direction {
            models::EdgeDirection::Outbound => q.outbound().unwrap(),
            models::EdgeDirection::Inbound => q.inbound().unwrap(),
        };

        let q: models::Query = if let Some(t) = t {
            q.t(t.clone()).count().unwrap().into()
        } else {
            q.count().unwrap().into()
        };

        extract_count(self.db.get(q)?).map_err(Error::Unsupported)
    }

    /// Gets vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub fn get_vertex_properties(&self, q: models::PipePropertyQuery) -> Result<Vec<models::VertexProperty>> {
        let props = extract_vertex_properties(self.db.get(q.into())?).map_err(Error::Unsupported)?;
        if props.len() > 1 {
            Err(Error::Unsupported)
        } else {
            let iter = props.into_iter().flat_map(|vps| {
                vps.props
                    .into_iter()
                    .map(move |vp| models::VertexProperty::new(vps.vertex.id, vp.value))
            });
            Ok(iter.collect())
        }
    }

    /// Gets all vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub fn get_all_vertex_properties(&self, q: models::Query) -> Result<Vec<models::VertexProperties>> {
        // `QueryExt::properties()` not used here because this function is not
        // generic in order to keep this object safe.
        let props_query = models::PipePropertyQuery::new(Box::new(q))?;
        let props = extract_vertex_properties(self.db.get(props_query.into())?).map_err(Error::Unsupported)?;
        Ok(props)
    }

    /// Sets a vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    /// * `value`: The property value.
    pub fn set_vertex_properties(&self, q: models::PipePropertyQuery, value: serde_json::Value) -> Result<()> {
        if let Some(name) = q.name {
            self.db.set_properties(*q.inner, name, value)
        } else {
            // Name must be specified for this compat fn to work
            Err(Error::Unsupported)
        }
    }

    /// Deletes vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub fn delete_vertex_properties(&self, q: models::PipePropertyQuery) -> Result<()> {
        self.db.delete(q.into())
    }

    /// Gets edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub fn get_edge_properties(&self, q: models::PipePropertyQuery) -> Result<Vec<models::EdgeProperty>> {
        let props = extract_edge_properties(self.db.get(q.into())?).map_err(Error::Unsupported)?;
        if props.len() > 1 {
            Err(Error::Unsupported)
        } else {
            let iter = props.into_iter().flat_map(move |eps| {
                let iter = eps
                    .props
                    .into_iter()
                    .map(|ep| models::EdgeProperty::new(eps.edge.clone(), ep.value));
                iter.collect::<Vec<models::EdgeProperty>>()
            });
            Ok(iter.collect())
        }
    }

    /// Gets all edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub fn get_all_edge_properties(&self, q: models::Query) -> Result<Vec<models::EdgeProperties>> {
        // `QueryExt::properties()` not used here because this function is not
        // generic in order to keep this object safe.
        let props_query = models::PipePropertyQuery::new(Box::new(q))?;
        extract_edge_properties(self.db.get(props_query.into())?).map_err(Error::Unsupported)
    }

    /// Sets edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    /// * `value`: The property value.
    pub fn set_edge_properties(&self, q: models::PipePropertyQuery, value: serde_json::Value) -> Result<()> {
        if let Some(name) = q.name {
            self.db.set_properties(*q.inner, name, value)
        } else {
            // Name must be specified for this compat fn to work
            Err(Error::Unsupported)
        }
    }

    /// Deletes edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    pub fn delete_edge_properties(&self, q: models::PipePropertyQuery) -> Result<()> {
        self.db.delete(q.into())
    }
}

// TODO: move these into datastore

pub fn create_edge_from<D: Datastore>(db: &TestDatabase<D>, outbound_id: Uuid) -> Uuid {
    let inbound_vertex_t = models::Identifier::new("test_inbound_vertex_type").unwrap();
    let inbound_v = models::Vertex::new(inbound_vertex_t);
    db.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    let edge = models::Edge::new(outbound_id, edge_t, inbound_v.id);
    db.create_edge(&edge).unwrap();
    inbound_v.id
}

pub fn create_edges<D: Datastore>(db: &TestDatabase<D>) -> (Uuid, [Uuid; 5]) {
    let outbound_vertex_t = models::Identifier::new("test_outbound_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(outbound_vertex_t);
    db.create_vertex(&outbound_v).unwrap();
    let inbound_ids: [Uuid; 5] = [
        create_edge_from(db, outbound_v.id),
        create_edge_from(db, outbound_v.id),
        create_edge_from(db, outbound_v.id),
        create_edge_from(db, outbound_v.id),
        create_edge_from(db, outbound_v.id),
    ];

    (outbound_v.id, inbound_ids)
}
