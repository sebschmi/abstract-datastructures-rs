/// A subgraph that contains all nodes and edges that another subgraph does not contain,
/// except for those edges that are missing endpoints after inversion.
pub mod inverted_subgraph;

/// A subgraph built from the union of two graphs.
pub mod union_subgraph;
