/// A subgraph that contains all nodes that another subgraph does not contain.
pub struct InvertedSubgraph<'a, Graph>(&'a Graph);

impl<'a, Graph> InvertedSubgraph<'a, Graph> {
    /// Create a new inverted subgraph from the given graph.
    pub fn new(graph: &'a Graph) -> Self {
        Self(graph)
    }
}

/*impl<'a, Graph: DecoratingSubgraph> DecoratingSubgraph for InvertedSubgraph<'a, Graph> {
    type ParentGraph = Graph;
    type ParentGraphRef = &'a Graph;

    fn new_empty(_graph: Self::ParentGraphRef) -> Self {
        unimplemented!("Construct this type only using new");
    }

    fn new_full(_graph: Self::ParentGraphRef) -> Self {
        unimplemented!("Construct this type only using new");
    }

    fn clear(&mut self) {
        // self.0.fill()
        unimplemented!("Not implementable for non-mutable subgraph decorator")
    }

    fn fill(&mut self) {
        // self.0.clear()
        unimplemented!("Not implementable for non-mutable subgraph decorator")
    }

    fn parent_graph(&self) -> &Self::ParentGraph {
        self.0
    }

    fn contains_node(&self, node_index: <Self::ParentGraph as GraphBase>::NodeIndex) -> bool {
        !self.0.contains_node(node_index)
    }

    fn contains_edge(&self, edge_index: <Self::ParentGraph as GraphBase>::EdgeIndex) -> bool {
        !self.0.contains_edge(edge_index)
    }

    fn add_node(&mut self, node_index: <Self::ParentGraph as GraphBase>::NodeIndex) {
        // self.0.remove_node(node_index)
        unimplemented!("Not implementable for non-mutable subgraph decorator")
    }

    fn add_edge(&mut self, edge_index: <Self::ParentGraph as GraphBase>::EdgeIndex) {
        // self.0.remove_edge(edge_index)
        unimplemented!("Not implementable for non-mutable subgraph decorator")
    }

    fn remove_node(&mut self, node_index: <Self::ParentGraph as GraphBase>::NodeIndex) {
        // self.0.add_node(node_index)
        unimplemented!("Not implementable for non-mutable subgraph decorator")
    }

    fn remove_edge(&mut self, edge_index: <Self::ParentGraph as GraphBase>::EdgeIndex) {
        // self.0.add_edge(edge_index)
        unimplemented!("Not implementable for non-mutable subgraph decorator")
    }

    fn node_count(&self) -> usize {
        unimplemented!("Will not implement if not necessary")
    }

    fn edge_count(&self) -> usize {
        unimplemented!("Will not implement if not necessary")
    }
}*/

/// A subgraph built from the union of two graphs.
pub struct UnionSubgraph<'a, Graph0, Graph1>(&'a Graph0, &'a Graph1);

impl<'a, Graph0, Graph1> UnionSubgraph<'a, Graph0, Graph1> {
    /// Construct a new subgraph from the union of the two given graphs.
    pub fn new(graph0: &'a Graph0, graph1: &'a Graph1) -> Self {
        Self(graph0, graph1)
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
