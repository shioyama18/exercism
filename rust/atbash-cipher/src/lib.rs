/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain.to_lowercase().chars()
        .filter_map(helper)
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|s| s.iter().cloned().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars()
        .filter_map(helper)
        .collect()
}

fn helper(c: char) -> Option<char> {
    match c {
        'a' ... 'z' => Some((b'a' + b'z' - c as u8) as char),
        '0' ... '9' => Some(c),
        _ => None
    }
}

