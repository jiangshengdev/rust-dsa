//! # Merge Sort
//!
//! This is adapted from
//!
//! <https://en.wikipedia.org/wiki/Merge_sort>
//!
//! Test case is derived from
//!
//! <https://github.com/rust-lang/rust/blob/60bd3f96779dbe6bd206dae09395e9af7d580552/library/alloc/src/collections/binary_heap/tests.rs>

/// Sorting the entire array.
pub fn sort<T: Ord + Copy>(a: &mut [T]) -> &mut [T] {
    let size = a.len();

    if size <= 1 {
        return a;
    }

    // One time copy of `a[]` to `b[]`.
    let mut b = a.to_vec();

    merge_sort(a, &mut b, size);
    a
}

/// Array `a[]` has the items to sort; array `b[]` is a work array.
fn merge_sort<T: Ord + Copy>(a: &mut [T], b: &mut [T], n: usize) {
    // Sort data from `b[]` into `a[]`.
    split_merge(b, 0, n, a);
}

/// Split `a[]` into 2 runs, sort both runs into `b[]`, merge both runs from `b[]` to `a[]`.
///
/// [begin, end);
fn split_merge<T: Ord + Copy>(b: &mut [T], begin: usize, end: usize, a: &mut [T]) {
    // If run size == 1, consider it sorted.
    if end - begin <= 1 {
        return;
    }

    // Split the run longer than 1 item into halves.
    // middle = mid point;
    let middle = begin + (end - begin) / 2;

    // Recursively sort both runs from array `a[]` into `b[]`.
    // Sort the left run.
    split_merge(a, begin, middle, b);
    // Sort the right run.
    split_merge(a, middle, end, b);

    // Merge the resulting runs from array `b[]` into `a[]`.
    merge(b, begin, middle, end, a);
}

/// Left source half is `a[begin, middle - 1]`.
///
/// Right source half is `a[middle, end - 1]`.
///
/// Result is `b[begin, end - 1]`.
fn merge<T: Ord + Copy>(a: &mut [T], begin: usize, middle: usize, end: usize, b: &mut [T]) {
    let mut i = begin;
    let mut j = middle;

    // While there are elements in the left or right runs...
    for k in begin..end {
        // If left run head exists and is <= existing right run head.
        if i < middle && (j >= end || a[i] <= a[j]) {
            b[k] = a[i];
            i += 1;
        } else {
            b[k] = a[j];
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::distributions::Uniform;
    use rand::Rng;

    use super::*;

    fn check_orderly<T: Ord>(a: &[T]) {
        if a.is_empty() {
            return;
        }

        for i in 0..a.len() - 1 {
            assert!(a[i] <= a[i + 1]);
        }
    }

    #[test]
    fn test_basic() {
        let mut data = vec![5, 9, 3];
        let sorted = vec![3, 5, 9];
        sort(&mut data);
        check_orderly(&data);
        assert_eq!(data, sorted);
    }

    #[test]
    fn test_normal() {
        let mut data = vec![2, 4, 6, 2, 1, 8, 10, 3, 5, 7, 0, 9, 1];
        let sorted = vec![0, 1, 1, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        sort(&mut data);
        check_orderly(&data);
        assert_eq!(data, sorted);
    }

    #[test]
    fn test_empty() {
        let mut data: Vec<i32> = vec![];
        let sorted: Vec<i32> = vec![];
        sort(&mut data);
        check_orderly(&data);
        assert_eq!(data, sorted);
    }

    #[test]
    fn test_reverse() {
        let mut data = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let sorted = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        sort(&mut data);
        check_orderly(&data);
        assert_eq!(data, sorted);
    }

    #[test]
    fn test_negative() {
        let mut data = vec![-10, 1, 2, 3, 3, -20, 5, 43];
        let sorted = vec![-20, -10, 1, 2, 3, 3, 5, 43];
        sort(&mut data);
        check_orderly(&data);
        assert_eq!(data, sorted);
    }

    #[test]
    fn test_batch() {
        check_orderly(sort::<i32>(&mut []));
        check_orderly(sort(&mut [5]));
        check_orderly(sort(&mut [3, 2]));
        check_orderly(sort(&mut [2, 3]));
        check_orderly(sort(&mut [5, 1, 2]));
        check_orderly(sort(&mut [1, 100, 2, 3]));
        check_orderly(sort(&mut [1, 3, 5, 7, 9, 2, 4, 6, 8, 0]));
        check_orderly(sort(&mut [2, 4, 6, 2, 1, 8, 10, 3, 5, 7, 0, 9, 1]));
        check_orderly(sort(&mut [
            9, 11, 9, 9, 9, 9, 11, 2, 3, 4, 11, 9, 0, 0, 0, 0,
        ]));
        check_orderly(sort(&mut [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
        check_orderly(sort(&mut [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]));
        check_orderly(sort(&mut [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 1, 2]));
        check_orderly(sort(&mut [5, 4, 3, 2, 1, 5, 4, 3, 2, 1, 5, 4, 3, 2, 1]));
    }

    #[test]
    fn test_random_small() {
        let mut rng = rand::thread_rng();
        let range = Uniform::new_inclusive(1, 20);
        let mut data = (&mut rng).sample_iter(range).take(10).collect::<Vec<i32>>();
        sort(&mut data);
        check_orderly(&data);
    }

    #[test]
    fn test_random_large() {
        let mut rng = rand::thread_rng();
        let range = Uniform::new_inclusive(-50000, 50000);
        let mut data = (&mut rng)
            .sample_iter(range)
            .take(50000)
            .collect::<Vec<i32>>();
        sort(&mut data);
        check_orderly(&data);
    }
}
