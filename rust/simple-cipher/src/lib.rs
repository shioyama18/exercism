extern crate rand;

use rand::distributions::Uniform;
use rand::prelude::*;


pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || !key.chars().all(|c| c.is_ascii_lowercase()) {
        None
    } else {
        let key = key.chars()
            .map(|k| k as u8 - b'a')
            .collect::<Vec<_>>();
        shift(&key, s)
    }
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || !key.chars().all(|c| c.is_ascii_lowercase()) {
        None
    } else {
        let key = key.chars()
            .map(|k| 26 - (k as u8 - b'a'))
            .collect::<Vec<_>>();
        shift(&key, s)
    }
}

fn shift(key: &[u8], s: &str) -> Option<String> {
    if s.chars().any(|c| !c.is_ascii_lowercase()) {
        None
    } else {
        Some(
            s.chars()
                .zip(key.iter().cycle())
                .map(|(c, k)| (b'a' + (c as u8 - b'a' + k) % 26) as char)
                .collect(),
        )
    }
}


pub fn encode_random(s: &str) -> (String, String) {
    let key: Vec<u8> = thread_rng()
        .sample_iter(&Uniform::new(0, 26))
        .take(100)
        .collect();

    (
        key.iter().map(|k| (b'a' + k) as char).collect(),
        shift(&key, s).unwrap(),
    )
}
