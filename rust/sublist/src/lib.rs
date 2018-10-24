#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(l1: &[T], l2: &[T]) -> Comparison {
    match (l1.len(), l2.len()) {
        (n1, n2) if n1 < n2 => 
            if n1 == 0 || l2.windows(n1).any(|x| x == l1) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            },
        (n1, n2) if n1 > n2 => 
            if n2 == 0 || l1.windows(n2).any(|x| x == l2) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            },
        (n1, _) => 
            if n1 == 0 || l1 == l2 {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
    }
}
