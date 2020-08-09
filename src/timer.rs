use std::time;

fn is_sorted<T: Ord + Copy>(arr: &[T]) -> bool {
    for win in arr.windows(2) {
        if win[1] < win[0] {
            return false;
        }
    }
    true
}

pub fn time<T: Ord + Copy>(f: impl Fn(&mut [T]), arr: &mut [T]) -> time::Duration {
    let start = time::Instant::now();
    f(arr);
    let elapsed = start.elapsed();
    assert!(is_sorted(arr));
    elapsed
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer() {
        let now = time::Instant::now();
        let mut arr = [1, 2];
        let dur = time(
            |_| {
                std::thread::sleep(time::Duration::from_secs(1));
            },
            &mut arr,
        )
        .as_secs();
        let timer_dur = now.elapsed().as_secs();
        assert!(dur == timer_dur)
    }
}
