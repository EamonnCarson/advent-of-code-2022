use std::path::Path;

use crate::utils;

enum Instruction {
    Noop(),
    AddX(i32),
}

impl Instruction {
    fn parse(line: String) -> Instruction {
        let mut parts = line.trim().split(" ");
        let name = parts.next().unwrap();
        match name {
            "noop" => Instruction::Noop(),
            "addx" => {
                let value = utils::parse_int(parts.next().unwrap()).unwrap();
                Instruction::AddX(value)
            },
            _ => panic!("Cannot understand instruction {}", name),
        }
    }

    fn factor_out(&self) -> Vec<Instruction> {
        match self {
            Instruction::Noop() => vec![Instruction::Noop()],
            Instruction::AddX(value) => vec![Instruction::Noop(), Instruction::AddX(*value)],
        }
    }
}

pub fn answer_part_1<P: AsRef<Path>>(path: P) -> String {
    let mut lines = utils::read_input(path);
    let (final_x, signal_strength_total) = lines
        .map(Instruction::parse)
        .flat_map(|instruction| instruction.factor_out().into_iter())
        .enumerate()
        .fold((1, 0), |(x, signal_strength_total), (cycle, instruction)| {
            let cycle = (cycle as i32) + 1;
            let signal_strength_total = match (cycle - 20) % 40 {
                0 => signal_strength_total + x * cycle,
                _ => signal_strength_total,
            };
            let x = match instruction {
                Instruction::Noop() => x,
                Instruction::AddX(y) => x + y,
            };
            (x, signal_strength_total)
        });
    signal_strength_total.to_string()
}

pub fn answer_part_2<P: AsRef<Path>>(path: P) -> String {
    let mut lines = utils::read_input(path);
    "n/a".to_string()
}