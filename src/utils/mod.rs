use std::{path::Path, fs::File, io::{self, BufRead}, fmt::Display};


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

#[derive(Debug)]
pub struct AOCError {
    message: String
}

impl AOCError {
    pub fn new<S: AsRef<str>>(message: S) -> Self {
        Self { message: message.as_ref().to_string() }
    }
}

impl Display for AOCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("AOCError: {}", self.message))
    }
}

pub type AOCResult<T> = Result<T, AOCError>;