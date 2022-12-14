//! # 3-Way Quick Sort (Dutch National Flag)
//!
//! This is adapted from
//!
//! <https://en.wikipedia.org/wiki/Quicksort>
//!
//! <https://en.wikipedia.org/wiki/Dutch_national_flag_problem>
//!
//! Test case is derived from
//!
//! <https://github.com/rust-lang/rust/blob/60bd3f96779dbe6bd206dae09395e9af7d580552/library/alloc/src/collections/binary_heap/tests.rs>

use std::cmp::Ordering;

/// Sorting the entire array.
pub fn sort<T: Ord>(a: &mut [T]) -> &mut [T] {
    let size = a.len();

    if size <= 1 {
        return a;
    }

    quick_sort(a, 0, size - 1);
    a
}

/// Sorts a (portion of an) array, divides it into partitions, then sorts those.
fn quick_sort<T: Ord>(a: &mut [T], lo: usize, hi: usize) {
    // Ensure indices are in correct order.
    if lo >= hi {
        return;
    }

    // Choice of pivot.
    pivot(a, lo, hi);

    // Partition array and get the pivot indices.
    let (left, right) = partition(a, lo, hi);

    // Sort the two partitions.
    if left > 1 {
        // Left side of pivot.
        quick_sort(a, lo, left - 1);
    }
    // Right side of pivot.
    quick_sort(a, right + 1, hi);
}

/// Median-of-three.
fn pivot<T: Ord>(a: &mut [T], lo: usize, hi: usize) {
    let mid = lo + (hi - lo) / 2;

    if a[mid] < a[lo] {
        a.swap(lo, mid);
    }

    if a[hi] < a[lo] {
        a.swap(lo, hi);
    }

    if a[mid] < a[hi] {
        a.swap(mid, hi);
    }
}

/// Divides array into three partitions.
fn partition<T: Ord>(a: &mut [T], lo: usize, hi: usize) -> (usize, usize) {
    // Choose the last element as the pivot.
    let p = hi;

    // Temporary pivot indices.
    let mut i = lo;
    let mut j = lo;

    let mut k = hi - 1;

    // mid = a[p];
    // [lo, i) < mid;
    // [i, j) == mid;
    // [j, k] not yet sorted;
    // [k + 1, hi - 1] > mid;
    while j <= k {
        match a[j].cmp(&a[p]) {
            Ordering::Less => {
                a.swap(i, j);
                i += 1;
                j += 1;
            }
            Ordering::Greater => {
                a.swap(j, k);

                if k == 0 {
                    break;
                }

                k -= 1;
            }
            Ordering::Equal => {
                j += 1;
            }
        }
    }

    // Move the pivot element to the correct pivot position
    // (between the smaller and larger elements).
    a.swap(j, p);

    // The pivot indices.
    (i, j)
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
