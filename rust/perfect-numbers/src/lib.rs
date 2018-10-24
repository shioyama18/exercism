use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num < 1 { 
        None
    } else {
        match aliquot_sum(num).cmp(&num) {
            Ordering::Greater => Some(Classification::Abundant),
            Ordering::Less => Some(Classification::Deficient),
            Ordering::Equal => Some(Classification::Perfect),
        }
    }
}

fn aliquot_sum(num: u64) -> u64 {
    (1..num / 2 + 1).filter(|i| num % i == 0).sum()
}

