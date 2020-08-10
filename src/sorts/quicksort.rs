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
        if arr[lo] > arr[hi] {
            arr.swap(mi, hi);
        }
        if arr[mi] < arr[hi] {
            arr.swap(lo, mi);
        }
        let pivot = arr[hi];
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

pub fn quicksort_three_way_partition<T: Ord + Copy>(arr: &mut [T]) {
    let partition = |arr: &mut [T]| {
        let mut i = 0;
        let mut j = 0;
        let mut k = arr.len();
        let pivot = {
            let lo = 0;
            let hi = arr.len() - 1;
            let mi = arr.len() / 2;
            if arr[lo] > arr[mi] {
                arr.swap(lo, mi);
            }
            if arr[lo] > arr[hi] {
                arr.swap(mi, hi);
            }
            if arr[mi] < arr[hi] {
                arr.swap(lo, mi);
            }
            arr[hi]
        };
        while j < k {
            if arr[j] < pivot {
                arr.swap(i, j);
                i += 1;
                j += 1;
            } else if arr[j] > pivot {
                k -= 1;
                arr.swap(j, k);
            } else {
                j += 1;
            }
        }
        (i, k)
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

    #[test]
    fn simple_quicksort_three_way() {
        let mut arr = [7, 4, 10, 24, 3, 2, 1];
        quicksort_three_way_partition(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn simple_quicksort_three_way_descending() {
        let mut arr = [24, 10, 7, 4, 3, 2, 1];
        quicksort_three_way_partition(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }
}
