use std::{path::Path, fs::File, io::{self, BufRead}};


pub fn parse_int<S: AsRef<str>>(line: S) -> Option<i32> {
    line.as_ref().parse::<i32>().ok()
}

pub fn parse_usize<S: AsRef<str>>(line: S) -> Option<usize> {
    line.as_ref().parse::<usize>().ok()
}

pub fn read_input<P: AsRef<Path>>(path: P) -> impl Iterator<Item = String> {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .filter(|line| line.is_ok()) 
        .map(|line| line.expect("already filtered out errors"));
    return lines;
}