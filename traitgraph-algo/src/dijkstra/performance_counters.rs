use std::ops::{Add, AddAssign};

/// Performance data collected by Dijkstra's algorithm.
/// This trait allows to collect the performance data optionally,
/// by providing a type that either collects it, or ignores it.
pub trait DijkstraPerformanceData {
    /// Increment the number of iterations of the main loop of Dijkstra's algorithm.
    fn add_iteration(&mut self);

    /// Increment the number of heap elements that already have a lower weight than what was stored in the heap.
    /// These are wasted cycles because our heap does not support the `decrease_key` operation.
    fn add_unnecessary_heap_element(&mut self);

    /// Get the number of iterations of the main loop of Dijkstra's algorithm.
    fn iterations(&self) -> Option<u64>;

    /// Get the number of unnecessary heap elements that were inserted during Dijkstra's algorithm.
    fn unnecessary_heap_elements(&self) -> Option<u64>;
}

/// A simple performance counter for Dijkstra's algorithm, keeping all supported counts.
#[derive(Default, Debug, Clone)]
pub struct DijkstraPerformanceCounter {
    /// The number of iterations of the main loop of Dijkstra's algorithm.
    pub iterations: u64,
    /// The number of unnecessary heap elements.
    pub unnecessary_heap_elements: u64,
}

/// A performance counter for Dijkstra's algorithm that ignores all counts.
#[derive(Default, Debug, Clone, Copy)]
pub struct NoopDijkstraPerformanceCounter;

impl DijkstraPerformanceData for DijkstraPerformanceCounter {
    fn add_iteration(&mut self) {
        self.iterations += 1;
    }

    fn add_unnecessary_heap_element(&mut self) {
        self.unnecessary_heap_elements += 1;
    }

    fn iterations(&self) -> Option<u64> {
        Some(self.iterations)
    }

    fn unnecessary_heap_elements(&self) -> Option<u64> {
        Some(self.unnecessary_heap_elements)
    }
}

impl DijkstraPerformanceData for NoopDijkstraPerformanceCounter {
    fn add_iteration(&mut self) {}

    fn add_unnecessary_heap_element(&mut self) {}

    fn iterations(&self) -> Option<u64> {
        None
    }

    fn unnecessary_heap_elements(&self) -> Option<u64> {
        None
    }
}

impl Add for DijkstraPerformanceCounter {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            iterations: self.iterations + rhs.iterations,
            unnecessary_heap_elements: self.unnecessary_heap_elements
                + rhs.unnecessary_heap_elements,
        }
    }
}

impl Add for NoopDijkstraPerformanceCounter {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self::Output {
        Self
    }
}

impl AddAssign for DijkstraPerformanceCounter {
    fn add_assign(&mut self, rhs: Self) {
        // I trust that the compiler optimises this correctly
        *self = self.clone() + rhs;
    }
}

impl AddAssign for NoopDijkstraPerformanceCounter {
    fn add_assign(&mut self, _rhs: Self) {
        // do nothing
    }
}
