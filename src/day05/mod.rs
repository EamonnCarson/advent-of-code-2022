use std::{fmt::{Display, Debug}, path::Path, fs::File, io::{self, BufRead}};

use crate::utils;


type Stack<T> = Vec<T>;

#[derive(Debug)]
struct Crate {
    id: char,
}

impl Crate {
    fn new_from_char(char: char) -> Option<Crate> {
        if char.is_ascii_alphabetic() {
            Some(Crate { id: char })
        } else {
            None
        }
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("[{}]", self.id))
    }
}

struct MoveInstruction {
    from: usize,
    to: usize,
    amount: usize,
}

impl Display for MoveInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("move {} from {} to {}", self.amount, self.to, self.from))
    }
}

impl MoveInstruction {
    /// parses instruction like "move 2 from 8 to 4"
    fn new_from_instruction<S: AsRef<str> + Debug>(line: S) -> Result<Self, std::io::Error> {
        let mut numbers_in_line = line
            .as_ref()
            .split(" ")
            .map(utils::parse_usize)
            .filter(|x| x.is_some())
            .map(|x| x.expect("filtered out the Nones"));
        let amount = numbers_in_line.next();
        let from = numbers_in_line.next();
        let to = numbers_in_line.next();
        match (amount, from, to) {
            (Some(amount), Some(from), Some(to)) => 
                // minus 1 because of 0 indexing
                Ok(Self {amount: amount, from: from - 1 , to: to - 1}),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData, 
                format!("Invalid line format {:?}", line),
            ))
        }
    }

    fn perform_9000(&self, crate_stacks: &mut Vec<Stack<Crate>>) {
        for _ in 0..self.amount {
            let from_stack = crate_stacks
                .get_mut(self.from)
                .expect(&format!("Instruction {} must point to valid index", self));
            let my_crate = from_stack
                .pop()
                .expect(&format!("Instruction {} must be valid (crate missing)", self));
            let to_stack = crate_stacks
                .get_mut(self.to)
                .expect(&format!("Instruction {} must point to valid index", self));
            to_stack.push(my_crate);
        }
    }

    fn perform_9001(&self, crate_stacks: &mut Vec<Stack<Crate>>) {
        let mut crates_picked_up: Stack<Crate> = Stack::new();
        let from_stack = crate_stacks
            .get_mut(self.from)
            .expect(&format!("Instruction {} must point to valid index", self));
        for _ in 0..self.amount {
            let my_crate = from_stack
                .pop()
                .expect(&format!("Instruction {} must be valid (crate missing)", self));
            crates_picked_up.push(my_crate);
        }
        let to_stack = crate_stacks
            .get_mut(self.to)
            .expect(&format!("Instruction {} must point to valid index", self));
        crates_picked_up.reverse();
        for my_crate in crates_picked_up {
            to_stack.push(my_crate);
        }
    }
}

/// Parses the crate diagram into a list of stacks of crates
/// input should be something like:
/// 
/// [W] [V]     [P]                    
/// [B] [T]     [C] [B]     [G]        
/// [G] [S]     [V] [H] [N] [T]        
/// [Z] [B] [W] [J] [D] [M] [S]        
/// [R] [C] [N] [N] [F] [W] [C]     [W]
/// [D] [F] [S] [M] [L] [T] [L] [Z] [Z]
/// [C] [W] [B] [G] [S] [V] [F] [D] [N]
/// [V] [G] [C] [Q] [T] [J] [P] [B] [M]
///  1   2   3   4   5   6   7   8   9 
/// 
/// (i.e. skip the numbers at the end)
fn parse_crate_diagram<S: AsRef<str>>(mut lines: Vec<S>) -> Vec<Stack<Crate>> {
    lines.pop(); // throw away number line (maybe use it for numbers if I cared)
    lines.reverse();
    let num_stacks = {
        let num_chars_in_base_line = lines
            .get(0)
            .expect("need to have at least one line in diagram")
            .as_ref()
            .chars()
            .count();
        // need a +1 here because the last space is replaced by newline
        (num_chars_in_base_line + 1) / 4
    };
    let mut stacks = Vec::<Stack<Crate>>::new();
    for _ in 0..num_stacks {
        stacks.push(vec![]);
    }
    lines
        .iter()
        .flat_map(|line| parse_crates_from_diagram_line(line)
            .into_iter()
            .enumerate()
        )
        .fold(stacks, |mut stacks, (stack_ix, my_crate)| {
            if let Some(my_crate) = my_crate {
                let stack = stacks
                    .get_mut(stack_ix)
                    .expect("We calculated the length of the crates, so we should be good");
                stack.push(my_crate);
            }
            stacks
        })
}

fn parse_crates_from_diagram_line<S: AsRef<str>>(line: S) -> Vec<Option<Crate>> {
    line
        .as_ref()
        .chars()
        .enumerate()
        // yes this is really hacky, but I don't want to spend time on boring things 
        // like input processing
        .filter(|(i, char)| i % 4 == 1)
        .map(|(_, char)| Crate::new_from_char(char))
        .collect()
}

pub fn answer_part_1<P: AsRef<Path>>(path: P) {
    let file = File::open(path).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .filter(|line| line.is_ok()) 
        .map(|line| line.expect("already filtered out errors"));

    // parse crate diagram
    let mut crate_diagram_lines = vec![];
    while let Some(line ) = lines.next() {
        if line.trim().len() == 0 {
            break;
        } else {
            crate_diagram_lines.push(line);
        }
    }
    let mut crate_stacks = parse_crate_diagram(crate_diagram_lines);

    // parse move instructions
    lines
        .map(MoveInstruction::new_from_instruction)
        .map(|instruction| instruction.unwrap())
        .map(|instruction| instruction.perform_9000(&mut crate_stacks))
        .count();

    // get tops of stacks
    let tops: Vec<String> = crate_stacks
        .iter()
        .map(|stack| stack.last())
        .map(|my_crate| my_crate.and_then(|x| Some(x.id.to_string())).unwrap_or(" ".to_string()))
        .collect();
    println!("Crate stack tops: {:#?} ", tops.join(""));
}

pub fn answer_part_2<P: AsRef<Path>>(path: P) {
    let file = File::open(path).unwrap();
    let mut lines = io::BufReader::new(file)
        .lines()
        .filter(|line| line.is_ok()) 
        .map(|line| line.expect("already filtered out errors"));

    // parse crate diagram
    let mut crate_diagram_lines = vec![];
    while let Some(line ) = lines.next() {
        if line.trim().len() == 0 {
            break;
        } else {
            crate_diagram_lines.push(line);
        }
    }
    let mut crate_stacks = parse_crate_diagram(crate_diagram_lines);

    // parse move instructions
    lines
        .map(MoveInstruction::new_from_instruction)
        .map(|instruction| instruction.unwrap())
        .map(|instruction| instruction.perform_9001(&mut crate_stacks))
        .count();

    // get tops of stacks
    let tops: Vec<String> = crate_stacks
        .iter()
        .map(|stack| stack.last())
        .map(|my_crate| my_crate.and_then(|x| Some(x.id.to_string())).unwrap_or(" ".to_string()))
        .collect();
    println!("Crate stack tops: {:#?} ", tops.join(""));
}

#[test]
fn test_parse_crate_diagram() {
    let input = vec![
        "[W] [V]     [P]                    ",
        "[B] [T]     [C] [B]     [G]        ",
        "[G] [S]     [V] [H] [N] [T]        ",
        "[Z] [B] [W] [J] [D] [M] [S]        ",
        "[R] [C] [N] [N] [F] [W] [C]     [W]",
        "[D] [F] [S] [M] [L] [T] [L] [Z] [Z]",
        "[C] [W] [B] [G] [S] [V] [F] [D] [N]",
        "[V] [G] [C] [Q] [T] [J] [P] [B] [M]",
        " 1   2   3   4   5   6   7   8   9 ",
    ];
    let crate_stacks = parse_crate_diagram(input);
    println!("{:#?}", crate_stacks);
    assert_eq!(crate_stacks.len(), 9);
}