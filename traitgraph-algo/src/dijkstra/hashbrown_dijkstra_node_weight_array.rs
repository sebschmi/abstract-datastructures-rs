use crate::dijkstra::NodeWeightArray;
use hashbrown::HashMap;

impl<WeightType: Clone> NodeWeightArray<WeightType> for HashMap<usize, WeightType> {
    fn new(_size: usize) -> Self {
        Default::default()
    }

    fn get(&self, node_index: usize) -> WeightType {
        self.get(&node_index).unwrap().clone()
    }

    fn get_mut(&mut self, node_index: usize) -> &mut WeightType {
        self.get_mut(&node_index).unwrap()
    }

    fn set(&mut self, node_index: usize, weight: WeightType) {
        self.insert(node_index, weight);
    }

    fn clear(&mut self) {
        self.clear()
    }
}
