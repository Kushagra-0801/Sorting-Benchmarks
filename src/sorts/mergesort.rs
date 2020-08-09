pub fn mergesort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    mergesort(&mut arr[..mid]);
    mergesort(&mut arr[mid..]);
    merge(arr);
}

fn merge<T: Ord + Copy>(arr: &mut [T]) {
    let mid = arr.len() / 2;
    if arr[mid - 1] <= arr[mid] {
        return;
    }
    if arr[arr.len() - 1] <= arr[0] {
        if arr.len() % 2 == 0 {
            even_descending_merge(arr);
        } else {
            odd_descending_merge(arr);
        }
    } else {
        general_merge(arr);
    }
}

fn general_merge<T: Ord + Copy>(arr: &mut [T]) {
    let mut gap = (arr.len() + 1) / 2;
    while gap > 0 {
        for i in 0..arr.len() - gap {
            if arr[i] > arr[i + gap] {
                arr.swap(i, i + gap);
            }
        }
        gap = if gap <= 1 { 0 } else { (gap + 1) / 2 };
    }
}

fn even_descending_merge<T: Ord + Copy>(arr: &mut [T]) {
    let mid = arr.len() / 2;
    for i in 0..mid {
        arr.swap(i, i + mid);
    }
}

fn odd_descending_merge<T: Ord + Copy>(arr: &mut [T]) {
    let mid = arr.len() / 2;
    let next_pos = |pos: usize| if pos < mid { pos + mid + 1 } else { pos - mid };
    let next = next_pos(mid);
    let mut tmp = arr[mid];
    std::mem::swap(&mut arr[next], &mut tmp);
    let mut pos = next;
    while pos != mid {
        let next = next_pos(pos);
        std::mem::swap(&mut arr[next], &mut tmp);
        pos = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_mergesort() {
        let mut arr = [7, 4, 10, 24, 3, 2, 1];
        mergesort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn ascending_mergesort() {
        let mut arr = [1, 2, 3, 4, 7, 10, 24];
        mergesort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn descending_mergesort() {
        let mut arr = [24, 10, 7, 4, 3, 2, 1];
        mergesort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }
}
