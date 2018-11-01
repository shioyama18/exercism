use std::collections::BTreeMap;

pub struct School {
    grades: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School { grades: BTreeMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let v = self.grades.entry(grade).or_insert(Vec::new());
        (*v).push(student.into());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().cloned().collect()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        if let Some(x) = self.grades.get(&grade) {
            let mut sorted = x.clone();
            sorted.sort();
            return Some(sorted);
        } else {
            return None;
        }
    }
}
