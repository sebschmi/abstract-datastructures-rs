use crate::index::{GraphIndex, OptionalGraphIndex};
use crate::interface::subgraph::SubgraphBase;
use crate::interface::{Edge, GraphBase, ImmutableGraphContainer};
use std::cmp::Ordering;
use std::iter::Peekable;
use std::marker::PhantomData;

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

/// An iterator that returns the union of two sorted iterators over graph indices.
pub struct UnionIndexIterator<
    Index: GraphIndex<OptionalIndex>,
    OptionalIndex: OptionalGraphIndex<Index>,
    IndexIterator0: Iterator<Item = Index>,
    IndexIterator1: Iterator<Item = Index>,
> {
    index_iterator_0: Peekable<IndexIterator0>,
    index_iterator_1: Peekable<IndexIterator1>,
    phantom_index: PhantomData<Index>,
    phantom_optional_index: PhantomData<OptionalIndex>,
}

impl<
        Index: GraphIndex<OptionalIndex>,
        OptionalIndex: OptionalGraphIndex<Index>,
        IndexIterator0: Iterator<Item = Index>,
        IndexIterator1: Iterator<Item = Index>,
    > Iterator for UnionIndexIterator<Index, OptionalIndex, IndexIterator0, IndexIterator1>
{
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.index_iterator_0.peek(), self.index_iterator_1.peek()) {
            (Some(i0), Some(i1)) => match i0.as_usize().cmp(&i1.as_usize()) {
                Ordering::Less => self.index_iterator_0.next(),
                Ordering::Equal => {
                    self.index_iterator_0.next();
                    self.index_iterator_1.next()
                }
                Ordering::Greater => self.index_iterator_1.next(),
            },
            (Some(_), None) => self.index_iterator_0.next(),
            (None, Some(_)) => self.index_iterator_1.next(),
            (None, None) => None,
        }
    }
}

impl<
        NodeIndex: GraphIndex<OptionalNodeIndex>,
        OptionalNodeIndex: OptionalGraphIndex<NodeIndex>,
        EdgeIndex: GraphIndex<OptionalEdgeIndex>,
        OptionalEdgeIndex: OptionalGraphIndex<EdgeIndex>,
        Graph0: ImmutableGraphContainer
            + SubgraphBase
            + GraphBase<
                NodeIndex = NodeIndex,
                OptionalNodeIndex = OptionalNodeIndex,
                EdgeIndex = EdgeIndex,
                OptionalEdgeIndex = OptionalEdgeIndex,
            >,
        Graph1: ImmutableGraphContainer
            + SubgraphBase
            + GraphBase<
                NodeIndex = NodeIndex,
                OptionalNodeIndex = OptionalNodeIndex,
                EdgeIndex = EdgeIndex,
                OptionalEdgeIndex = OptionalEdgeIndex,
            >,
    > ImmutableGraphContainer for UnionSubgraph<'_, Graph0, Graph1>
where
    <Self as SubgraphBase>::RootGraph: ImmutableGraphContainer,
{
    type NodeIndices<'a>
        = UnionIndexIterator<
        NodeIndex,
        OptionalNodeIndex,
        Graph0::NodeIndices<'a>,
        Graph1::NodeIndices<'a>,
    >
    where
        Self: 'a;
    type EdgeIndices<'a>
        = UnionIndexIterator<
        EdgeIndex,
        OptionalEdgeIndex,
        Graph0::EdgeIndices<'a>,
        Graph1::EdgeIndices<'a>,
    >
    where
        Self: 'a;
    type NodeIndicesCopied = UnionIndexIterator<
        NodeIndex,
        OptionalNodeIndex,
        Graph0::NodeIndicesCopied,
        Graph1::NodeIndicesCopied,
    >;
    type EdgeIndicesCopied = UnionIndexIterator<
        EdgeIndex,
        OptionalEdgeIndex,
        Graph0::EdgeIndicesCopied,
        Graph1::EdgeIndicesCopied,
    >;

    fn node_indices(&self) -> Self::NodeIndices<'_> {
        UnionIndexIterator {
            index_iterator_0: self.0.node_indices().peekable(),
            index_iterator_1: self.1.node_indices().peekable(),
            phantom_index: Default::default(),
            phantom_optional_index: Default::default(),
        }
    }

    fn edge_indices(&self) -> Self::EdgeIndices<'_> {
        UnionIndexIterator {
            index_iterator_0: self.0.edge_indices().peekable(),
            index_iterator_1: self.1.edge_indices().peekable(),
            phantom_index: Default::default(),
            phantom_optional_index: Default::default(),
        }
    }

    fn node_indices_copied(&self) -> Self::NodeIndicesCopied {
        UnionIndexIterator {
            index_iterator_0: self.0.node_indices_copied().peekable(),
            index_iterator_1: self.1.node_indices_copied().peekable(),
            phantom_index: Default::default(),
            phantom_optional_index: Default::default(),
        }
    }

    fn edge_indices_copied(&self) -> Self::EdgeIndicesCopied {
        UnionIndexIterator {
            index_iterator_0: self.0.edge_indices_copied().peekable(),
            index_iterator_1: self.1.edge_indices_copied().peekable(),
            phantom_index: Default::default(),
            phantom_optional_index: Default::default(),
        }
    }

    fn contains_node_index(&self, node_id: Self::NodeIndex) -> bool {
        self.0.contains_node_index(node_id) || self.1.contains_node_index(node_id)
    }

    fn contains_edge_index(&self, edge_id: Self::EdgeIndex) -> bool {
        self.0.contains_edge_index(edge_id) || self.1.contains_edge_index(edge_id)
    }

    fn node_count(&self) -> usize {
        self.node_indices().count()
    }

    fn edge_count(&self) -> usize {
        self.edge_indices().count()
    }

    fn node_data(&self, node_id: Self::NodeIndex) -> &Self::NodeData {
        self.root().node_data(node_id)
    }

    fn edge_data(&self, edge_id: Self::EdgeIndex) -> &Self::EdgeData {
        self.root().edge_data(edge_id)
    }

    fn edge_endpoints(&self, edge_id: Self::EdgeIndex) -> Edge<Self::NodeIndex> {
        self.root().edge_endpoints(edge_id)
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
