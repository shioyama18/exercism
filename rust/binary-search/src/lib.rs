use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let n = array.len();

    if n == 0 {
        return None;
    }

    let mid_i = array.len() / 2;
    let mid = array[mid_i];

    match mid.cmp(&key) {
        Ordering::Less    => match find(&array[mid_i+1..], key) {
            Some(i) => Some(mid_i + 1 + i),
            None    => None,
        }
        Ordering::Equal   => Some(mid_i),
        Ordering::Greater => find(&array[..mid_i], key),
    }
}
