//! Cocktail sort is very similar to bubble sort.
//! But while bubble sort always traverses in the
//! same direction, cocktail sort traverses the array
//! in both directions for every outer loop iteration.
//!
//! Due to its bi-directional nature, both large and small
//! elements can reach their final position very quickly,
//! as opposed to bubble sort, which takes the complete
//! (n^2) iterations to move a small element from end of
//! the list to its start.
//!
//! Naive implementations complete the full (2 * n^2)
//! iterations.
//! Classical implementations can shorten the inner loop
//! for every outer loop iteration.
//! Optimized implementations improve for the best and average
//! case by breaking the outer loop if no swaps were made.

/// Naive implementation
/// Does (2 * n^2) iterations of the array
pub fn naive_cocktail_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 1..len {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
            }
        }
        for j in (1..len).rev() {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
            }
        }
    }
}

/// Classical implementation
/// Does (n * (n - 1)) iterations of the array
pub fn cocktail_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in (i + 1)..(len - i) {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
            }
        }
        for j in ((i + 1)..(len - i - 1)).rev() {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
            }
        }
    }
}

/// Optimized implementation
/// Same as classical version but breaks the outer loop
/// as soon as the array has been sorted
/// Best Case -> 1 iteration of the array
/// Worst Case -> (n * (n - 1)) iterations of the array
pub fn cocktail_sort_optimized<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        let mut swapped = false;
        for j in (i + 1)..(len - i) {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
        for j in ((i + 1)..(len - i - 1)).rev() {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
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
    fn simple_naive_cocktail_sort() {
        let mut arr = [7, 4, 10, 24, 3, 2, 1];
        naive_cocktail_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn simple_naive_cocktail_sort_descending() {
        let mut arr = [24, 10, 7, 4, 3, 2, 1];
        naive_cocktail_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn simple_cocktail_sort() {
        let mut arr = [7, 4, 10, 24, 3, 2, 1];
        cocktail_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn simple_cocktail_sort_descending() {
        let mut arr = [24, 10, 7, 4, 3, 2, 1];
        cocktail_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn simple_cocktail_sort_optimized() {
        let mut arr = [7, 4, 10, 24, 3, 2, 1];
        cocktail_sort_optimized(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn simple_cocktail_sort_optimized_descending() {
        let mut arr = [24, 10, 7, 4, 3, 2, 1];
        cocktail_sort_optimized(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }
}
