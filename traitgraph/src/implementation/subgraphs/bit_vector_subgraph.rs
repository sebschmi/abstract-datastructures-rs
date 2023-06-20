use crate::implementation::subgraphs::filter_iterators::{
    FilterEdgeIndexIterator, FilterNeighborIterator,
};
use crate::index::GraphIndex;
use crate::interface::{
    Edge, GraphBase, ImmutableGraphContainer, MutableSubgraph, NavigableGraph, SubgraphBase,
};
use bitvec::bitvec;
use bitvec::vec::BitVec;

/// A subgraph that stores the presence or absence of a node or edge using bitvectors.
pub struct BitVectorSubgraph<'a, Graph> {
    parent_graph: &'a Graph,
    present_nodes: BitVec,
    present_edges: BitVec,
}

impl<'a, Graph: SubgraphBase> BitVectorSubgraph<'a, Graph>
where
    Graph::RootGraph: ImmutableGraphContainer,
{
    /// Constructs a new instance decorating the given graph.
    /// The subgraph is initialised empty.
    pub fn new_empty(parent_graph: &'a Graph) -> Self {
        Self {
            parent_graph,
            present_nodes: bitvec![0; parent_graph.root().node_count()],
            present_edges: bitvec![0; parent_graph.root().edge_count()],
        }
    }
}

impl<Graph: GraphBase> GraphBase for BitVectorSubgraph<'_, Graph> {
    type NodeData = Graph::NodeData;
    type EdgeData = Graph::EdgeData;
    type OptionalNodeIndex = Graph::OptionalNodeIndex;
    type OptionalEdgeIndex = Graph::OptionalEdgeIndex;
    type NodeIndex = Graph::NodeIndex;
    type EdgeIndex = Graph::EdgeIndex;
}

impl<Graph: ImmutableGraphContainer> ImmutableGraphContainer for BitVectorSubgraph<'_, Graph> {
    type NodeIndices<'a> = std::iter::Filter<Graph::NodeIndices<'a>, Box<dyn 'a + Fn(&Graph::NodeIndex) -> bool>> where Self: 'a, Graph: 'a;
    type EdgeIndices<'a> = std::iter::Filter<Graph::EdgeIndices<'a>, Box<dyn 'a + Fn(&Graph::EdgeIndex) -> bool>> where Self: 'a, Graph: 'a;

    fn node_indices(&self) -> Self::NodeIndices<'_> {
        self.parent_graph
            .node_indices()
            .filter(Box::new(|&node_index| self.contains_node_index(node_index)))
    }

    fn edge_indices(&self) -> Self::EdgeIndices<'_> {
        self.parent_graph
            .edge_indices()
            .filter(Box::new(|&edge_index| self.contains_edge_index(edge_index)))
    }

    fn contains_node_index(&self, node_id: Self::NodeIndex) -> bool {
        debug_assert!(
            self.parent_graph.contains_node_index(node_id)
                || !self.present_nodes[node_id.as_usize()]
        );
        self.present_nodes[node_id.as_usize()]
    }

    fn contains_edge_index(&self, edge_id: Self::EdgeIndex) -> bool {
        debug_assert!(
            self.parent_graph.contains_edge_index(edge_id)
                || !self.present_edges[edge_id.as_usize()]
        );
        self.present_edges[edge_id.as_usize()]
    }

    fn node_count(&self) -> usize {
        self.node_indices().count()
    }

    fn edge_count(&self) -> usize {
        self.edge_indices().count()
    }

    fn node_data(&self, node_id: Self::NodeIndex) -> &Self::NodeData {
        debug_assert!(self.contains_node_index(node_id));
        self.parent_graph.node_data(node_id)
    }

    fn edge_data(&self, edge_id: Self::EdgeIndex) -> &Self::EdgeData {
        debug_assert!(self.contains_edge_index(edge_id));
        self.parent_graph.edge_data(edge_id)
    }

    fn edge_endpoints(&self, edge_id: Self::EdgeIndex) -> Edge<Self::NodeIndex> {
        debug_assert!(self.contains_edge_index(edge_id));
        self.parent_graph.edge_endpoints(edge_id)
    }
}

impl<Graph: NavigableGraph> NavigableGraph for BitVectorSubgraph<'_, Graph> {
    type OutNeighbors<'a> = FilterNeighborIterator<'a, <Graph as NavigableGraph>::OutNeighbors<'a>, Self> where Self: 'a;
    type InNeighbors<'a> = FilterNeighborIterator<'a, <Graph as NavigableGraph>::InNeighbors<'a>, Self> where Self: 'a;
    type EdgesBetween<'a> = FilterEdgeIndexIterator<'a, <Graph as NavigableGraph>::EdgesBetween<'a>, Self> where Self: 'a;

    fn out_neighbors(&self, node_id: Self::NodeIndex) -> Self::OutNeighbors<'_> {
        FilterNeighborIterator::new(self.parent_graph.out_neighbors(node_id), self)
    }

    fn in_neighbors(&self, node_id: Self::NodeIndex) -> Self::InNeighbors<'_> {
        FilterNeighborIterator::new(self.parent_graph.in_neighbors(node_id), self)
    }

    fn edges_between(
        &self,
        from_node_id: Self::NodeIndex,
        to_node_id: Self::NodeIndex,
    ) -> Self::EdgesBetween<'_> {
        FilterEdgeIndexIterator::new(
            self.parent_graph.edges_between(from_node_id, to_node_id),
            self,
        )
    }
}

impl<Graph: ImmutableGraphContainer + SubgraphBase> SubgraphBase for BitVectorSubgraph<'_, Graph> {
    type RootGraph = Graph::RootGraph;

    fn root(&self) -> &Self::RootGraph {
        self.parent_graph.root()
    }
}

impl<Graph: ImmutableGraphContainer + SubgraphBase> MutableSubgraph for BitVectorSubgraph<'_, Graph>
where
    Self: GraphBase<
        NodeIndex = <Graph as GraphBase>::NodeIndex,
        EdgeIndex = <Graph as GraphBase>::EdgeIndex,
    >,
{
    fn clear(&mut self) {
        self.present_nodes.clear();
        self.present_edges.clear();
    }

    fn fill(&mut self) {
        self.parent_graph
            .node_indices()
            .for_each(|node_index| self.enable_node(node_index));
        self.parent_graph
            .edge_indices()
            .for_each(|edge_index| self.enable_edge(edge_index));
    }

    fn enable_node(
        &mut self,
        node_index: <<Self as SubgraphBase>::RootGraph as GraphBase>::NodeIndex,
    ) {
        debug_assert!(self.parent_graph.contains_node_index(node_index));
        self.present_nodes.set(node_index.as_usize(), true);
    }

    fn enable_edge(
        &mut self,
        edge_index: <<Self as SubgraphBase>::RootGraph as GraphBase>::EdgeIndex,
    ) {
        debug_assert!(self.parent_graph.contains_edge_index(edge_index));
        self.present_edges.set(edge_index.as_usize(), true);
    }

    fn disable_node(
        &mut self,
        node_index: <<Self as SubgraphBase>::RootGraph as GraphBase>::NodeIndex,
    ) {
        debug_assert!(self.parent_graph.contains_node_index(node_index));
        self.present_nodes.set(node_index.as_usize(), false);
    }

    fn disable_edge(
        &mut self,
        edge_index: <<Self as SubgraphBase>::RootGraph as GraphBase>::EdgeIndex,
    ) {
        debug_assert!(self.parent_graph.contains_edge_index(edge_index));
        self.present_edges.set(edge_index.as_usize(), false);
    }
}

#[cfg(test)]
mod tests {
    use bitvec::bitvec;

    #[test]
    fn test_bitvec_construction() {
        let bv = bitvec![0; 12];
        assert_eq!(bv.len(), 12);
        assert_eq!(bv.iter_ones().sum::<usize>(), 0);
        assert_eq!(bv.iter_zeros().sum::<usize>(), (0..12).sum());
    }
}
