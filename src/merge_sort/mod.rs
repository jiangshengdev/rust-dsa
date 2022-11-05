//! # Merge sort
//!
//! This is adapted from
//!
//! <https://en.wikipedia.org/wiki/Merge_sort>
//!
//! Test case is derived from
//!
//! <https://github.com/rust-lang/rust/blob/60bd3f96779dbe6bd206dae09395e9af7d580552/library/alloc/src/collections/binary_heap/tests.rs>

/// Sorting the entire array
fn sort<T: Ord + Copy>(a: &mut [T]) -> &mut [T] {
    let size = a.len();

    if size <= 1 {
        return a;
    }

    let mut b = vec![];
    b.resize(size, a[0]);
    merge_sort(a, &mut b, size);
    a
}

/// Array A[] has the items to sort; array B[] is a work array.
fn merge_sort<T: Ord + Copy>(a: &mut [T], b: &mut [T], n: usize) {
    // one time copy of A[] to B[]
    b.copy_from_slice(a);

    // sort data from B[] into A[]
    split_merge(b, 0, n, a);
}

/// Split A[] into 2 runs, sort both runs into B[], merge both runs from B[] to A[]
/// iBegin is inclusive; iEnd is exclusive (A[iEnd] is not in the set).
fn split_merge<T: Ord + Copy>(b: &mut [T], i_begin: usize, i_end: usize, a: &mut [T]) {
    // if run size == 1
    if i_end - i_begin <= 1 {
        // consider it sorted
        return;
    }

    // split the run longer than 1 item into halves
    // iMiddle = mid point
    let i_middle = (i_end + i_begin) / 2;

    // recursively sort both runs from array A[] into B[]
    // sort the left  run
    split_merge(a, i_begin, i_middle, b);
    // sort the right run
    split_merge(a, i_middle, i_end, b);

    // merge the resulting runs from array B[] into A[]
    merge(b, i_begin, i_middle, i_end, a);
}

///  Left source half is A[ iBegin:iMiddle-1].
/// Right source half is A[iMiddle:iEnd-1   ].
/// Result is            B[ iBegin:iEnd-1   ].
fn merge<T: Ord + Copy>(a: &mut [T], i_begin: usize, i_middle: usize, i_end: usize, b: &mut [T]) {
    let mut i = i_begin;
    let mut j = i_middle;

    // While there are elements in the left or right runs...
    for k in i_begin..i_end {
        // If left run head exists and is <= existing right run head.
        if i < i_middle && (j >= i_end || a[i] <= a[j]) {
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
