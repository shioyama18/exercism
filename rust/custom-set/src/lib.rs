use std::fmt::Debug;


#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    // Not using HashSet to implement `Custom` set
    vec: Vec<T>
}

impl<T: Debug + Ord + Eq + Clone> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut vec = input.to_vec();
        vec.sort();
        vec.dedup();
        CustomSet { vec }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.vec.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.vec.push(element);
            self.vec.sort();
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.vec.iter().all(|item| other.contains(item))
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.vec.iter().any(|item| other.contains(item))
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let vec = self.vec.iter()
            .filter(|item| other.contains(item))
            .map(|item| item.clone())
            .collect::<Vec<_>>();
        CustomSet { vec }
    }

    pub fn difference(&self, other: &Self) -> Self {
        let vec = self.vec.iter()
            .filter(|item| !other.contains(item))
            .map(|item| item.clone())
            .collect::<Vec<_>>();
        CustomSet { vec }
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut vec = Vec::new();
        vec.extend(self.vec.clone());
        vec.extend(other.vec.clone());
        CustomSet::new(&vec)
    }
}
