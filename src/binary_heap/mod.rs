//! This is adapted from
//! https://github.com/facebook/react/blob/65e32e58b6057db1fdfed95a942fad4fc96da191/packages/scheduler/src/SchedulerMinHeap.js
//!
//! https://github.com/rust-lang/rust/blob/cb9467515b5a9b15aaa905683c6b4dd9e851056c/library/alloc/src/collections/binary_heap.rs
//!
//! Test case is derived from
//! https://github.com/rust-lang/rust/blob/60bd3f96779dbe6bd206dae09395e9af7d580552/library/alloc/src/collections/binary_heap/tests.rs

/// A priority queue implemented with a binary heap.
///
/// This will be a min-heap.
#[derive(Debug)]
pub struct BinaryHeap<T> {
    data: Vec<T>,
}

impl<T> BinaryHeap<T>
where
    T: Ord + Copy,
{
    fn new() -> Self {
        BinaryHeap { data: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        let heap = &mut self.data;
        let index = heap.len();
        heap.push(item);
        self.siftUp(index);
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    pub fn pop(&mut self) -> Option<T> {
        let heap = &mut self.data;
        let size = heap.len();

        if size == 0 {
            return None;
        }

        let first = heap.first().unwrap();
        let last = heap.last().unwrap();

        if *last != *first {
            heap.swap(0, size - 1);
            let ret = heap.pop();
            self.siftDown(0);
            ret
        } else {
            heap.pop()
        }
    }

    fn siftUp(&mut self, i: usize) {}

    fn siftDown(&mut self, i: usize) {}
}

#[cfg(test)]
mod tests {
    use crate::binary_heap::BinaryHeap;

    #[test]
    fn test_basic() {
        let mut heap = BinaryHeap::new();
        heap.push(1);
        heap.push(2);
        heap.push(3);
        assert_eq!(heap.pop(), Some(1));
        println!("{:?}", heap);
    }
}
