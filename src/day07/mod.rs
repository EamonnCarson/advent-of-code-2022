use std::{path::Path, rc::Weak};

use crate::utils;

struct Directory {
    parent: Weak<Directory>, // weak reference upwards
    files: Vec<File>,
    directories: Vec<Directory>,
}

struct File {
    name: String,
    size: usize,
}


pub fn answer_part_1<P: AsRef<Path>>(path: P) -> String {
    let mut lines = utils::read_input(path);
    "n/a".to_string()
}

pub fn answer_part_2<P: AsRef<Path>>(path: P) -> String {
    let mut lines = utils::read_input(path);
    "n/a".to_string()
}