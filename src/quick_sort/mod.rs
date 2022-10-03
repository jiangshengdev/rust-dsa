//! This is adapted from https://en.wikipedia.org/wiki/Quicksort

/// Sorts a (portion of an) array,
/// divides it into partitions, then sorts those
pub fn quick_sort<T>(a: &mut [T], lo: usize, hi: usize)
where
    T: Copy + Ord,
{
    // Ensure indices are in correct order
    if lo >= hi {
        return;
    }

    // Partition array and get the pivot index
    let p = partition(a, lo, hi);

    // Sort the two partitions
    quick_sort(a, lo, p - 1); // Left side of pivot
    quick_sort(a, p + 1, hi); // Right side of pivot
}

/// Divides array into two partitions
fn partition<T>(a: &mut [T], lo: usize, hi: usize) -> usize
where
    T: Copy + Ord,
{
    let pivot = a[hi]; // Choose the last element as the pivot

    // Temporary pivot index
    let mut i: isize = lo as isize - 1;

    for j in lo..=hi - 1 {
        // If the current element is less than or equal to the pivot
        if a[j] <= pivot {
            // Move the temporary pivot index forward
            i += 1;
            // Swap the current element with the element at the temporary pivot index
            a.swap(i as usize, j);
        }
    }

    // Move the pivot element to the correct pivot position
    // (between the smaller and larger elements)
    i += 1;
    a.swap(i as usize, hi);
    i as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = [1, 3, 5, 7, 9, 2, 4, 6, 8, 10];
        let len = a.len();
        println!("{:?}", a);

        quick_sort(&mut a, 0, len - 1);

        println!("{:?}", a);
    }
}
