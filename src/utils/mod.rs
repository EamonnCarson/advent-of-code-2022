
pub fn parse_int<S: AsRef<str>>(line: S) -> Option<i32> {
    line.as_ref().parse::<i32>().ok()
}

pub fn parse_usize<S: AsRef<str>>(line: S) -> Option<usize> {
    line.as_ref().parse::<usize>().ok()
}