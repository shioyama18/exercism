extern crate threadpool;

use std::collections::HashMap;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let pool = ThreadPool::new(worker_count);
    let mut histogram = HashMap::new();
    let (sender, receiver) = channel();

    let mut n = 0;
    for word in input {
        let sender = sender.clone();
        let data = word.to_string();
        pool.execute(move || {
            if let Err(_) = sender.send(count_char(&data)) {
                panic!("Error in worker {}: {}", n, &data);
            }
        });
        n += 1;
    }

    for _ in 0..n {
        merge(&mut histogram, &receiver.recv().unwrap());
    }

    histogram
}

fn count_char(word: &str) -> HashMap<char, usize> {
    let mut histogram = HashMap::new();
    for c in word.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
        *histogram.entry(c).or_insert(0) += 1;
    }
    histogram
}

fn merge(a: &mut HashMap<char, usize>, b: &HashMap<char, usize>) {
    for (&k, &v) in b.iter() {
            *a.entry(k).or_insert(0) += v;
    }
}

