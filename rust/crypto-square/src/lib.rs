pub fn encrypt(input: &str) -> String {
    let normalized = input.to_lowercase().chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<Vec<_>>();

    if normalized.is_empty() {
        return "".to_string();
    }

    let col = (normalized.len() as f64).sqrt().ceil() as usize;

    normalized.chunks(col)
        .fold(vec![String::new(); col], |mut v, chunk| {
            for (i, &c) in chunk.iter().enumerate() {
                v[i].push(c);
            }

            // padding
            if chunk.len() < col {
                let fill = col - chunk.len();
                (col - fill..col).for_each(|i| v[i].push(' '));
            }

            v
        })
        .join(" ")

}
