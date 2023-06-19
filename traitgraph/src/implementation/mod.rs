/// A subgraph implementation based on bitvectors.
pub mod bit_vector_subgraph;
/// A subgraph implementation that allows to combine multiple subgraphs into one if they are totally ordered by the subset relation.
pub mod incremental_subgraph;
/// A subgraph implementation based on bitvectors.
/// This subgraph only allows to enable or disable nodes,
/// and edges are automatically contained if their endpoints exist.
pub mod induced_bit_vector_subgraph;
/// A subgraph implementation that allows to combine multiple subgraphs into one if they are totally ordered by the subset relation.
/// This subgraph only allows to enable or disable nodes,
/// and edges are automatically contained if their endpoints exist.
pub mod induced_incremental_subgraph;
/// A graph implementation based on the `petgraph` crate.
pub mod petgraph_impl;
/// Inverting subgraphs and computing the union or cut set of subgraphs.
pub mod subgraph_operators;
