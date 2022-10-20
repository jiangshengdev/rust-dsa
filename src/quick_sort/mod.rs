//! This is adapted from
//! https://en.wikipedia.org/wiki/Quicksort
//!
//! Test case is derived from
//! https://github.com/rust-lang/rust/blob/60bd3f96779dbe6bd206dae09395e9af7d580552/library/alloc/src/collections/binary_heap/tests.rs

pub fn sort<T>(a: &mut [T]) -> &mut [T]
where
    T: Copy + Ord,
{
    quick_sort(a, 0, a.len() as isize - 1);
    a
}

/// Sorts a (portion of an) array,
/// divides it into partitions, then sorts those
pub fn quick_sort<T>(a: &mut [T], lo: isize, hi: isize)
where
    T: Copy + Ord,
{
    // Ensure indices are in correct order
    if lo >= hi || lo < 0 {
        return;
    }

    // Partition array and get the pivot index
    let p = partition(a, lo, hi);

    // Sort the two partitions
    quick_sort(a, lo, p - 1); // Left side of pivot
    quick_sort(a, p + 1, hi); // Right side of pivot
}

/// Divides array into two partitions
fn partition<T>(a: &mut [T], lo: isize, hi: isize) -> isize
where
    T: Copy + Ord,
{
    let pivot = a[hi as usize]; // Choose the last element as the pivot

    // Temporary pivot index
    let mut i = lo - 1;

    for j in lo..=hi - 1 {
        // If the current element is less than or equal to the pivot
        if a[j as usize] <= pivot {
            // Move the temporary pivot index forward
            i += 1;
            // Swap the current element with the element at the temporary pivot index
            a.swap(i as usize, j as usize);
        }
    }

    // Move the pivot element to the correct pivot position
    // (between the smaller and larger elements)
    i += 1;
    a.swap(i as usize, hi as usize);
    i // the pivot index
}

#[cfg(test)]
mod tests {
    use rand::distributions::Uniform;
    use rand::Rng;

    use super::*;

    fn check_orderly<T>(a: &[T])
    where
        T: Ord,
    {
        if a.is_empty() {
            return;
        }

        for i in 0..a.len() - 1 {
            assert!(a[i] <= a[i + 1]);
        }
    }

    #[test]
    fn test_basic() {
        let mut data = [5, 9, 3];
        let sorted = [3, 5, 9];
        sort(&mut data);
        check_orderly(&data);
        assert_eq!(data, sorted);
    }

    #[test]
    fn test_normal() {
        let mut data = [2, 4, 6, 2, 1, 8, 10, 3, 5, 7, 0, 9, 1];
        let sorted = [0, 1, 1, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        sort(&mut data);
        check_orderly(&data);
        assert_eq!(data, sorted);
    }

    #[test]
    fn test_empty() {
        let mut data: [i32; 0] = [];
        let sorted: [i32; 0] = [];
        sort(&mut data);
        check_orderly(&data);
        assert_eq!(data, sorted);
    }

    #[test]
    fn test_reverse() {
        let mut data = [9, 8, 7, 6, 5, 4, 3, 2, 1];
        let sorted = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        sort(&mut data);
        check_orderly(&data);
        assert_eq!(data, sorted);
    }

    #[test]
    fn test_negative() {
        let mut data = [-10, 1, 2, 3, 3, -20, 5, 43];
        let sorted = [-20, -10, 1, 2, 3, 3, 5, 43];
        sort(&mut data);
        check_orderly(&data);
        assert_eq!(data, sorted);
    }

    #[test]
    fn test_batch() {
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
