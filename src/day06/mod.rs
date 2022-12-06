use std::{path::Path, collections::{self, HashMap, VecDeque}, ops::Deref, hash::Hash};

use crate::utils;

struct MultiSet<T> where T: Eq + Hash {
    hash_map: HashMap<T, u32>,
    size: usize,
}

impl<T: Eq + Hash> MultiSet<T> {
    fn new() -> Self {
        Self { hash_map: HashMap::<T, u32>::new() , size: 0}
    }

    fn add(&mut self, item: T) {
        let count = self.hash_map.get(&item);
        self.size += 1;
        match count {
            None => self.hash_map.insert(item, 1),
            Some(count) => self.hash_map.insert(item, count + 1),
        };
    }

    fn remove(&mut self, item: T) {
        let count = self.hash_map.get(&item);
        if count.is_some() {
            self.size -= 1;
        }
        match count {
            None => self.hash_map.insert(item, 1),
            Some(&1) => self.hash_map.remove(&item),
            Some(count) => self.hash_map.insert(item, count - 1),
        };
    }

    fn len(&self) -> usize {
        self.size
    }

    fn unique_len(&self) -> usize {
        self.hash_map.len()
    }
}

pub fn index_of_unique_run(input: String, unique_run_len: usize) -> Option<usize> {
    let mut queue = VecDeque::<char>::new();
    let mut multiset = MultiSet::<char>::new();
    for (index, c) in input.chars().enumerate() {
        queue.push_back(c);
        multiset.add(c);
        if queue.len() > unique_run_len {
            let popped = queue.pop_front().expect("queue not empty");
            multiset.remove(popped);
        }
        if multiset.unique_len() == unique_run_len {
            return Some(index);
        }
    }
    return None;

}

pub fn answer_part_1<P: AsRef<Path>>(path: P) -> String {
    let mut lines = utils::read_input(path);
    let input = lines.next().unwrap(); // only one line
    let index = index_of_unique_run(input, 4);
    // index + 1 because answer is one indexed
    match index.and_then(|x| Some(x + 1)) {
        Some(i) => i.to_string(),
        None => "None".to_string(),
    }
}

pub fn answer_part_2<P: AsRef<Path>>(path: P) -> String {
    let mut lines = utils::read_input(path);
    let input = lines.next().unwrap(); // only one line
    let index = index_of_unique_run(input, 14);
    // index + 1 because answer is one indexed
    match index.and_then(|x| Some(x + 1)) {
        Some(i) => i.to_string(),
        None => "None".to_string(),
    }
}