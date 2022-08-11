use crate::dijkstra::{DijkstraWeight, NodeWeightArray};
use hashbrown::HashMap;

/// Wrapper around `usize` with a `From<&Self>` implementation.
/// This is required to efficiently perform get-or-insert on a [hashbrown::HashMap].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ToOwnedUsize {
    /// The wrapped value.
    pub value: usize,
}

impl<'a> From<&'a ToOwnedUsize> for ToOwnedUsize {
    fn from(v: &'a ToOwnedUsize) -> Self {
        Self { value: v.value }
    }
}

impl<WeightType: DijkstraWeight + Clone> NodeWeightArray<WeightType>
    for HashMap<ToOwnedUsize, WeightType>
{
    fn new(_size: usize) -> Self {
        Default::default()
    }

    fn get(&self, node_index: usize) -> WeightType {
        let key = ToOwnedUsize { value: node_index };
        self.get(&key)
            .cloned()
            .unwrap_or_else(|| WeightType::infinity())
    }

    fn get_mut<'this: 'result, 'result>(
        &'this mut self,
        node_index: usize,
    ) -> &'result mut WeightType {
        let key = ToOwnedUsize { value: node_index };
        self.entry_ref(&key)
            .or_insert_with(|| WeightType::infinity())
    }

    fn set(&mut self, node_index: usize, weight: WeightType) {
        let key = ToOwnedUsize { value: node_index };
        self.insert(key, weight);
    }

    fn clear(&mut self) {
        self.clear()
    }

    fn size(&self) -> usize {
        self.len()
    }
}
