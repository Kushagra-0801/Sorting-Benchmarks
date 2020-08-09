use std::fmt::Debug;

pub fn heapsort<T: Ord + Copy>(arr: &mut [T]) {
    make_max_heap(arr);
    let len = arr.len();
    for i in (0..len).rev() {
        arr.swap(0, i);
        trickle_down(&mut arr[..i]);
    }
}

fn make_max_heap<T: Ord + Copy>(arr: &mut [T]) {
    fn trickle_up<T: Ord + Copy>(arr: &mut [T]) {
        let parent = |idx: usize| (idx - 1) / 2;
        let mut current = arr.len() - 1;
        while current > 0 {
            let p = parent(current);
            if arr[p] < arr[current] {
                arr.swap(p, current);
                current = p;
            } else {
                break;
            }
        }
    }
    if arr.len() <= 1 {
        return;
    }
    for i in 1..=arr.len() {
        trickle_up(&mut arr[..i]);
    }
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
        let left = left(current);
        let right = right(current);
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
            if arr[left] >= arr[right] && arr[left] > arr[current] {
                arr.swap(left, current);
                current = left;
            } else if arr[right] >= arr[left] && arr[right] > arr[current] {
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
