//! Bubble sort is a very simple sorting algorithm that works
//! by repeatedly sorting adjacent elements if they are in the
//! wrong order.
//!
//! Bubble sort is very good for taking large elements from
//! start to the end of the array. But it is very inefficient
//! if small elements are at the end.
//!
//! Naive bubble sort implementations always traverse the whole
//! array in every outer loop iteration. This is very wasteful
//! because after the i-th iteration of the outer loop, the last i
//! elements are definitely in order and do not need to be compared.
//!
//! Classical bubble sort implementations reduce the inner loop
//! iterations by one after every outer loop iteration.
//! The best and average case can be made even faster by stopping
//! the algorithm if the array has already been sorted. This adds
//! some overhead in the worst case though.
//!
//! This can be done be checking if any swaps occured in the inner
//! loop and breaking the loop if no swaps occured.

/// Naive Bubble Sort
/// Does (n ^ 2) iterations of the array
pub fn naive_bubble_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..(len - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

/// Classical Bubble Sort
/// Does (n * (n - 1)) / 2 iterations of the array
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..(len - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

/// Optimized Bubble Sort
/// Same as classical version but breaks the outer loop
/// as soon as the array has been sorted
/// Best Case -> 1 iteration of the array
/// Worst Case -> (n * (n - 1)) / 2 iterations of the array
pub fn bubble_sort_optimized<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        let mut swapped = false;
        for j in 0..(len - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_naive_bubble_sort() {
        let mut arr = [7, 4, 10, 24, 3, 2, 1];
        naive_bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn simple_naive_bubble_sort_descending() {
        let mut arr = [24, 10, 7, 4, 3, 2, 1];
        naive_bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn simple_bubble_sort() {
        let mut arr = [7, 4, 10, 24, 3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn simple_bubble_sort_descending() {
        let mut arr = [24, 10, 7, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn simple_bubble_sort_optimized() {
        let mut arr = [7, 4, 10, 24, 3, 2, 1];
        bubble_sort_optimized(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn simple_bubble_sort_optimized_descending() {
        let mut arr = [24, 10, 7, 4, 3, 2, 1];
        bubble_sort_optimized(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }
}
