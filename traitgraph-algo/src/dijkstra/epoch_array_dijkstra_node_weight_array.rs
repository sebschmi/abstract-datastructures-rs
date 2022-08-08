use crate::dijkstra::{DijkstraWeight, NodeWeightArray};

/// An epoch counter array.
/// This can be used to check if an index is current by comparing its entry in the epoch array to the current epoch.
/// To unmark all values, the current epoch can be increased in O(1). Only overflows have to be handled by resetting all epoch counters.
pub struct EpochArray {
    epochs: Vec<u32>,
    current_epoch: u32,
}

impl EpochArray {
    /// Create a new epoch array of given length where all values are outdated.
    pub fn new(len: usize) -> Self {
        Self {
            epochs: vec![0; len],
            current_epoch: 1,
        }
    }

    /// Outdate all indices.
    pub fn clear(&mut self) {
        if self.current_epoch == u32::MAX {
            for epoch in self.epochs.iter_mut() {
                *epoch = 0;
            }
            self.current_epoch = 1;
        } else {
            self.current_epoch += 1;
        }
    }

    /// Set the given index as current.
    ///
    /// Safety: Undefined behaviour if the index is out of bounds of the epoch array.
    #[inline]
    pub fn update(&mut self, index: usize) {
        unsafe {
            *self.epochs.get_unchecked_mut(index) = self.current_epoch;
        }
        //self.epochs[index] = self.current_epoch;
    }

    /// Returns true if the given index is current, and false otherwise.
    #[inline]
    pub fn get(&self, index: usize) -> bool {
        self.epochs[index] == self.current_epoch
    }

    /// Updates the given index and returns true if the given index was current before, and false otherwise.
    #[inline]
    pub fn get_and_update(&mut self, index: usize) -> bool {
        if self.epochs[index] == self.current_epoch {
            true
        } else {
            self.epochs[index] = self.current_epoch;
            false
        }
    }
}

/// An epoched node weight array that can be cleared in O(1) most of the times.
/// Only if the epoch in the epoch array overflows, clearing takes linear time.
pub struct EpochNodeWeightArray<WeightType> {
    weights: Vec<WeightType>,
    epochs: EpochArray,
}

impl<WeightType: DijkstraWeight> EpochNodeWeightArray<WeightType> {
    #[inline]
    fn make_current(&mut self, node_index: usize) {
        if !self.epochs.get_and_update(node_index) {
            self.weights[node_index] = WeightType::infinity();
        }
    }
}

impl<WeightType: DijkstraWeight + Copy> NodeWeightArray<WeightType>
    for EpochNodeWeightArray<WeightType>
{
    fn new(len: usize) -> Self {
        Self {
            weights: vec![WeightType::infinity(); len],
            epochs: EpochArray::new(len),
        }
    }

    #[inline]
    fn get(&self, node_index: usize) -> WeightType {
        if self.epochs.get(node_index) {
            self.weights[node_index]
        } else {
            WeightType::infinity()
        }
    }

    #[inline]
    fn get_mut<'this: 'result, 'result>(
        &'this mut self,
        node_index: usize,
    ) -> &'result mut WeightType {
        self.make_current(node_index);
        &mut self.weights[node_index]
    }

    #[inline]
    fn set(&mut self, node_index: usize, weight: WeightType) {
        self.weights[node_index] = weight;
        self.epochs.update(node_index);
    }

    fn clear(&mut self) {
        self.epochs.clear();
    }

    fn size(&self) -> usize {
        self.weights.size()
    }
}
