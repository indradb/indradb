use super::edges::EdgeKey;
use super::properties::NamedProperty;
use super::vertices::Vertex;

#[derive(Clone, Debug, PartialEq)]
pub enum BulkInsertItem {
    Vertex(Vertex, Vec<NamedProperty>),
    Edge(EdgeKey, Vec<NamedProperty>),
}
