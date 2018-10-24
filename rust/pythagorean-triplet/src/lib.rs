pub fn find() -> Option<u32> {
    for c in 2..1000 {
        for a in 1..c {
            let b = 1000 - c - a;
            if a*a + b*b == c*c {
                return Some(a * b * c);
            }
        }
    }
    None
}

