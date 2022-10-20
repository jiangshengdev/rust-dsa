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
    T: Copy + Ord,
{
    fn new() -> Self {
        BinaryHeap { data: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        let heap = &mut self.data;
        let index = heap.len();
        heap.push(item);
        self.sift_up(index);
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    pub fn pop(&mut self) -> Option<T> {
        let heap = &mut self.data;
        let size = heap.len();

        if size < 2 {
            return heap.pop();
        }

        heap.swap(0, size - 1);
        let item = heap.pop();
        self.sift_down(0);
        item
    }

    fn sift_up(&mut self, i: usize) {
        let mut index = i;
        let heap = &mut self.data;
        let item = *heap.get(index).unwrap();

        while index > 0 {
            let parent_index = (index - 1) / 2;
            let parent = *heap.get(parent_index).unwrap();
            if parent > item {
                heap.swap(index, parent_index);
                index = parent_index;
            } else {
                return;
            }
        }
    }

    fn sift_down(&mut self, i: usize) {
        let heap = &mut self.data;
        let mut index = i;
        let item = *heap.get(index).unwrap();
        let size = heap.len();
        let half_size = size / 2;
        while index < half_size {
            let left_index = index * 2 + 1;
            let right_index = left_index + 1;
            let left = heap.get(left_index);
            let right = heap.get(right_index);

            if *left.unwrap() < item {
                if right_index < size && *right.unwrap() < *left.unwrap() {
                    heap.swap(index, right_index);
                    index = right_index;
                } else {
                    heap.swap(index, left_index);
                    index = left_index;
                }
            } else if right_index < size && *right.unwrap() < item {
                heap.swap(index, right_index);
                index = right_index;
            } else {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut heap = BinaryHeap::new();
        heap.push(1);
        heap.push(2);
        heap.push(3);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        println!("{:?}", heap);
    }
}
