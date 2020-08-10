fn quicksort<T: Ord + Copy>(arr: &mut [T], partition: &dyn Fn(&mut [T]) -> (usize, usize)) {
    if arr.len() < 2 {
        return;
    }
    let (left, right) = partition(arr);
    quicksort(&mut arr[..left], partition);
    quicksort(&mut arr[right..], partition);
}

pub fn basic_quicksort_right_most_pivot<T: Ord + Copy>(arr: &mut [T]) {
    let partition = |arr: &mut [T]| {
        let pivot = arr[arr.len() - 1];
        let lo = 0;
        let hi = arr.len() - 1;
        let mut i = 0;
        for j in lo..=hi {
            if arr[j] < pivot {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, hi);
        (i, i + 1)
    };
    quicksort(arr, &partition);
}

pub fn basic_quicksort_median_pivot<T: Ord + Copy>(arr: &mut [T]) {
    let partition = |arr: &mut [T]| {
        let lo = 0;
        let hi = arr.len() - 1;
        let mi = arr.len() / 2;
        if arr[lo] > arr[mi] {
            arr.swap(lo, mi);
        }
        if arr[mi] > arr[hi] {
            arr.swap(mi, hi);
        }
        if arr[lo] > arr[mi] {
            arr.swap(lo, mi);
        }
        let pivot = arr[mi];
        let mut i = 0;
        for j in lo..=hi {
            if arr[j] < pivot {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, hi);
        (i, i + 1)
    };
    quicksort(arr, &partition);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_quicksort_right_pivot() {
        let mut arr = [7, 4, 10, 24, 3, 2, 1];
        basic_quicksort_right_most_pivot(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn simple_quicksort_right_pivot_descending() {
        let mut arr = [24, 10, 7, 4, 3, 2, 1];
        basic_quicksort_right_most_pivot(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn simple_quicksort_median_pivot() {
        let mut arr = [7, 4, 10, 24, 3, 2, 1];
        basic_quicksort_right_most_pivot(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn simple_quicksort_median_pivot_descending() {
        let mut arr = [24, 10, 7, 4, 3, 2, 1];
        basic_quicksort_right_most_pivot(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }
}
