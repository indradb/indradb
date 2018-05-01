use super::http;
use super::statics;
use indradb::{Datastore, Edge, EdgeDirection, EdgeKey, EdgeMetadata, EdgeQuery, Error, Transaction, Type, Vertex, VertexMetadata, VertexQuery};
use serde_json::value::Value as JsonValue;
use uuid::Uuid;
use juniper;

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
    fn request(&self, body: &str) -> Result<juniper::Value, Error> {
        let (value, errors) = juniper::execute(
            body,
            None,
            &http::Schema::new(http::RootQuery, http::RootMutation),
            &juniper::Variables::new(),
            &self.context,
        ).unwrap();

        if errors.len() > 0 {
            let description = format!("{:?}", errors[0]);
            Err(Error::from(description))
        } else {
            Ok(value)
        }
    }
}

impl Transaction for ClientTransaction {
    fn create_vertex(&self, v: &Vertex) -> Result<bool, Error> {
        unimplemented!();
    }

    fn create_vertex_from_type(&self, t: Type) -> Result<Uuid, Error> {
        unimplemented!();
    }

    fn get_vertices(&self, q: &VertexQuery) -> Result<Vec<Vertex>, Error> {
        unimplemented!();
    }

    fn delete_vertices(&self, q: &VertexQuery) -> Result<(), Error> {
        unimplemented!();
    }

    fn get_vertex_count(&self) -> Result<u64, Error> {
        unimplemented!();
    }

    fn create_edge(&self, e: &EdgeKey) -> Result<bool, Error> {
        unimplemented!();
    }

    fn get_edges(&self, q: &EdgeQuery) -> Result<Vec<Edge>, Error> {
        unimplemented!();
    }

    fn delete_edges(&self, q: &EdgeQuery) -> Result<(), Error> {
        unimplemented!();
    }

    fn get_edge_count(&self, id: Uuid, type_filter: Option<&Type>, direction: EdgeDirection) -> Result<u64, Error> {
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
