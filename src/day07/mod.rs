use std::{path::Path, rc::Weak};

use crate::utils;

enum FileSystemObject {
    Directory {name: String, contents: Vec<FileSystemObject>, parent: Weak<FileSystemObject>},
    File {name: String, size: usize, parent: Weak<FileSystemObject>},
}

pub fn answer_part_1<P: AsRef<Path>>(path: P) -> String {
    let mut lines = utils::read_input(path);
    "n/a".to_string()
}

pub fn answer_part_2<P: AsRef<Path>>(path: P) -> String {
    let mut lines = utils::read_input(path);
    "n/a".to_string()
}