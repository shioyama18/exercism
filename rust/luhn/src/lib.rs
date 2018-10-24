/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = 0;
    let mut len = 0;

    for (i, c) in code.chars().rev().filter(|&c| !c.is_whitespace()).enumerate() {
        len += 1;
        match (i % 2, c.to_digit(10)) {
            (1, Some(x)) if x > 4 => sum += x * 2 - 9,
            (1, Some(x)) => sum += x * 2,
            (0, Some(x)) => sum += x,
            (_, _) => return false,
        }
    }

    (len > 1) && (sum % 10 == 0)
}
