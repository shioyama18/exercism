pub fn nth(n: usize) -> usize {
    if n == 0 { return 2 }
        
    let mut p = Primes::new();

    while let Some(_) = p.next() {
        if p.primes.len() - 1 == n {
            return p.primes[n];
        }
    }
    0
}

struct Primes {
    primes: Vec<usize>,
    current: usize,
}

impl Primes {
    fn new() -> Self {
        Primes { primes: Vec::new(), current: 2 }
    }
}

impl Iterator for Primes {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        for i in self.current..std::usize::MAX {
            if self.primes.iter().all(|x| i % x != 0) {
                self.primes.push(i);
                self.current = i+1;
                return Some(i);
            }
        }
        None
    }
}

