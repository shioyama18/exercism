pub fn rotate(input: &str, key: u8) -> String {
    let rotate = |c, ascii_code| ascii_code + (c - ascii_code + key) % 26;

    input.chars()
        .map(|c| {
            match c {
                x if x.is_ascii_uppercase() => {
                    rotate(x as u8, b'A') as char
                }
                x if x.is_ascii_lowercase() => {
                    rotate(x as u8, b'a') as char
                }
                _ => c,
            }
        })
        .collect()
}
