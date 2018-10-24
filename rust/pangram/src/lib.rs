use std::collections::HashSet;

const ALPHA_LENGTH: usize = 26;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    ALPHA_LENGTH == sentence.chars()
        .filter(char::is_ascii_alphabetic)
        .flat_map(char::to_lowercase)
        .collect::<HashSet<_>>().len()
}
