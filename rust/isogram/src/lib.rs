use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::new();

    candidate.to_lowercase().chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| set.insert(c))  // insert method returns fall if c already exists in set
}
