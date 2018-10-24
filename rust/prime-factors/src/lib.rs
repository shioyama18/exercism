pub fn factors(n: u64) -> Vec<u64> {
    let mut x = n;
    let mut output = Vec::<u64>::new();

    for i in (2..).take_while(|i| i * i <= n) {
        while x % i == 0 {
            output.push(i);
            x /= i;
        }
    }
    if x != 1 {
        output.push(x);
    }
    output
}


