//! # A Priority Queue Implemented With A Binary Heap
//!
//! This is adapted from
//!
//! <https://github.com/facebook/react/blob/65e32e58b6057db1fdfed95a942fad4fc96da191/packages/scheduler/src/SchedulerMinHeap.js>
//!
//! <https://github.com/rust-lang/rust/blob/cb9467515b5a9b15aaa905683c6b4dd9e851056c/library/alloc/src/collections/binary_heap.rs>
//!
//! Test case is derived from
//!
//! <https://github.com/rust-lang/rust/blob/60bd3f96779dbe6bd206dae09395e9af7d580552/library/alloc/src/collections/binary_heap/tests.rs>

use std::vec;

/// A priority queue implemented with a binary heap.
///
/// This will be a min-heap.
#[derive(Debug)]
pub struct BinaryHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> Default for BinaryHeap<T> {
    /// Creates an empty [`BinaryHeap`].
    #[inline]
    fn default() -> Self {
        BinaryHeap::new()
    }
}

impl<T: Ord> BinaryHeap<T> {
    /// Creates an empty [`BinaryHeap`] as a min-heap.
    pub fn new() -> Self {
        BinaryHeap { data: vec![] }
    }

    /// Pushes an item onto the binary heap.
    pub fn push(&mut self, item: T) {
        let items = &mut self.data;
        let index = items.len();
        items.push(item);
        self.sift_up(index);
    }

    /// Returns the least item in the binary heap, or [`None`] if it is empty.
    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    /// Removes the least item from the binary heap and returns it, or [`None`] if it
    /// is empty.
    pub fn pop(&mut self) -> Option<T> {
        let items = &mut self.data;
        let size = items.len();

        if size < 2 {
            return items.pop();
        }

        items.swap(0, size - 1);
        let item = items.pop();
        self.sift_down(0);
        item
    }

    /// Returns the length of the binary heap.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the binary heap is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Take an element at `pos` and move it up the heap,
    /// while its parent is larger.
    fn sift_up(&mut self, pos: usize) {
        let mut index = pos;
        let items = &mut self.data;

        while index > 0 {
            let parent_index = (index - 1) / 2;

            if items[parent_index] > items[index] {
                // The parent is larger. Swap positions.
                items.swap(index, parent_index);
                index = parent_index;
            } else {
                // The parent is smaller. Exit.
                return;
            }
        }
    }

    /// Take an element at `pos` and move it down the heap,
    /// while its children are smaller.
    fn sift_down(&mut self, pos: usize) {
        let mut index = pos;
        let items = &mut self.data;
        let size = items.len();
        let half_size = size / 2;

        while index < half_size {
            let left_index = index * 2 + 1;
            let right_index = left_index + 1;

            // If the left or right node is smaller, swap with the smaller of those.
            if items[left_index] < items[index] {
                if right_index < size && items[right_index] < items[left_index] {
                    items.swap(index, right_index);
                    index = right_index;
                } else {
                    items.swap(index, left_index);
                    index = left_index;
                }
            } else if right_index < size && items[right_index] < items[index] {
                items.swap(index, right_index);
                index = right_index;
            } else {
                // Neither child is smaller. Exit.
                return;
            }
        }
    }

    fn rebuild(&mut self) {
        let mut index = self.len() / 2;

        while index > 0 {
            index -= 1;
            self.sift_down(index);
        }
    }
}

impl<T: Ord> From<Vec<T>> for BinaryHeap<T> {
    /// Converts a [`Vec`] into a [`BinaryHeap`].
    fn from(vec: Vec<T>) -> Self {
        let mut heap = BinaryHeap { data: vec };
        heap.rebuild();
        heap
    }
}

/// An owning iterator over the elements of a [`BinaryHeap`].
#[derive(Clone)]
pub struct IntoIter<T> {
    iter: vec::IntoIter<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> IntoIterator for BinaryHeap<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Creates a consuming iterator, that is, one that moves each value out of
    /// the binary heap in arbitrary order. The binary heap cannot be used
    /// after calling this.
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.data.into_iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_iter_collect() {
        let data = vec![-5, -9, -3];
        let iter_out = vec![-9, -5, -3];
        let pq = BinaryHeap::from(data);

        let v: Vec<_> = pq.into_iter().collect();
        assert_eq!(v, iter_out);
    }

    #[test]
    fn test_into_iter_size_hint() {
        let data = vec![-5, -9];
        let pq = BinaryHeap::from(data);

        let mut it = pq.into_iter();

        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(it.next(), Some(-9));

        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next(), Some(-5));

        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_peek_and_pop() {
        let data = vec![-2, -4, -6, -2, -1, -8, -10, -3, -5, -7, -0, -9, -1];
        let mut sorted = data.clone();
        sorted.sort_by(|a, b| a.cmp(b).reverse());
        let mut heap = BinaryHeap::from(data);
        while !heap.is_empty() {
            assert_eq!(heap.peek().unwrap(), sorted.last().unwrap());
            assert_eq!(heap.pop().unwrap(), sorted.pop().unwrap());
        }
    }

    #[test]
    fn test_push() {
        let mut heap = BinaryHeap::from(vec![-2, -4, -9]);
        assert_eq!(heap.len(), 3);
        assert_eq!(*heap.peek().unwrap(), -9);
        heap.push(-11);
        assert_eq!(heap.len(), 4);
        assert_eq!(*heap.peek().unwrap(), -11);
        heap.push(-5);
        assert_eq!(heap.len(), 5);
        assert_eq!(*heap.peek().unwrap(), -11);
        heap.push(-27);
        assert_eq!(heap.len(), 6);
        assert_eq!(*heap.peek().unwrap(), -27);
        heap.push(-3);
        assert_eq!(heap.len(), 7);
        assert_eq!(*heap.peek().unwrap(), -27);
        heap.push(-103);
        assert_eq!(heap.len(), 8);
        assert_eq!(*heap.peek().unwrap(), -103);
    }

    fn check_batch(data: Vec<i32>) {
        let mut heap = BinaryHeap::from(data.clone());
        let mut sorted = data;
        sorted.sort();

        for item in sorted {
            assert_eq!(item, heap.pop().unwrap())
        }
    }

    #[test]
    fn test_to_vec() {
        check_batch(vec![]);
        check_batch(vec![5]);
        check_batch(vec![3, 2]);
        check_batch(vec![2, 3]);
        check_batch(vec![5, 1, 2]);
        check_batch(vec![1, 100, 2, 3]);
        check_batch(vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 0]);
        check_batch(vec![2, 4, 6, 2, 1, 8, 10, 3, 5, 7, 0, 9, 1]);
        check_batch(vec![9, 11, 9, 9, 9, 9, 11, 2, 3, 4, 11, 9, 0, 0, 0, 0]);
        check_batch(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        check_batch(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
        check_batch(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 1, 2]);
        check_batch(vec![5, 4, 3, 2, 1, 5, 4, 3, 2, 1, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_empty_pop() {
        let mut heap = BinaryHeap::<i32>::new();
        assert!(heap.pop().is_none());
    }

    #[test]
    fn test_empty_peek() {
        let empty = BinaryHeap::<i32>::new();
        assert!(empty.peek().is_none());
    }
}
