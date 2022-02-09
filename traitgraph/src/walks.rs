use crate::interface::{GraphBase, StaticGraph};
use traitsequence::interface::Sequence;

/// A sequence of nodes in a graph, where each consecutive pair of nodes is connected by an edge.
pub trait NodeWalk<'a, Graph: GraphBase, NodeSubwalk: NodeWalk<'a, Graph, NodeSubwalk> + ?Sized>:
    Sequence<'a, Graph::NodeIndex, NodeSubwalk>
where
    Graph::NodeIndex: 'a,
{
    /// Returns the edge walk represented by this node walk.
    /// If there is a consecutive pair of nodes with a multiedge, then None is returned.
    /// If this walk contains less than two nodes, then None is returned.
    /// If there is a consecutive pair of node not connected by an edge, then this method panics.
    fn clone_as_edge_walk<ResultWalk: From<Vec<Graph::EdgeIndex>>>(
        &'a self,
        graph: &Graph,
    ) -> Option<ResultWalk>
    where
        Graph: StaticGraph,
    {
        if self.len() < 2 {
            return None;
        }

        let mut walk = Vec::new();
        for node_pair in self.iter().take(self.len() - 1).zip(self.iter().skip(1)) {
            let from = *node_pair.0;
            let to = *node_pair.1;
            let mut edges_between = graph.edges_between(from, to);

            if let Some(edge) = edges_between.next() {
                walk.push(edge);
            } else {
                panic!("Not a valid node walk");
            }
            if edges_between.next().is_some() {
                return None;
            }
        }

        Some(ResultWalk::from(walk))
    }

    /// Returns true if this is a proper subwalk of the given walk.
    /// Proper means that the walks are not equal.
    fn is_proper_subwalk_of(&'a self, other: &Self) -> bool
    where
        Graph::NodeIndex: Eq,
    {
        self.is_proper_subsequence_of(other)
    }
}

/// A sequence of edges in a graph, where each consecutive pair of edges is connected by a node.
pub trait EdgeWalk<'a, Graph: GraphBase, EdgeSubwalk: EdgeWalk<'a, Graph, EdgeSubwalk> + ?Sized>:
    Sequence<'a, Graph::EdgeIndex, EdgeSubwalk>
where
    Graph::EdgeIndex: 'a,
{
    /// Returns the node walk represented by this edge walk.
    /// If this walk contains no edge, then None is returned.
    /// If there is a consecutive pair of edges not connected by a node, then this method panics.
    fn clone_as_node_walk<ResultWalk: From<Vec<Graph::NodeIndex>>>(
        &'a self,
        graph: &Graph,
    ) -> Option<ResultWalk>
    where
        Graph: StaticGraph,
    {
        if self.is_empty() {
            return None;
        }

        let mut walk = vec![
            graph
                .edge_endpoints(self.first().cloned().unwrap())
                .from_node,
        ];
        for edge_pair in self.iter().take(self.len() - 1).zip(self.iter().skip(1)) {
            let node = graph.edge_endpoints(*edge_pair.0).to_node;
            debug_assert_eq!(
                node,
                graph.edge_endpoints(*edge_pair.1).from_node,
                "Not a valid edge walk"
            );
            walk.push(node);
        }
        walk.push(graph.edge_endpoints(self.last().cloned().unwrap()).to_node);

        Some(ResultWalk::from(walk))
    }

    /// Returns true if this is a proper subwalk of the given walk.
    /// Proper means that the walks are not equal.
    fn is_proper_subwalk_of(&'a self, other: &Self) -> bool
    where
        Graph::EdgeIndex: Eq,
    {
        self.is_proper_subsequence_of(other)
    }

    /// Returns true if this is a valid circular walk in the given graph.
    fn is_circular_walk(&'a self, graph: &Graph) -> bool
    where
        Graph: StaticGraph,
    {
        if self.is_empty() {
            return true;
        }

        let mut connecting_node = graph.edge_endpoints(*self.last().unwrap()).to_node;
        for &edge in self.iter() {
            let edge_endpoints = graph.edge_endpoints(edge);
            if edge_endpoints.from_node != connecting_node {
                return false;
            } else {
                connecting_node = edge_endpoints.to_node;
            }
        }

        true
    }
}

////////////////////
////// Slices //////
////////////////////

impl<'a, Graph: GraphBase> NodeWalk<'a, Graph, [Graph::NodeIndex]> for [Graph::NodeIndex] where
    Graph::NodeIndex: 'a
{
}

impl<'a, Graph: GraphBase> EdgeWalk<'a, Graph, [Graph::EdgeIndex]> for [Graph::EdgeIndex] where
    Graph::EdgeIndex: 'a
{
}

/////////////////////////
////// VecNodeWalk //////
/////////////////////////

/// A node walk that is represented as a vector of node indices.
pub type VecNodeWalk<Graph> = Vec<<Graph as GraphBase>::NodeIndex>;

impl<'a, Graph: GraphBase> NodeWalk<'a, Graph, [Graph::NodeIndex]> for VecNodeWalk<Graph> where
    Graph::NodeIndex: 'a
{
}

/////////////////////////
////// VecEdgeWalk //////
/////////////////////////

/// An edge walk that is represented as a vector of edge indices.
pub type VecEdgeWalk<Graph> = Vec<<Graph as GraphBase>::EdgeIndex>;

impl<'a, Graph: GraphBase> EdgeWalk<'a, Graph, [Graph::EdgeIndex]> for VecEdgeWalk<Graph> where
    Graph::EdgeIndex: 'a
{
}
