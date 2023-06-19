use crate::index::{GraphIndex, OptionalGraphIndex};
use crate::interface::{Edge, GraphBase, ImmutableGraphContainer, SubgraphBase};
use std::iter;
use std::marker::PhantomData;

/// A subgraph that contains all nodes and edges that another subgraph does not contain,
/// except for those edges that are missing endpoints after inversion.
pub struct InvertedSubgraph<'a, Graph>(&'a Graph);

impl<'a, Graph> InvertedSubgraph<'a, Graph> {
    /// Create a new inverted subgraph from the given graph.
    pub fn new(graph: &'a Graph) -> Self {
        Self(graph)
    }
}

impl<Graph: GraphBase> GraphBase for InvertedSubgraph<'_, Graph> {
    type NodeData = Graph::NodeData;
    type EdgeData = Graph::EdgeData;
    type OptionalNodeIndex = Graph::OptionalNodeIndex;
    type OptionalEdgeIndex = Graph::OptionalEdgeIndex;
    type NodeIndex = Graph::NodeIndex;
    type EdgeIndex = Graph::EdgeIndex;
}

impl<Graph: SubgraphBase> SubgraphBase for InvertedSubgraph<'_, Graph> {
    type RootGraph = Graph::RootGraph;

    fn root(&self) -> &Self::RootGraph {
        self.0.root()
    }
}

/// An iterator that returns all graph indices not present in another iterator.
/// The iterator `UninvertedIterator` is expected to be sorted.
pub struct InvertedIndexIterator<Index, OptionalIndex, UninvertedIterator: Iterator> {
    uninverted_iterator: iter::Peekable<UninvertedIterator>,
    current: usize,
    maximum: usize,
    phantom_index: PhantomData<Index>,
    phantom_optional_index: PhantomData<OptionalIndex>,
}

impl<
        Index: GraphIndex<OptionalIndex>,
        OptionalIndex: OptionalGraphIndex<Index>,
        UninvertedIterator: Iterator<Item = Index>,
    > Iterator for InvertedIndexIterator<Index, OptionalIndex, UninvertedIterator>
{
    type Item = UninvertedIterator::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current <= self.maximum {
            if let Some(next_missing_index) = self.uninverted_iterator.peek() {
                if self.current < next_missing_index.as_usize() {
                    let result = self.current;
                    self.current += 1;
                    return Some(result.into());
                }

                self.current += 1;
                self.uninverted_iterator.next().unwrap();
            } else {
                let result = self.current;
                self.current += 1;
                return Some(result.into());
            }
        }

        None
    }
}

impl<Graph: ImmutableGraphContainer + SubgraphBase> ImmutableGraphContainer
    for InvertedSubgraph<'_, Graph>
where
    <Graph as SubgraphBase>::RootGraph: ImmutableGraphContainer,
{
    type NodeIndices<'a> = InvertedIndexIterator<Self::NodeIndex, Self::OptionalNodeIndex, Graph::NodeIndices<'a>> where Self: 'a;
    type EdgeIndices<'a> = InvertedIndexIterator<Self::EdgeIndex, Self::OptionalEdgeIndex, Graph::EdgeIndices<'a>> where Self: 'a;

    fn node_indices(&self) -> Self::NodeIndices<'_> {
        InvertedIndexIterator {
            uninverted_iterator: self.0.node_indices().peekable(),
            current: 0,
            maximum: self.root().node_count(),
            phantom_index: Default::default(),
            phantom_optional_index: Default::default(),
        }
    }

    fn edge_indices(&self) -> Self::EdgeIndices<'_> {
        InvertedIndexIterator {
            uninverted_iterator: self.0.edge_indices().peekable(),
            current: 0,
            maximum: self.root().edge_count(),
            phantom_index: Default::default(),
            phantom_optional_index: Default::default(),
        }
    }

    fn contains_node_index(&self, node_id: Self::NodeIndex) -> bool {
        !self.0.contains_node_index(node_id)
    }

    fn contains_edge_index(&self, edge_id: Self::EdgeIndex) -> bool {
        let Edge { from_node, to_node } = self.edge_endpoints(edge_id);
        !self.0.contains_edge_index(edge_id)
            && self.contains_node_index(from_node)
            && self.contains_node_index(to_node)
    }

    fn node_count(&self) -> usize {
        self.node_indices().count()
    }

    fn edge_count(&self) -> usize {
        self.edge_indices().count()
    }

    fn node_data(&self, node_id: Self::NodeIndex) -> &Self::NodeData {
        debug_assert!(self.contains_node_index(node_id));
        self.root().node_data(node_id)
    }

    fn edge_data(&self, edge_id: Self::EdgeIndex) -> &Self::EdgeData {
        debug_assert!(self.contains_edge_index(edge_id));
        self.root().edge_data(edge_id)
    }

    fn edge_endpoints(&self, edge_id: Self::EdgeIndex) -> Edge<Self::NodeIndex> {
        debug_assert!(self.contains_edge_index(edge_id));
        self.root().edge_endpoints(edge_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::implementation::subgraph_operators::inverted_subgraph::InvertedIndexIterator;
    use crate::index::{GraphIndex, NodeIndex};

    #[test]
    fn test_inverted_index_iterator() {
        let tests = [
            (
                [3, 5, 6, 7, 10].as_slice(),
                13,
                [0, 1, 2, 4, 8, 9, 11, 12, 13].as_slice(),
            ),
            (&[0, 3, 5, 6, 7, 10, 12, 13], 13, &[1, 2, 4, 8, 9, 11]),
            (&[0, 1, 2, 3], 3, &[]),
            (&[], 3, &[0, 1, 2, 3]),
        ];

        for (iterator, maximum, expected) in tests {
            let inverted_iterator = InvertedIndexIterator {
                uninverted_iterator: iterator.iter().map(|n| NodeIndex::from(*n)).peekable(),
                current: 0,
                maximum,
                phantom_index: Default::default(),
                phantom_optional_index: Default::default(),
            };

            let actual: Vec<_> = inverted_iterator
                .map(|n: NodeIndex<usize>| n.as_usize())
                .collect();
            assert_eq!(expected, actual);
        }
    }
}
