use crate::interface::{GraphBase, SubgraphBase};

/// A subgraph built from the union of two graphs.
pub struct UnionSubgraph<'a, Graph0, Graph1>(&'a Graph0, &'a Graph1);

impl<'a, Graph0, Graph1> UnionSubgraph<'a, Graph0, Graph1> {
    /// Construct a new subgraph from the union of the two given graphs.
    pub fn new(graph0: &'a Graph0, graph1: &'a Graph1) -> Self {
        Self(graph0, graph1)
    }
}

impl<Graph0: GraphBase, Graph1: GraphBase> GraphBase for UnionSubgraph<'_, Graph0, Graph1> {
    type NodeData = Graph0::NodeData;
    type EdgeData = Graph0::EdgeData;
    type OptionalNodeIndex = Graph0::OptionalNodeIndex;
    type OptionalEdgeIndex = Graph0::OptionalEdgeIndex;
    type NodeIndex = Graph0::NodeIndex;
    type EdgeIndex = Graph0::EdgeIndex;
}

//impl<RootGraph: SubgraphBase<RootGraph = RootGraph>, Graph0: SubgraphBase<RootGraph = RootGraph>, Graph1: SubgraphBase<RootGraph = RootGraph>> SubgraphBase for UnionSubgraph<'_, Graph0, Graph1> {
impl<Graph0: SubgraphBase, Graph1: SubgraphBase> SubgraphBase
    for UnionSubgraph<'_, Graph0, Graph1>
{
    type RootGraph = Graph0::RootGraph;

    fn root(&self) -> &Self::RootGraph {
        self.0.root()
    }
}

/*impl<'a, NodeIndex, EdgeIndex, Graph0: GraphBase<NodeIndex = NodeIndex, EdgeIndex = EdgeIndex> + DecoratingSubgraph, Graph1: GraphBase<NodeIndex = NodeIndex, EdgeIndex = EdgeIndex> + DecoratingSubgraph> DecoratingSubgraph for UnionSubgraph<'a, Graph0, Graph1>
    //where <Self as GraphBase>::NodeIndex = NodeIndex
 //: GraphBase<NodeIndex = NodeIndex, EdgeIndex = EdgeIndex>
{
    type ParentGraph = Graph0;
    type ParentGraphRef = &'a Graph0;

    fn new_empty(graph: Self::ParentGraphRef) -> Self {
        unimplemented!("Construct this type only using new");
    }

    fn new_full(graph: Self::ParentGraphRef) -> Self {
        unimplemented!("Construct this type only using new");
    }

    fn clear(&mut self) {
        unimplemented!("Not implementable for non-mutable subgraph decorator")
    }

    fn fill(&mut self) {
        unimplemented!("Not implementable for non-mutable subgraph decorator")
    }

    fn parent_graph(&self) -> &Self::ParentGraph {
        unimplemented!("Not implementable for binary subgraph decorator")
    }

    fn contains_node(&self, node_index: <Self::ParentGraph as GraphBase>::NodeIndex) -> bool {
        self.0.contains_node(node_index) || self.1.contains_node(node_index)
    }

    fn contains_edge(&self, edge_index: <Self::ParentGraph as GraphBase>::EdgeIndex) -> bool {
        self.0.contains_edge(edge_index) || self.1.contains_edge(edge_index)
    }

    fn add_node(&mut self, node_index: <Self::ParentGraph as GraphBase>::NodeIndex) {
        unimplemented!("Not implementable for non-mutable subgraph decorator")
    }

    fn add_edge(&mut self, edge_index: <Self::ParentGraph as GraphBase>::EdgeIndex) {
        unimplemented!("Not implementable for non-mutable subgraph decorator")
    }

    fn remove_node(&mut self, node_index: <Self::ParentGraph as GraphBase>::NodeIndex) {
        unimplemented!("Not implementable for non-mutable subgraph decorator")
    }

    fn remove_edge(&mut self, edge_index: <Self::ParentGraph as GraphBase>::EdgeIndex) {
        unimplemented!("Not implementable for non-mutable subgraph decorator")
    }

    fn node_count(&self) -> usize {
        unimplemented!("Will not implement if not necessary")
    }

    fn edge_count(&self) -> usize {
        unimplemented!("Will not implement if not necessary")
    }
}*/
