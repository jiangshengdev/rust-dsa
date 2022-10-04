//! This is adapted from https://en.wikipedia.org/wiki/Quicksort

pub fn sort<T>(a: &mut [T])
where
    T: Copy + Ord,
{
    quick_sort(a, 0, a.len() as isize - 1);
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
    i
}

#[cfg(test)]
mod tests {
    use rand::distributions::Uniform;
    use rand::Rng;

    use super::*;

    fn is_orderly<T>(a: &[T])
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
    fn it_works() {
        let mut a = [1, 3, 5, 7, 9, 2, 4, 6, 8, 10];

        println!("{:?}", a);
        sort(&mut a);
        println!("{:?}", a);

        is_orderly(&a);
    }

    #[test]
    fn test_empty() {
        let mut a: [i32; 0] = [];

        println!("{:?}", a);
        sort(&mut a);
        println!("{:?}", a);

        is_orderly(&a);
    }

    #[test]
    fn test_random() {
        let mut rng = rand::thread_rng();
        let range = Uniform::new_inclusive(1, 20);
        let mut v = (&mut rng).sample_iter(range).take(10).collect::<Vec<i32>>();

        println!("{:?}", v);
        sort(&mut v);
        println!("{:?}", v);

        is_orderly(&v);
    }
}
