use crate::interface::{GraphBase, ImmutableGraphContainer, Neighbor};

/// An iterator over the neighbors of a node in a subgraph.
pub struct FilterNeighborIterator<'a, Neighbors, Graph> {
    iterator: Neighbors,
    graph: &'a Graph,
}

impl<'a, Neighbors, Graph> FilterNeighborIterator<'a, Neighbors, Graph> {
    /// Creates a new instance iterating over the given `iterator` while filtering out edges that are not in `graph`.
    pub fn new(iterator: Neighbors, graph: &'a Graph) -> Self {
        Self { iterator, graph }
    }
}

impl<
        NodeIndex,
        EdgeIndex: Clone,
        Neighbors: Iterator<Item = Neighbor<NodeIndex, EdgeIndex>>,
        Graph: GraphBase<NodeIndex = NodeIndex, EdgeIndex = EdgeIndex> + ImmutableGraphContainer,
    > Iterator for FilterNeighborIterator<'_, Neighbors, Graph>
{
    type Item = Neighbor<NodeIndex, EdgeIndex>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .by_ref()
            .find(|index| self.graph.contains_edge_index(index.edge_id.clone()))
    }
}

/// An iterator over the edge indices of a subgraph.
pub struct FilterEdgeIndexIterator<'a, EdgeIndices, Graph> {
    iterator: EdgeIndices,
    graph: &'a Graph,
}

impl<'a, EdgeIndices, Graph> FilterEdgeIndexIterator<'a, EdgeIndices, Graph> {
    /// Creates a new instance iterating over the given `iterator` while filtering out edges that are not in `graph`.
    pub fn new(iterator: EdgeIndices, graph: &'a Graph) -> Self {
        Self { iterator, graph }
    }
}

impl<
        EdgeIndex: Clone,
        EdgeIndices: Iterator<Item = EdgeIndex>,
        Graph: GraphBase<EdgeIndex = EdgeIndex> + ImmutableGraphContainer,
    > Iterator for FilterEdgeIndexIterator<'_, EdgeIndices, Graph>
{
    type Item = EdgeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .by_ref()
            .find(|index| self.graph.contains_edge_index(index.clone()))
    }
}
