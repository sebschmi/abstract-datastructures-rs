use crate::index::{GraphIndex, OptionalGraphIndex};
use crate::interface::subgraph::SubgraphBase;
use crate::interface::{Edge, GraphBase, ImmutableGraphContainer};
use std::iter::Filter;
use std::marker::PhantomData;

type IntegerType = usize;

/// A subgraph that stores the presence or absence of a node using integers.
///
/// Additionally, this subgraph has a current step that can be altered.
/// Nodes are added with that step, and only nodes with a step lower or equal to the current one are counted as present.
/// This allows to combine multiple subgraphs into one, if they are totally ordered by the subset relation.
///
/// In this variant of the incremental subgraph, edges are part of a subgraph if their endpoints are part of the subgraph.
pub struct InducedIncrementalSubgraph<'a, Graph: GraphBase> {
    parent_graph: &'a Graph,
    present_nodes: Vec<IntegerType>,
    new_nodes: Vec<Vec<Graph::NodeIndex>>,
    current_step: IntegerType,
}

impl<Graph: GraphBase> GraphBase for InducedIncrementalSubgraph<'_, Graph> {
    type NodeData = Graph::NodeData;
    type EdgeData = Graph::EdgeData;
    type OptionalNodeIndex = Graph::OptionalNodeIndex;
    type OptionalEdgeIndex = Graph::OptionalEdgeIndex;
    type NodeIndex = Graph::NodeIndex;
    type EdgeIndex = Graph::EdgeIndex;
}

impl<Graph: SubgraphBase> SubgraphBase for InducedIncrementalSubgraph<'_, Graph> {
    type RootGraph = Graph::RootGraph;

    fn root(&self) -> &Self::RootGraph {
        self.parent_graph.root()
    }
}

impl<'a, Graph: ImmutableGraphContainer> InducedIncrementalSubgraph<'a, Graph> {
    /// Create an incremental subgraph with the given amount of incremental steps.
    pub fn new_with_incremental_steps(graph: &'a Graph, incremental_steps: usize) -> Self {
        Self {
            parent_graph: graph,
            present_nodes: vec![IntegerType::MAX; graph.node_count()],
            new_nodes: vec![Default::default(); incremental_steps],
            current_step: 0,
        }
    }

    /// Set the current incremental step of the graph.
    pub fn set_current_step(&mut self, current_step: IntegerType) {
        debug_assert!(current_step < self.new_nodes.len());
        self.current_step = current_step;
    }

    /// Return the nodes that are added in the current incremental step.
    pub fn new_nodes(&self) -> &Vec<Graph::NodeIndex> {
        debug_assert!(self.current_step < self.new_nodes.len());
        &self.new_nodes[self.current_step]
    }

    /* /// Return the edges that are added in the current incremental step.
    pub fn new_edges(&self) -> &Vec<Graph::EdgeIndex> {
        debug_assert!(self.current_step < self.new_nodes.len());
        &self.new_edges[self.current_step]
    }*/

    /// Returns true if this node was added in the current step.
    pub fn is_new_node(&self, node_index: <Self as GraphBase>::NodeIndex) -> bool {
        debug_assert!(node_index.as_usize() < self.present_nodes.capacity());
        self.present_nodes[node_index.as_usize()] == self.current_step
    }

    /* /// Returns true if this edge was added in the current step.
    pub fn is_new_edge(&self, edge_index: <Self as GraphBase>::EdgeIndex) -> bool {
        debug_assert!(edge_index.as_usize() < self.present_edges.capacity());
        self.present_edges[edge_index.as_usize()] == self.current_step
    }*/

    /// Returns true if this node was removed in the current reverse step.
    pub fn is_newly_removed_node(&self, node_index: <Self as GraphBase>::NodeIndex) -> bool {
        debug_assert!(node_index.as_usize() < self.present_nodes.capacity());
        self.present_nodes[node_index.as_usize()] == self.current_step + 1
    }

    /* /// Returns true if this edge was removed in the current reverse step.
    pub fn is_newly_removed_edge(&self, edge_index: <Self as GraphBase>::EdgeIndex) -> bool {
        debug_assert!(edge_index.as_usize() < self.present_edges.capacity());
        self.present_edges[edge_index.as_usize()] == self.current_step + 1
    }*/
}

/// An iterator over the node indices of a subgraph.
pub struct FilterNodeIndexIterator<
    'a,
    NodeIndex,
    OptionalNodeIndex,
    NodeIndices: Iterator<Item = NodeIndex>,
    Graph,
> {
    iterator: NodeIndices,
    graph: &'a Graph,
    phantom_optional_index: PhantomData<OptionalNodeIndex>,
}

impl<
        NodeIndex: GraphIndex<OptionalNodeIndex>,
        OptionalNodeIndex: OptionalGraphIndex<NodeIndex>,
        NodeIndices: Iterator<Item = NodeIndex>,
        Graph: GraphBase<NodeIndex = NodeIndex> + ImmutableGraphContainer,
    > Iterator for FilterNodeIndexIterator<'_, NodeIndex, OptionalNodeIndex, NodeIndices, Graph>
{
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .by_ref()
            .find(|&index| self.graph.contains_node_index(index))
    }
}

impl<Graph: ImmutableGraphContainer> ImmutableGraphContainer
    for InducedIncrementalSubgraph<'_, Graph>
{
    type NodeIndices<'a>
        = FilterNodeIndexIterator<
        'a,
        <Self as GraphBase>::NodeIndex,
        <Self as GraphBase>::OptionalNodeIndex,
        <Graph as ImmutableGraphContainer>::NodeIndices<'a>,
        Self,
    >
    where
        Self: 'a;
    type EdgeIndices<'a>
        = Filter<Graph::EdgeIndices<'a>, Box<dyn 'a + Fn(&<Graph as GraphBase>::EdgeIndex) -> bool>>
    where
        Self: 'a;

    fn node_indices(&self) -> Self::NodeIndices<'_> {
        FilterNodeIndexIterator {
            iterator: self.parent_graph.node_indices(),
            graph: self,
            phantom_optional_index: Default::default(),
        }
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
        let Edge { from_node, to_node } = self.edge_endpoints(edge_id);
        self.contains_node_index(from_node) && self.contains_node_index(to_node)
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
        self.parent_graph.edge_endpoints(edge_id)
    }
}
