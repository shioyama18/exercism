pub fn number(user_number: &str) -> Option<String> {
    let mut input = user_number.chars()
        .filter(char::is_ascii_digit)
        .collect::<Vec<_>>();

    let mut output = String::new();

    if input.len() == 11 && input[0] == '1' {
        input.remove(0);
    }

    for (i, &c) in input.iter().enumerate() {
        match (i, c) {
            (0, '0'...'1') => return None,
            (3, '0'...'1') => return None,
            (_, _) if i > 9 => return None,
            (_, _) => output.push(c),
        }
    }

    Some(output)
}
