pub fn heapsort<T: Ord + Copy>(arr: &mut [T]) {
    make_max_heap(arr);
    let len = arr.len();
    for i in (0..len).rev() {
        arr.swap(0, i);
        trickle_down(&mut arr[..i]);
    }
}

fn make_max_heap<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    make_max_heap(&mut arr[1..]);
    trickle_down(arr);
}

fn trickle_down<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let left = |idx: usize| 2 * idx + 1;
    let right = |idx: usize| 2 * idx + 2;
    let mut current = 0;
    loop {
        let right = right(current);
        let left = left(current);
        if left >= len {
            break;
        }
        if right >= len {
            if arr[left] > arr[current] {
                arr.swap(left, current);
                current = left;
            } else {
                break;
            }
        } else {
            if arr[left] > arr[right] && arr[left] > arr[current] {
                arr.swap(left, current);
                current = left;
            } else if arr[right] > arr[left] && arr[right] > arr[current] {
                arr.swap(right, current);
                current = right;
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
    fn simple_heapsort() {
        let mut arr = [7, 4, 10, 24, 3, 2, 1];
        heapsort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn simple_heapsort_descending() {
        let mut arr = [24, 10, 7, 4, 3, 2, 1];
        heapsort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }
}
