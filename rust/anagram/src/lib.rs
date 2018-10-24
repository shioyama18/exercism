use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut sorted_word = word.to_lowercase().chars().collect::<Vec<char>>();
    sorted_word.sort();

    possible_anagrams.iter()
        .cloned()
        .filter(|w| {
            let mut ws = w.to_lowercase().chars().collect::<Vec<char>>();
            ws.sort();
            word.to_lowercase() != w.to_lowercase() && ws == sorted_word
        })
        .collect()
}
