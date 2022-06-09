use crate::index::{GraphIndex, GraphIndices};
use crate::interface::{
    Edge, GraphBase, ImmutableGraphContainer, MutableGraphContainer, NavigableGraph, Neighbor,
};
use num_traits::{PrimInt, ToPrimitive};
use petgraph::graph::{DiGraph, Edges, EdgesConnecting};
use petgraph::visit::EdgeRef;
use petgraph::{Directed, Direction};
use std::iter::Map;

pub use petgraph;

/// A wrapper around the [petgraph::graph::Graph] type replacing its methods with implementations of our traits.
#[derive(Debug, Clone)]
pub struct PetGraph<NodeData, EdgeData>(DiGraph<NodeData, EdgeData, usize>);

impl<NodeData, EdgeData> PetGraph<NodeData, EdgeData> {
    /// Create a new graph implemented using the `petgraph::graph::Graph` type.
    pub fn new() -> PetGraph<NodeData, EdgeData> {
        PetGraph(DiGraph::<NodeData, EdgeData, usize>::default())
    }
}

impl<NodeData, EdgeData> GraphBase for PetGraph<NodeData, EdgeData> {
    type NodeData = NodeData;
    type EdgeData = EdgeData;
    type OptionalNodeIndex = crate::index::OptionalNodeIndex<usize>;
    type OptionalEdgeIndex = crate::index::OptionalEdgeIndex<usize>;
    type NodeIndex = crate::index::NodeIndex<usize>;
    type EdgeIndex = crate::index::EdgeIndex<usize>;
}

impl<NodeData, EdgeData> ImmutableGraphContainer for PetGraph<NodeData, EdgeData> {
    fn node_indices(&self) -> GraphIndices<Self::NodeIndex, Self::OptionalNodeIndex> {
        GraphIndices::from((0, self.node_count()))
    }

    fn edge_indices(&self) -> GraphIndices<Self::EdgeIndex, Self::OptionalEdgeIndex> {
        GraphIndices::from((0, self.edge_count()))
    }

    fn contains_node_index(&self, node_id: Self::NodeIndex) -> bool {
        self.0.node_weight(node_id.into()).is_some()
    }

    fn contains_edge_index(&self, edge_id: Self::EdgeIndex) -> bool {
        self.0.edge_weight(edge_id.into()).is_some()
    }

    fn node_count(&self) -> usize {
        self.0.node_count()
    }

    fn edge_count(&self) -> usize {
        self.0.edge_count()
    }

    fn node_data(&self, node_id: Self::NodeIndex) -> &Self::NodeData {
        self.0.node_weight(node_id.into()).unwrap()
    }

    fn edge_data(&self, edge_id: Self::EdgeIndex) -> &Self::EdgeData {
        self.0.edge_weight(edge_id.into()).unwrap()
    }

    fn node_data_mut(&mut self, node_id: Self::NodeIndex) -> &mut Self::NodeData {
        self.0.node_weight_mut(node_id.into()).unwrap()
    }

    fn edge_data_mut(&mut self, edge_id: Self::EdgeIndex) -> &mut Self::EdgeData {
        self.0.edge_weight_mut(edge_id.into()).unwrap()
    }

    fn contains_edge_between(&self, from: Self::NodeIndex, to: Self::NodeIndex) -> bool {
        self.0
            .edges_connecting(from.into(), to.into())
            .next()
            .is_some()
    }

    fn edge_count_between(&self, from: Self::NodeIndex, to: Self::NodeIndex) -> usize {
        self.0.edges_connecting(from.into(), to.into()).count()
    }

    fn edge_endpoints(&self, edge_id: Self::EdgeIndex) -> Edge<Self::NodeIndex> {
        let endpoints = self.0.edge_endpoints(edge_id.into()).unwrap();
        Edge {
            from_node: endpoints.0.index().into(),
            to_node: endpoints.1.index().into(),
        }
    }
}

impl<NodeData, EdgeData> MutableGraphContainer for PetGraph<NodeData, EdgeData> {
    fn add_node(&mut self, node_data: NodeData) -> Self::NodeIndex {
        self.0.add_node(node_data).index().into()
    }

    fn add_edge(
        &mut self,
        from: Self::NodeIndex,
        to: Self::NodeIndex,
        edge_data: EdgeData,
    ) -> Self::EdgeIndex {
        self.0
            .add_edge(from.into(), to.into(), edge_data)
            .index()
            .into()
    }

    fn remove_node(&mut self, node_id: Self::NodeIndex) -> Option<NodeData> {
        self.0.remove_node(node_id.into())
    }

    fn remove_edge(&mut self, edge_id: Self::EdgeIndex) -> Option<EdgeData> {
        self.0.remove_edge(edge_id.into())
    }

    fn remove_edges_sorted(&mut self, edge_ids: &[Self::EdgeIndex]) {
        edge_ids.windows(2).for_each(|w| debug_assert!(w[0] < w[1]));

        for edge_id in edge_ids.iter().rev() {
            self.remove_edge(*edge_id);
        }
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}

type PetgraphNeighborTranslator<'a, EdgeData, NodeIndex, EdgeIndex> = Map<
    Edges<'a, EdgeData, Directed, usize>,
    fn(petgraph::graph::EdgeReference<'a, EdgeData, usize>) -> Neighbor<NodeIndex, EdgeIndex>,
>;

type PetgraphRestrictedNeighborTranslator<'a, EdgeData, EdgeIndex> = Map<
    EdgesConnecting<'a, EdgeData, Directed, usize>,
    fn(petgraph::graph::EdgeReference<'a, EdgeData, usize>) -> EdgeIndex,
>;

impl<'a, NodeData, EdgeData: 'a> NavigableGraph<'a> for PetGraph<NodeData, EdgeData> {
    type OutNeighbors = PetgraphNeighborTranslator<
        'a,
        EdgeData,
        <Self as GraphBase>::NodeIndex,
        <Self as GraphBase>::EdgeIndex,
    >;
    type InNeighbors = PetgraphNeighborTranslator<
        'a,
        EdgeData,
        <Self as GraphBase>::NodeIndex,
        <Self as GraphBase>::EdgeIndex,
    >;
    type EdgesBetween =
        PetgraphRestrictedNeighborTranslator<'a, EdgeData, <Self as GraphBase>::EdgeIndex>;

    fn out_neighbors(&'a self, node_id: <Self as GraphBase>::NodeIndex) -> Self::OutNeighbors {
        debug_assert!(self.contains_node_index(node_id));
        self.0
            .edges_directed(node_id.into(), Direction::Outgoing)
            .map(|edge| Neighbor {
                edge_id: <Self as GraphBase>::EdgeIndex::from(edge.id().index()),
                node_id: <Self as GraphBase>::NodeIndex::from(edge.target().index()),
            })
    }

    fn in_neighbors(&'a self, node_id: <Self as GraphBase>::NodeIndex) -> Self::InNeighbors {
        debug_assert!(self.contains_node_index(node_id));
        self.0
            .edges_directed(node_id.into(), Direction::Incoming)
            .map(|edge| Neighbor {
                edge_id: <Self as GraphBase>::EdgeIndex::from(edge.id().index()),
                node_id: <Self as GraphBase>::NodeIndex::from(edge.source().index()),
            })
    }

    fn edges_between(
        &'a self,
        from_node_id: <Self as GraphBase>::NodeIndex,
        to_node_id: <Self as GraphBase>::NodeIndex,
    ) -> Self::EdgesBetween {
        debug_assert!(self.contains_node_index(from_node_id));
        debug_assert!(self.contains_node_index(to_node_id));
        self.0
            .edges_connecting(from_node_id.into(), to_node_id.into())
            .map(|edge| <Self as GraphBase>::EdgeIndex::from(edge.id().index()))
    }
}

impl<IndexType: PrimInt + ToPrimitive + petgraph::graph::IndexType>
    From<crate::index::NodeIndex<IndexType>> for petgraph::graph::NodeIndex<IndexType>
{
    fn from(index: crate::index::NodeIndex<IndexType>) -> Self {
        petgraph::graph::NodeIndex::new(index.as_usize())
    }
}

impl<IndexType: PrimInt + ToPrimitive + petgraph::graph::IndexType>
    From<crate::index::EdgeIndex<IndexType>> for petgraph::graph::EdgeIndex<IndexType>
{
    fn from(index: crate::index::EdgeIndex<IndexType>) -> Self {
        petgraph::graph::EdgeIndex::new(index.as_usize())
    }
}

impl<NodeData: PartialEq, EdgeData: PartialEq> PartialEq for PetGraph<NodeData, EdgeData> {
    fn eq(&self, other: &Self) -> bool {
        self.node_count() == other.node_count()
            || self.edge_count() == other.edge_count()
                && self
                    .node_indices()
                    .zip(other.node_indices())
                    .all(|(a, b)| a == b && self.node_data(a) == other.node_data(b))
                && self.edge_indices().zip(other.edge_indices()).all(|(a, b)| {
                    a == b
                        && self.edge_endpoints(a) == other.edge_endpoints(b)
                        && self.edge_data(a) == other.edge_data(b)
                })
    }
}

impl<NodeData: Eq, EdgeData: Eq> Eq for PetGraph<NodeData, EdgeData> {}

impl<NodeData, EdgeData> Default for PetGraph<NodeData, EdgeData> {
    fn default() -> Self {
        Self(Default::default())
    }
}
