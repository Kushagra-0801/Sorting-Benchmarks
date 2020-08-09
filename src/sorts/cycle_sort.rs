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

pub fn hash_based_cycle_sort<T: Ord + Copy>(
    arr: &mut [T],
    buf: &mut [u32],
    f: impl Fn(&T) -> usize,
) {
    todo!("search from last possible position to current position for empty block");
    for item in arr.iter() {
        buf[f(item)] += 1;
    }
    for i in 1..buf.len() {
        buf[i] += buf[i - 1];
    }
    let len = arr.len();
    for cycle_start in 0..len - 1 {
        dbg!(cycle_start);
        let mut item = arr[cycle_start];
        let mut pos = buf[f(&item) - 1] as usize;
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

    #[test]
    fn simple_hash_cycle_sort() {
        let mut arr = [1, 1, 1, 3, 3, 4, 5, 2, 2, 2, 2, 1, 5, 4, 5, 3, 4];
        let mut buf = [0; 6];
        hash_based_cycle_sort(&mut arr, &mut buf, |&i| i as usize);
        // assert_eq!(arr, [1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5]);
    }

    #[test]
    fn simple_hash_cycle_sort_descending() {
        let mut arr = [5, 5, 5, 4, 4, 4, 3, 3, 3, 2, 2, 2, 2, 1, 1, 1, 1];
        let mut buf = [0; 6];
        hash_based_cycle_sort(&mut arr, &mut buf, |&i| i as usize);
        assert_eq!(arr, [1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5]);
    }
}
