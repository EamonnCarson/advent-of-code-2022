use std::io::{self, BufRead};
use std::path::Path;
use std::fs::{self, File};

struct ElfSack {
    items: Vec<ElfSackObject>
}

impl ElfSack {
    fn new() -> Self {
        ElfSack { items: vec![] }
    }

    fn total_calories(&self) -> i32 {
        self.items
            .iter()
            .map(|item| match item {
                ElfSackObject::Calories(c) => *c,
            })
            .reduce(|total, c| c + total)
            .unwrap_or(0)
    }

    /// this question did not go where I thought it was going
    /// useless
    fn max_calories(&self) -> Option<i32> {
        self.items
            .iter()
            .map(|item| match item {
                ElfSackObject::Calories(c) => *c,
            })
            .reduce(|total, c| c.max(total))
    }
}

impl AsRef<Vec<ElfSackObject>> for ElfSack {
    fn as_ref(&self) -> &Vec<ElfSackObject> {
        &self.items
    }
}

impl AsMut<Vec<ElfSackObject>> for ElfSack {
    fn as_mut(&mut self) -> &mut Vec<ElfSackObject> {
        &mut self.items
    }
}

enum ElfSackObject {
    Calories(i32),
}

fn read_input<P: AsRef<Path>>(file: P) -> Vec<ElfSack> {
    let file = File::open(file).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut elf_sacks: Vec<ElfSack> = vec![ElfSack::new()];
    for line in lines {
        match line {
            Ok(line) => {
                match parse_int(line) {
                    Some(calories) => {
                        let sack = elf_sacks.last_mut()
                            .expect("we populated a sack so it's there");
                        sack.items.push(ElfSackObject::Calories(calories));
                    }
                    None => {
                        elf_sacks.push(ElfSack::new());
                    },
                }
            },
            Err(_) => {},
        }
    }
    elf_sacks
}

fn parse_int<S: AsRef<str>>(line: S) -> Option<i32> {
    line.as_ref().parse::<i32>().ok()
}

pub fn answer_part_1<P: AsRef<Path>>(path: P) {
    let elf_sacks = read_input(path);
    let max_total_calories_in_a_elf_sack = elf_sacks.iter()
        .map(|sack| sack.total_calories())
        .reduce(|accumulator, sack| accumulator.max(sack));
    println!("{:?}", max_total_calories_in_a_elf_sack);
}

pub fn answer_part_2<P: AsRef<Path>>(path: P) {
    let elf_sacks = read_input(path);
    // I could probably use some fancy binary tree top-k algorithm stuff
    // but a man's on a schedule over here.
    let mut elf_sack_calories: Vec<i32> = elf_sacks.iter()
        .map(|sack| sack.total_calories())
        .collect();
    elf_sack_calories.sort();
    let num_sacks = elf_sack_calories.len();
    let largest_3_sacks = &elf_sack_calories[0.max(num_sacks - 3)..];
    let sum = largest_3_sacks.into_iter().map(|a| *a).reduce(|a, b| a + b);
    println!("{:?}", sum);
}