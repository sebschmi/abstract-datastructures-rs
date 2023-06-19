use crate::index::GraphIndex;
use crate::interface::{Edge, GraphBase, ImmutableGraphContainer, MutableSubgraph, SubgraphBase};
use bitvec::bitvec;
use bitvec::vec::BitVec;

/// A subgraph implementation based on bitvectors.
/// This subgraph only allows to enable or disable nodes,
/// and edges are automatically contained if their endpoints exist.
pub struct InducedBitVectorSubgraph<'a, Graph> {
    parent_graph: &'a Graph,
    present_nodes: BitVec,
}

impl<'a, Graph: SubgraphBase> InducedBitVectorSubgraph<'a, Graph>
where
    Graph::RootGraph: ImmutableGraphContainer,
{
    /// Constructs a new instance decorating the given graph.
    /// The subgraph is initialised empty.
    pub fn new_empty(parent_graph: &'a Graph) -> Self {
        Self {
            parent_graph,
            present_nodes: bitvec![0; parent_graph.root().node_count()],
        }
    }
}

impl<'a, Graph: GraphBase> GraphBase for InducedBitVectorSubgraph<'a, Graph> {
    type NodeData = Graph::NodeData;
    type EdgeData = Graph::EdgeData;
    type OptionalNodeIndex = Graph::OptionalNodeIndex;
    type OptionalEdgeIndex = Graph::OptionalEdgeIndex;
    type NodeIndex = Graph::NodeIndex;
    type EdgeIndex = Graph::EdgeIndex;
}

impl<'a, Graph: ImmutableGraphContainer> ImmutableGraphContainer
    for InducedBitVectorSubgraph<'a, Graph>
{
    type NodeIndices<'node_indices> = std::iter::Filter<Graph::NodeIndices<'node_indices>, Box<dyn 'node_indices + Fn(&Graph::NodeIndex) -> bool>> where Self: 'node_indices, Graph: 'node_indices;
    type EdgeIndices<'edge_indices> = std::iter::Filter<Graph::EdgeIndices<'edge_indices>, Box<dyn 'edge_indices + Fn(&Graph::EdgeIndex) -> bool>> where Self: 'edge_indices, Graph: 'edge_indices;

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
        debug_assert!(self.parent_graph.contains_edge_index(edge_id));
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

impl<'a, Graph: ImmutableGraphContainer + SubgraphBase> SubgraphBase
    for InducedBitVectorSubgraph<'a, Graph>
{
    type RootGraph = Graph::RootGraph;

    fn root(&self) -> &Self::RootGraph {
        self.parent_graph.root()
    }
}

impl<'a, Graph: ImmutableGraphContainer + SubgraphBase> MutableSubgraph
    for InducedBitVectorSubgraph<'a, Graph>
where
    Self: GraphBase<
        NodeIndex = <Graph as GraphBase>::NodeIndex,
        EdgeIndex = <Graph as GraphBase>::EdgeIndex,
    >,
{
    fn clear(&mut self) {
        self.present_nodes.clear();
    }

    fn fill(&mut self) {
        self.parent_graph
            .node_indices()
            .for_each(|node_index| self.enable_node(node_index));
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
        _edge_index: <<Self as SubgraphBase>::RootGraph as GraphBase>::EdgeIndex,
    ) {
        unimplemented!("the induced bitvector subgraph allows only nodes to be enabled/disabled");
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
        _edge_index: <<Self as SubgraphBase>::RootGraph as GraphBase>::EdgeIndex,
    ) {
        unimplemented!("the induced bitvector subgraph allows only nodes to be enabled/disabled");
    }
}
