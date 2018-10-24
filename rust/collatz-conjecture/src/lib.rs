pub fn collatz(mut n: u64) -> Option<u64> {
    let mut count = 0;
    loop {
        match n {
            0 => return None,
            1 => return Some(count),
            x if x % 2 == 0 => n /= 2,
            _ => n = 3 * n + 1,
        }
        count += 1;
    }
}
