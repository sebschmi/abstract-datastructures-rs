use crate::implementation::subgraphs::filter_iterators::{
    FilterEdgeIndexIterator, FilterNeighborIterator,
};
use crate::index::GraphIndex;
use crate::interface::subgraph::{MutableSubgraph, SubgraphBase};
use crate::interface::{Edge, GraphBase, ImmutableGraphContainer, NavigableGraph};
use std::iter::Filter;

type IntegerType = usize;

/// A subgraph that stores the presence or absence of a node or edge using integers.
///
/// Additionally, this subgraph has a current step that can be altered.
/// Nodes and edges are added with that step, and only nodes and edges with a step lower or equal to the current one are counted as present.
/// This allows to combine multiple subgraphs into one, if they are totally ordered by the subset relation.
pub struct IncrementalSubgraph<'a, Graph: GraphBase> {
    parent_graph: &'a Graph,
    present_nodes: Vec<IntegerType>,
    present_edges: Vec<IntegerType>,
    new_nodes: Vec<Vec<Graph::NodeIndex>>,
    new_edges: Vec<Vec<Graph::EdgeIndex>>,
    current_step: IntegerType,
}

impl<Graph: GraphBase> GraphBase for IncrementalSubgraph<'_, Graph> {
    type NodeData = Graph::NodeData;
    type EdgeData = Graph::EdgeData;
    type OptionalNodeIndex = Graph::OptionalNodeIndex;
    type OptionalEdgeIndex = Graph::OptionalEdgeIndex;
    type NodeIndex = Graph::NodeIndex;
    type EdgeIndex = Graph::EdgeIndex;
}

impl<Graph: SubgraphBase> SubgraphBase for IncrementalSubgraph<'_, Graph> {
    type RootGraph = Graph::RootGraph;

    fn root(&self) -> &Self::RootGraph {
        self.parent_graph.root()
    }
}

impl<'a, Graph: ImmutableGraphContainer> IncrementalSubgraph<'a, Graph> {
    /// Create an incremental subgraph with the given amount of incremental steps.
    pub fn new_with_incremental_steps(graph: &'a Graph, incremental_steps: usize) -> Self {
        Self {
            parent_graph: graph,
            present_nodes: vec![IntegerType::MAX; graph.node_count()],
            present_edges: vec![IntegerType::MAX; graph.edge_count()],
            new_nodes: vec![Default::default(); incremental_steps],
            new_edges: vec![Default::default(); incremental_steps],
            current_step: 0,
        }
    }

    /// Set the current incremental step of the graph.
    pub fn set_current_step(&mut self, current_step: IntegerType) {
        debug_assert!(current_step < self.new_nodes.len() && current_step < self.new_edges.len());
        self.current_step = current_step;
    }

    /// Return the nodes that are added in the current incremental step.
    pub fn new_nodes(&self) -> &Vec<Graph::NodeIndex> {
        debug_assert!(self.current_step < self.new_nodes.len());
        &self.new_nodes[self.current_step]
    }

    /// Return the edges that are added in the current incremental step.
    pub fn new_edges(&self) -> &Vec<Graph::EdgeIndex> {
        debug_assert!(self.current_step < self.new_edges.len());
        &self.new_edges[self.current_step]
    }

    /// Returns true if this node was added in the current step.
    pub fn is_new_node(&self, node_index: <Self as GraphBase>::NodeIndex) -> bool {
        debug_assert!(node_index.as_usize() < self.present_nodes.capacity());
        self.present_nodes[node_index.as_usize()] == self.current_step
    }

    /// Returns true if this edge was added in the current step.
    pub fn is_new_edge(&self, edge_index: <Self as GraphBase>::EdgeIndex) -> bool {
        debug_assert!(edge_index.as_usize() < self.present_edges.capacity());
        self.present_edges[edge_index.as_usize()] == self.current_step
    }

    /// Returns true if this node was removed in the current reverse step.
    pub fn is_newly_removed_node(&self, node_index: <Self as GraphBase>::NodeIndex) -> bool {
        debug_assert!(node_index.as_usize() < self.present_nodes.capacity());
        self.present_nodes[node_index.as_usize()] == self.current_step + 1
    }

    /// Returns true if this edge was removed in the current reverse step.
    pub fn is_newly_removed_edge(&self, edge_index: <Self as GraphBase>::EdgeIndex) -> bool {
        debug_assert!(edge_index.as_usize() < self.present_edges.capacity());
        self.present_edges[edge_index.as_usize()] == self.current_step + 1
    }
}

impl<Graph: ImmutableGraphContainer> ImmutableGraphContainer for IncrementalSubgraph<'_, Graph> {
    type NodeIndices<'a> = Filter<Graph::NodeIndices<'a>, Box<dyn 'a + Fn(&<Graph as GraphBase>::NodeIndex) -> bool>> where Self: 'a;
    type EdgeIndices<'a> = Filter<Graph::EdgeIndices<'a>, Box<dyn 'a + Fn(&<Graph as GraphBase>::EdgeIndex) -> bool>> where Self: 'a;

    fn node_indices(&self) -> Self::NodeIndices<'_> {
        self.parent_graph
            .node_indices()
            .filter(Box::new(|n| self.contains_node_index(*n)))
    }

    fn edge_indices(&self) -> Self::EdgeIndices<'_> {
        self.parent_graph
            .edge_indices()
            .filter(Box::new(|e| self.contains_edge_index(*e)))
    }
    type NodeIndicesCopied = std::vec::IntoIter<Graph::NodeIndex>;
    type EdgeIndicesCopied = std::vec::IntoIter<Graph::EdgeIndex>;
    fn node_indices_copied(&self) -> Self::NodeIndicesCopied {
        self.node_indices().collect::<Vec<_>>().into_iter()
    }

    fn edge_indices_copied(&self) -> Self::EdgeIndicesCopied {
        self.edge_indices().collect::<Vec<_>>().into_iter()
    }

    fn contains_node_index(&self, node_id: Self::NodeIndex) -> bool {
        debug_assert!(node_id.as_usize() < self.present_nodes.len());
        self.present_nodes[node_id.as_usize()] <= self.current_step
    }

    fn contains_edge_index(&self, edge_id: Self::EdgeIndex) -> bool {
        debug_assert!(edge_id.as_usize() < self.present_edges.len());
        self.present_edges[edge_id.as_usize()] <= self.current_step
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

impl<Graph: NavigableGraph> NavigableGraph for IncrementalSubgraph<'_, Graph> {
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

impl<Graph: ImmutableGraphContainer + SubgraphBase> MutableSubgraph
    for IncrementalSubgraph<'_, Graph>
{
    fn clear(&mut self) {
        unimplemented!("Not supported")
    }

    fn fill(&mut self) {
        unimplemented!("Not supported")
    }

    fn enable_node(
        &mut self,
        node_index: <<Self as SubgraphBase>::RootGraph as GraphBase>::NodeIndex,
    ) {
        debug_assert!(!self.contains_node_index(node_index));
        self.new_nodes[self.current_step].push(node_index);
        self.present_nodes[node_index.as_usize()] = self.current_step;
    }

    fn enable_edge(
        &mut self,
        edge_index: <<Self as SubgraphBase>::RootGraph as GraphBase>::EdgeIndex,
    ) {
        debug_assert!(!self.contains_edge_index(edge_index));
        self.new_edges[self.current_step].push(edge_index);
        self.present_edges[edge_index.as_usize()] = self.current_step;
    }

    fn disable_node(
        &mut self,
        _node_index: <<Self as SubgraphBase>::RootGraph as GraphBase>::NodeIndex,
    ) {
        unimplemented!("Not supported")
    }

    fn disable_edge(
        &mut self,
        _edge_index: <<Self as SubgraphBase>::RootGraph as GraphBase>::EdgeIndex,
    ) {
        unimplemented!("Not supported")
    }
}
