use super::edges::EdgeKey;
use super::vertices::Vertex;
use super::properties::NamedProperty;

#[derive(Clone, Debug, PartialEq)]
pub enum BulkInsertItem {
    Vertex(Vertex, Vec<NamedProperty>),
    Edge(EdgeKey, Vec<NamedProperty>),
}
