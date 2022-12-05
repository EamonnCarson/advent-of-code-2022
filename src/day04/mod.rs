use std::{fmt::Debug, io::{self, BufRead}, fs::File, path::Path, ops::RangeBounds};


struct Pair {
    start: i32,
    end: i32,
}

impl Pair {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        // only two cases where no overlap
        // a b  | |
        // | |  a b
        // best to think of it as area over overlap.
        // area of overlap is min(ends) - max(starts)
        // which in both of the above cases is zero
        // so test is
        self.end.min(other.end) >= self.start.max(other.start)
        // Note: the >= is because our ranges are inclusive
    }
}

/// Parse i32 from a string
fn parse_int<S: AsRef<str>>(line: S) -> Option<i32> {
    line.as_ref().parse::<i32>().ok()
}

/// Parses X-Y into a pair
fn parse_range<S: AsRef<str> + Debug>(range: S) -> Result<Pair, io::Error> {
    let mut bounds = range.as_ref().trim().split("-");
    let start = bounds.next().and_then(parse_int);
    let end = bounds.next().and_then(parse_int);
    match (start, end) {
        (Some(start), Some(end)) => Ok(Pair { start, end }),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData, 
            format!("Invalid range format {:?}", range)
        ))
    }
}

/// Parses X-Y,W-Z into two pairs
fn parse_line<S: AsRef<str> + Debug>(line: S) -> Result<(Pair, Pair), io::Error> {
    let mut ranges = line.as_ref().trim().split(",");
    let range1 = ranges.next();
    let range2 = ranges.next();
    match (range1, range2) {
        (Some(range1), Some(range2)) =>
            Ok((parse_range(range1)?, parse_range(range2)?)),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData, 
            format!("Invalid line format {:?}", line)
        ))
    }
}

pub fn answer_part_1<P: AsRef<Path>>(path: P) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .filter(|line| line.is_ok()) 
        .map(|line| line.expect("already filtered out errors"));
    let num_fully_overlapping_pairs = lines
        .map(|line| parse_line(line).unwrap())
        .filter(|(range1, range2)| range1.contains(range2) || range2.contains(range1))
        .count();
    println!("{:?}", num_fully_overlapping_pairs);
}


pub fn answer_part_2<P: AsRef<Path>>(path: P) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .filter(|line| line.is_ok()) 
        .map(|line| line.expect("already filtered out errors"));
    let num_overlapping_pairs = lines
        .map(|line| parse_line(line).unwrap())
        .filter(|(range1, range2)| range1.overlaps(range2))
        .count();
    println!("{:?}", num_overlapping_pairs);
}
