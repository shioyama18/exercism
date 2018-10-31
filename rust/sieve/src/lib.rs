pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut candidates = vec![true; upper_bound as usize + 1];

    for i in 2..(upper_bound as f64).sqrt().ceil() as usize {
        if candidates[i] {
            for j in (i * i..upper_bound as usize + 1).step_by(i) {
                candidates[j] = false;
            }
        }
    }

    for i in 2..upper_bound as usize + 1 {
        if candidates[i] {
            primes.push(i as u64);
        }
    }

    primes
}
