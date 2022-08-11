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

    /// Record the current heap size of Dijkstra's algorithm.
    fn record_heap_size(&mut self, heap_size: usize);

    /// Record the current distance array size of Dijkstra's algorithm.
    fn record_distance_array_size(&mut self, distance_array_size: usize);

    /// Finish an invocation of Dijkstra's algorithm.
    /// Performs finalisation of recorded metrics that are local to single Dijkstra invocations.
    fn finish_dijkstra(&mut self);

    /// Get the number of iterations of the main loop of Dijkstra's algorithm.
    fn iterations(&self) -> Option<u64>;

    /// Get the number of unnecessary heap elements that were inserted during Dijkstra's algorithm.
    fn unnecessary_heap_elements(&self) -> Option<u64>;

    /// Get the maximum heap size encountered at any point during execution.
    fn max_max_heap_size(&self) -> Option<usize>;

    /// Get the maximum distance array size encountered at any point during execution.
    fn max_max_distance_array_size(&self) -> Option<usize>;

    /// Get the maximum heap size as average over all invocations of Dijkstra's algorithm.
    fn average_max_heap_size(&self) -> Option<f64>;

    /// Get the maximum distance array size as average over all invocations of Dijkstra's algorithm.
    fn average_max_distance_array_size(&self) -> Option<f64>;
}

/// A simple performance counter for Dijkstra's algorithm, keeping all supported counts.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct DijkstraPerformanceCounter {
    /// The number of iterations of the main loop of Dijkstra's algorithm.
    pub iterations: u64,
    /// The number of unnecessary heap elements.
    pub unnecessary_heap_elements: u64,
    max_heap_size: usize,
    max_distance_array_size: usize,
    max_max_heap_size: usize,
    max_max_distance_array_size: usize,
    sum_max_heap_size: u128,
    sum_max_distance_array_size: u128,
    total_invocations: u64,
}

/// A performance counter for Dijkstra's algorithm that ignores all counts.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct NoopDijkstraPerformanceCounter;

impl DijkstraPerformanceData for DijkstraPerformanceCounter {
    fn add_iteration(&mut self) {
        self.iterations += 1;
    }

    fn add_unnecessary_heap_element(&mut self) {
        self.unnecessary_heap_elements += 1;
    }

    fn record_heap_size(&mut self, heap_size: usize) {
        self.max_heap_size = self.max_heap_size.max(heap_size);
    }

    fn record_distance_array_size(&mut self, distance_array_size: usize) {
        self.max_distance_array_size = self.max_distance_array_size.max(distance_array_size);
    }

    fn finish_dijkstra(&mut self) {
        self.max_max_heap_size = self.max_max_heap_size.max(self.max_heap_size);
        self.max_max_distance_array_size = self
            .max_max_distance_array_size
            .max(self.max_distance_array_size);
        self.sum_max_heap_size += self.max_heap_size as u128;
        self.sum_max_distance_array_size += self.max_distance_array_size as u128;

        self.max_heap_size = 0;
        self.max_distance_array_size = 0;
        self.total_invocations += 1;
    }

    fn iterations(&self) -> Option<u64> {
        Some(self.iterations)
    }

    fn unnecessary_heap_elements(&self) -> Option<u64> {
        Some(self.unnecessary_heap_elements)
    }

    fn max_max_heap_size(&self) -> Option<usize> {
        Some(self.max_max_heap_size)
    }

    fn max_max_distance_array_size(&self) -> Option<usize> {
        Some(self.max_max_distance_array_size)
    }

    fn average_max_heap_size(&self) -> Option<f64> {
        Some(self.sum_max_heap_size as f64 / self.total_invocations as f64)
    }

    fn average_max_distance_array_size(&self) -> Option<f64> {
        Some(self.sum_max_distance_array_size as f64 / self.total_invocations as f64)
    }
}

impl DijkstraPerformanceData for NoopDijkstraPerformanceCounter {
    fn add_iteration(&mut self) {}

    fn add_unnecessary_heap_element(&mut self) {}

    fn record_heap_size(&mut self, _heap_size: usize) {}

    fn record_distance_array_size(&mut self, _distance_array_size: usize) {}

    fn finish_dijkstra(&mut self) {}

    fn iterations(&self) -> Option<u64> {
        None
    }

    fn unnecessary_heap_elements(&self) -> Option<u64> {
        None
    }

    fn max_max_heap_size(&self) -> Option<usize> {
        None
    }

    fn max_max_distance_array_size(&self) -> Option<usize> {
        None
    }

    fn average_max_heap_size(&self) -> Option<f64> {
        None
    }

    fn average_max_distance_array_size(&self) -> Option<f64> {
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
            max_heap_size: self.max_heap_size.max(rhs.max_heap_size),
            max_distance_array_size: self
                .max_distance_array_size
                .max(rhs.max_distance_array_size),
            max_max_heap_size: self.max_max_heap_size.max(rhs.max_max_heap_size),
            max_max_distance_array_size: self
                .max_max_distance_array_size
                .max(rhs.max_max_distance_array_size),
            sum_max_heap_size: self.sum_max_heap_size + rhs.sum_max_heap_size,
            sum_max_distance_array_size: self.sum_max_distance_array_size
                + rhs.sum_max_distance_array_size,
            total_invocations: self.total_invocations + rhs.total_invocations,
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
