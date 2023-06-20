use crate::interface::GraphBase;

/// A type that represents a subgraph of another graph.
pub trait SubgraphBase: GraphBase {
    /// The root graph of this subgraph, which is either its parent or the root of a DAG of subgraphs.
    type RootGraph: GraphBase<
        NodeData = Self::NodeData,
        EdgeData = Self::EdgeData,
        NodeIndex = Self::NodeIndex,
        EdgeIndex = Self::EdgeIndex,
        OptionalNodeIndex = Self::OptionalNodeIndex,
        OptionalEdgeIndex = Self::OptionalEdgeIndex,
    >;

    /// Returns a reference to the root graph of this subgraph.
    fn root(&self) -> &Self::RootGraph;
}

/// A type that represents a mutable subgraph, to which nodes and edges existing in the parent graph can be added,
/// and nodes and edges can be removed.
pub trait MutableSubgraph: SubgraphBase {
    /// Removes all nodes and edges from the subgraph.
    fn clear(&mut self);

    /// Adds all nodes and edges from the parent graph to this subgraph.
    fn fill(&mut self);

    /// Enables the given node index that exists in the root graph in this subgraph.
    /// This method should only be called for nodes that are enabled in the parent of this subgraph.
    fn enable_node(
        &mut self,
        node_index: <<Self as SubgraphBase>::RootGraph as GraphBase>::NodeIndex,
    );

    /// Enables the given edge index that exists in the root graph in this subgraph.
    /// This method should only be called for edges that are enabled in the parent of this subgraph.
    fn enable_edge(
        &mut self,
        edge_index: <<Self as SubgraphBase>::RootGraph as GraphBase>::EdgeIndex,
    );

    /// Disables the given node index that exists in the root graph in this subgraph.
    /// This method should only be called for nodes that are enabled in the parent of this subgraph.
    fn disable_node(
        &mut self,
        node_index: <<Self as SubgraphBase>::RootGraph as GraphBase>::NodeIndex,
    );

    /// Disables the given edge index that exists in the root graph in this subgraph.
    /// This method should only be called for edges that are enabled in the parent of this subgraph.
    fn disable_edge(
        &mut self,
        edge_index: <<Self as SubgraphBase>::RootGraph as GraphBase>::EdgeIndex,
    );
}
