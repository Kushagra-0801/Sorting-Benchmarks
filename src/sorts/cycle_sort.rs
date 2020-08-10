pub fn cycle_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    for cycle_start in 0..len - 1 {
        let mut item = arr[cycle_start];
        let mut pos = cycle_start;
        for i in cycle_start + 1..len {
            if arr[i] < item {
                pos += 1;
            }
        }
        if pos == cycle_start {
            continue;
        }
        while item == arr[pos] {
            pos += 1;
        }
        std::mem::swap(&mut item, &mut arr[pos]);
        while pos != cycle_start {
            pos = cycle_start;
            for i in cycle_start + 1..len {
                if arr[i] < item {
                    pos += 1;
                }
            }
            while item == arr[pos] {
                pos += 1;
            }
            std::mem::swap(&mut item, &mut arr[pos]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_cycle_sort() {
        let mut arr = [7, 4, 10, 24, 3, 2, 1];
        cycle_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn simple_cycle_sort_descending() {
        let mut arr = [24, 10, 7, 4, 3, 2, 1];
        cycle_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }
}
