pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in (0..len).rev() {
        let mut biggest = 0;
        for j in 1..=i {
            if arr[biggest] < arr[j] {
                biggest = j;
            }
        }
        arr.swap(biggest, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_selection_sort() {
        let mut arr = [7, 4, 10, 24, 3, 2, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn simple_selection_sort_descending() {
        let mut arr = [24, 10, 7, 4, 3, 2, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }
}
