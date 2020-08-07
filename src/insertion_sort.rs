pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for mut i in 1..len {
        while i > 0 {
            if arr[i] < arr[i - 1] {
                arr.swap(i, i - 1);
                i -= 1;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_selection_sort() {
        let mut arr = [7, 4, 10, 24, 3, 2, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn simple_selection_sort_descending() {
        let mut arr = [24, 10, 7, 4, 3, 2, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }
}
