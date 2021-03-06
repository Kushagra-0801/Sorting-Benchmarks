use std::time;

fn is_sorted<T: Ord + Copy>(arr: &[T]) -> bool {
    for win in arr.windows(2) {
        if win[1] < win[0] {
            return false;
        }
    }
    true
}

pub fn time_it<T: Ord + Copy>(f: impl Fn(&mut [T]), arr: Vec<T>) -> time::Duration {
    let arrs: Vec<Vec<_>> = (0..10).map(move |_| arr.clone()).collect();
    let mut time_taken = time::Duration::new(0, 0);
    arrs.into_iter().for_each(|mut arr| {
        let start = time::Instant::now();
        f(&mut arr);
        time_taken += start.elapsed();
        assert!(is_sorted(&arr));
    });
    time_taken / 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer() {
        let now = time::Instant::now();
        let mut arr = vec![1, 2];
        let dur = time_it(
            |_| {
                std::thread::sleep(time::Duration::from_secs(1));
            },
            arr,
        )
        .as_secs();
        let timer_dur = now.elapsed().as_secs();
        assert!(dur == timer_dur)
    }
}
