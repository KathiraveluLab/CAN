use fastbloom::BloomFilter;
use std::hash::Hash;

/// A wrapper around Fastbloom to provide content indexing.
/// The paper mentions that Bloom Filters help in scaling the Content Routing Table (CRT).
pub struct BloomFilterWrapper {
    filter: BloomFilter,
}

impl BloomFilterWrapper {
    /// Creates a new Bloom Filter with a specified capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            filter: BloomFilter::with_num_bits(capacity * 10).expected_items(capacity), // 10 bits per item for ~1% false positive rate
        }
    }

    /// Inserts a content ID or name into the filter.
    pub fn insert<T: Hash>(&mut self, item: T) {
        self.filter.insert(&item);
    }

    /// Checks if a content ID or name is likely in the filter.
    pub fn contains<T: Hash>(&self, item: T) -> bool {
        self.filter.contains(&item)
    }
}

impl Default for BloomFilterWrapper {
    fn default() -> Self {
        Self::new(1000)
    }
}
