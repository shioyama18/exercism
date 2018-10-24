extern crate itertools;
use itertools::Itertools;

pub fn encode(source: &str) -> String {
    source.chars()
        .group_by(|&c| c)
        .into_iter()
        .map(|(c, group)| 
             match group.count() {
                 1 => c.to_string(),
                 n => format!("{}{}", n, c),
             })
        .collect()
}

pub fn decode(source: &str) -> String {
    let mut output = String::new();
    let mut group = String::new();

    for c in source.chars() {
        if c.is_numeric() {
            group.push(c);
        } else {
            let n = group.parse().unwrap_or(1);
            output += c.to_string().repeat(n).as_str();
            group.clear();
        }
    }
    output
}
