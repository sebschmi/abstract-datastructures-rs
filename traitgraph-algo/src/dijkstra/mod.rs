use crate::dijkstra::epoch_array_dijkstra_node_weight_array::EpochNodeWeightArray;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Add;
use traitgraph::index::{GraphIndex, NodeIndex};
use traitgraph::interface::{GraphBase, StaticGraph};

mod dijkstra_weight_implementations;

/// Using an epoched array as [NodeWeightArray].
pub mod epoch_array_dijkstra_node_weight_array;
/// Contains the implementation of the [NodeWeightArray] as [hashbrown::HashMap].
#[cfg(feature = "hashbrown_dijkstra_node_weight_array")]
pub mod hashbrown_dijkstra_node_weight_array;

/// A Dijkstra implementation with a set of common optimisations.
pub type DefaultDijkstra<Graph, WeightType> = Dijkstra<
    Graph,
    WeightType,
    EpochNodeWeightArray<WeightType>,
    BinaryHeap<std::cmp::Reverse<(WeightType, <Graph as GraphBase>::NodeIndex)>>,
>;

/// A weight-type usable in Dijkstra's algorithm.
pub trait DijkstraWeight: Ord + Add<Output = Self> + Sized + Clone {
    /// The infinity value of this type.
    fn infinity() -> Self;

    /// The zero value of this type.
    fn zero() -> Self;
}

/// Edge data that has a weight usable for shortest path computation.
pub trait DijkstraWeightedEdgeData<WeightType: DijkstraWeight> {
    /// The weight of the edge.
    fn weight(&self) -> WeightType;
}

impl<WeightType: DijkstraWeight + Copy> DijkstraWeightedEdgeData<WeightType> for WeightType {
    #[inline]
    fn weight(&self) -> WeightType {
        *self
    }
}

/// An array to store minimal node weights for Dijkstra's algorithm.
pub trait NodeWeightArray<WeightType> {
    /// Create a new NodeWeightArray of given size.
    fn new(size: usize) -> Self;

    /// Returns the current weight of the given node index.
    fn get(&self, node_index: usize) -> WeightType;

    /// Returns the current weight of the given node index as mutable reference.
    fn get_mut<'this: 'result, 'result>(
        &'this mut self,
        node_index: usize,
    ) -> &'result mut WeightType;

    /// Sets the current weight of the given node index.
    fn set(&mut self, node_index: usize, weight: WeightType);

    /// Resets the weights of all node indices to infinity
    fn clear(&mut self);
}

impl<WeightType: DijkstraWeight + Copy> NodeWeightArray<WeightType> for Vec<WeightType> {
    fn new(size: usize) -> Self {
        vec![WeightType::infinity(); size]
    }

    #[inline]
    fn get(&self, node_index: usize) -> WeightType {
        self[node_index]
    }

    #[inline]
    fn get_mut<'this: 'result, 'result>(
        &'this mut self,
        node_index: usize,
    ) -> &'result mut WeightType {
        &mut self[node_index]
    }

    #[inline]
    fn set(&mut self, node_index: usize, weight: WeightType) {
        self[node_index] = weight;
    }

    fn clear(&mut self) {
        for entry in self.iter_mut() {
            *entry = WeightType::infinity();
        }
    }
}

/// A data structure that decides whether a given node index is a target of the current Dijkstra search.
pub trait DijkstraTargetMap<Graph: GraphBase> {
    /// Returns true if the given node index is a target of the current Dijkstra search.
    fn is_target(&self, node_index: Graph::NodeIndex) -> bool;
}

impl<Graph: GraphBase> DijkstraTargetMap<Graph> for Vec<bool> {
    fn is_target(&self, node_index: Graph::NodeIndex) -> bool {
        self[node_index.as_usize()]
    }
}

impl<IndexType: Sized + Eq, Graph: GraphBase<NodeIndex = NodeIndex<IndexType>>>
    DijkstraTargetMap<Graph> for NodeIndex<IndexType>
{
    fn is_target(&self, node_index: Graph::NodeIndex) -> bool {
        *self == node_index
    }
}

/// A min-heap used in Dijkstra's shortest path algorithm.
pub trait DijkstraHeap<WeightType, IndexType>: Default {
    /// Insert an index-weight pair into the heap.
    fn insert(&mut self, weight: WeightType, index: IndexType);

    /// Remove the weight and index with the smallest weight from the heap.
    fn remove_min(&mut self) -> Option<(WeightType, IndexType)>;

    /// Remove all entries from the heap.
    fn clear(&mut self);
}

impl<WeightType: Ord, IndexType: Ord> DijkstraHeap<WeightType, IndexType>
    for BinaryHeap<std::cmp::Reverse<(WeightType, IndexType)>>
{
    fn insert(&mut self, weight: WeightType, index: IndexType) {
        self.push(std::cmp::Reverse((weight, index)));
    }

    fn remove_min(&mut self) -> Option<(WeightType, IndexType)> {
        self.pop().map(|packed| packed.0)
    }

    fn clear(&mut self) {
        self.clear()
    }
}

/// Data structure for Dijkstra's shortest path algorithm.
///
/// This variant of Dijkstra's algorithm supports only computing the length of a shortest path, and not the shortest path itself.
/// Therefore it does not need an array of back pointers for each node, saving a bit of memory.
pub struct Dijkstra<
    Graph: GraphBase,
    WeightType: DijkstraWeight,
    NodeWeights: NodeWeightArray<WeightType>,
    Heap: DijkstraHeap<WeightType, Graph::NodeIndex>,
> {
    heap: Heap,
    // back_pointers: Vec<Graph::OptionalNodeIndex>,
    node_weights: NodeWeights,
    graph: PhantomData<Graph>,
    _weight_type_phantom: PhantomData<WeightType>,
}

impl<
        WeightType: DijkstraWeight + Eq + Debug,
        EdgeData: DijkstraWeightedEdgeData<WeightType>,
        Graph: StaticGraph<EdgeData = EdgeData>,
        NodeWeights: NodeWeightArray<WeightType>,
        Heap: DijkstraHeap<WeightType, Graph::NodeIndex>,
    > Dijkstra<Graph, WeightType, NodeWeights, Heap>
{
    /// Create the data structures for the given graph.
    pub fn new(graph: &Graph) -> Self {
        Self {
            heap: Default::default(),
            // back_pointers: vec![Default::default(); graph.node_count()],
            node_weights: NodeWeights::new(graph.node_count()),
            graph: Default::default(),
            _weight_type_phantom: Default::default(),
        }
    }

    /// Compute the shortest paths from source to all targets, with given maximum weight.
    #[inline(never)]
    #[allow(clippy::too_many_arguments)]
    pub fn shortest_path_lens<TargetMap: DijkstraTargetMap<Graph>>(
        &mut self,
        graph: &Graph,
        source: Graph::NodeIndex,
        targets: &TargetMap,
        target_amount: usize,
        max_weight: WeightType,
        forbid_source_target: bool,
        distances: &mut Vec<(Graph::NodeIndex, WeightType)>,
    ) {
        //println!("Shortest path lens of {}", source.as_usize());
        self.heap.insert(WeightType::zero(), source);
        //self.back_pointers[source.as_usize()] = source.into();
        self.node_weights.set(source.as_usize(), WeightType::zero());
        distances.clear();

        //let mut iterations = 0;
        //let mut unnecessary_iterations = 0;
        //let max_iterations = self.graph.node_count();
        while let Some((weight, node_index)) = self.heap.remove_min() {
            //iterations += 1;
            //println!("Finalising node {}", node_index.as_usize());
            // Check if the node was already processed
            let actual_weight = self.node_weights.get(node_index.as_usize());
            if actual_weight < weight {
                //unnecessary_iterations += 1;
                continue;
            }
            debug_assert_eq!(actual_weight, weight);

            // Check if we are still lower than or equal to max_weight
            if weight > max_weight {
                //println!("Aborting early by max_weight after {}/{} iterations of which {} are unnecessary", iterations, max_iterations, unnecessary_iterations);
                break;
            }

            // Check if we found a target
            if targets.is_target(node_index) && (!forbid_source_target || node_index != source) {
                distances.push((node_index, weight.clone()));

                // Check if we already found all paths
                if distances.len() == target_amount {
                    //println!("Aborting early after finding all targets");
                    break;
                }
            }

            // Relax neighbors
            for out_neighbor in graph.out_neighbors(node_index) {
                let new_neighbor_weight =
                    weight.clone() + graph.edge_data(out_neighbor.edge_id).weight();
                let neighbor_weight = self.node_weights.get_mut(out_neighbor.node_id.as_usize());
                if new_neighbor_weight < *neighbor_weight {
                    *neighbor_weight = new_neighbor_weight.clone();
                    self.heap.insert(new_neighbor_weight, out_neighbor.node_id);
                    //self.back_pointers[out_neighbor.node_id.as_usize()] = node_index.into();
                }
            }
        }

        self.heap.clear();
        /*for back_pointer in &mut self.back_pointers {
            *back_pointer = Default::default();
        }*/
        self.node_weights.clear();
    }
}

#[cfg(test)]
mod tests {
    use crate::dijkstra::DefaultDijkstra;
    use traitgraph::implementation::petgraph_impl::PetGraph;
    use traitgraph::interface::MutableGraphContainer;

    #[test]
    fn test_dijkstra_simple() {
        let mut graph = PetGraph::new();
        let n1 = graph.add_node(());
        let n2 = graph.add_node(());
        let n3 = graph.add_node(());
        graph.add_edge(n1, n2, 2);
        graph.add_edge(n2, n3, 2);
        graph.add_edge(n1, n3, 5);

        let mut dijkstra = DefaultDijkstra::new(&graph);
        let mut distances = Vec::new();
        let mut targets = vec![false, false, true];
        dijkstra.shortest_path_lens(&graph, n1, &targets, 1, 6, false, &mut distances);
        debug_assert_eq!(distances, vec![(n3, 4)]);

        dijkstra.shortest_path_lens(&graph, n1, &targets, 1, 6, false, &mut distances);
        debug_assert_eq!(distances, vec![(n3, 4)]);

        dijkstra.shortest_path_lens(&graph, n2, &targets, 1, 6, false, &mut distances);
        debug_assert_eq!(distances, vec![(n3, 2)]);

        dijkstra.shortest_path_lens(&graph, n3, &targets, 1, 6, false, &mut distances);
        debug_assert_eq!(distances, vec![(n3, 0)]);

        targets = vec![false, true, false];
        dijkstra.shortest_path_lens(&graph, n3, &targets, 1, 6, false, &mut distances);
        debug_assert_eq!(distances, vec![]);
    }

    #[test]
    fn test_dijkstra_cycle() {
        let mut graph = PetGraph::new();
        let n1 = graph.add_node(());
        let n2 = graph.add_node(());
        let n3 = graph.add_node(());
        graph.add_edge(n1, n2, 2);
        graph.add_edge(n2, n3, 2);
        graph.add_edge(n3, n1, 5);

        let mut dijkstra = DefaultDijkstra::new(&graph);
        let mut distances = Vec::new();
        let targets = vec![false, false, true];
        dijkstra.shortest_path_lens(&graph, n1, &targets, 1, 6, false, &mut distances);
        debug_assert_eq!(distances, vec![(n3, 4)]);
    }
}
