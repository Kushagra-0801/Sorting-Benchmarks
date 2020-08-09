struct Gap {
    gap: usize,
}

impl Iterator for Gap {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let gap = self.gap;
        if gap > 0 {
            let next_gap = (gap - 1) / 3;
            self.gap = next_gap;
            Some(gap)
        } else {
            None
        }
    }
}

fn gap_sequence(len: usize) -> Gap {
    let max = (len + 2) / 3;
    let mut gap = 1;
    while (3 * gap + 1) <= max {
        gap = 3 * gap + 1;
    }
    Gap { gap }
}

pub fn shell_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    let gaps = gap_sequence(len);
    for gap in gaps {
        for i in gap..len {
            let mut j = i;
            while j >= gap && arr[j - gap] > arr[j] {
                arr.swap(j, j - gap);
                j -= gap;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_shell_sort() {
        let mut arr = [7, 4, 10, 24, 3, 2, 1];
        shell_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }

    #[test]
    fn simple_shell_sort_descending() {
        let mut arr = [24, 10, 7, 4, 3, 2, 1];
        shell_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 7, 10, 24]);
    }
}
