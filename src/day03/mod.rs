/*
Thoughts:
- My first thought is that we could xor bit vectors to do this
- but while that would be pretty efficient, there isn't really need to go after such
  a technical solution since we don't seem to need to store these vectors long term.
  so the only benefit would be the speed of xor over the other solution.
- better to use something like a set. Since our elements are easily enumerable, it's
  easy to do so with an array
*/

use std::{fmt::Debug, path::Path, fs::File, io::{self, BufRead}, collections::HashSet};

const ASCII_CODE_OF_LOWER_A: u32 = 97;
const ASCII_CODE_OF_UPPER_A: u32 = 65;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Item {
    priority: u32
}

impl AsRef<u32> for Item {
    fn as_ref(&self) -> &u32 {
        &self.priority
    }
}

impl Item {
    fn new_from_char(letter: char) -> Self {
        match letter {
            _ if letter.is_ascii_uppercase() => {
                let base_priority = 27;
                let priority = u32::from(letter).checked_sub(ASCII_CODE_OF_UPPER_A);
                Self { priority: base_priority + priority.expect("A must be lowest ascii uppercase") } 
                },
            _ if letter.is_ascii_lowercase() => {
                let base_priority = 1;
                let priority = u32::from(letter).checked_sub(ASCII_CODE_OF_LOWER_A);
                Self { priority: base_priority + priority.expect("a must be lowest ascii lowercase") }
                },
            _ => panic!("Char {:?} is not a valid Item indicator", letter),
        }
    }
}


fn chars_to_item_set<S>(chars: S) -> HashSet<Item> where S: AsRef<str> {
    chars.as_ref()
        .trim()
        .chars()
        .enumerate()
        .map(|(_, letter)| Item::new_from_char(letter))
        .collect()
}

pub fn split_line_in_half<'a>(line: &'a str) -> (&'a str, &'a str) {
    let line = line.trim();
    assert!(line.is_ascii()); // length below not valid if not ascii
    let num_items = line.len();
    assert_eq!(num_items % 2, 0);
    line.split_at(num_items / 2)
}


pub fn answer_part_1<P: AsRef<Path>>(path: P) {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();
    // To make this easier semantically, the rucksack has a left and right compartment
    let priority_sum: Option<u32> = lines
        .filter(|line| line.is_ok()) 
        .map(|line| line.expect("already filtered out errors"))
        .map(|line| {
            let (left, right) = split_line_in_half(line.as_str());
            let left_item_set = chars_to_item_set(left);
            let right_item_set = chars_to_item_set(right);
            let overlapping_items: Vec<&Item> = left_item_set
                .intersection(&right_item_set)
                .collect();
            match overlapping_items.last() {
                Some(item) => item.priority,
                None => 0,
            }
        })
        .reduce(|a, b| a + b);
    println!("Sum of priorities {:?}", priority_sum)
}
