use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    // // Functional Programming
    // h.iter().flat_map(|(score, char_vec)| {
    //     char_vec.into_iter()
    //         .collect::<String>()
    //         .to_lowercase()
    //         .chars()
    //         .map(|c| (c, score.to_owned()))
    //         .collect::<Vec<_>>()
    // }).collect()

    let mut output = BTreeMap::new();

    for (k, v) in h.iter() {
        let s: String = v.into_iter().collect();
        for c in s.to_lowercase().chars() {
            output.insert(c, k.to_owned());
        }
    }

    output
}
