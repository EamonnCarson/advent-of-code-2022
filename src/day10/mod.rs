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
    let lines = utils::read_input(path);
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
    let lines = utils::read_input(path);
    let xs = lines
        .map(Instruction::parse)
        .flat_map(|instruction| instruction.factor_out().into_iter())
        .scan(1, |x, instruction| {
            let x_during = *x;
            *x = match instruction {
                Instruction::Noop() => *x,
                Instruction::AddX(y) => *x + y,
            };
            Some(x_during)
        });
    //let xs = vec![1].into_iter().chain(xs);
    let pixel_being_rendered = (0..6).flat_map(|_| 0..40);
    let pixels = xs.zip(pixel_being_rendered)
        .map(|(x, pixel_being_rendered)| {
            //println!("pixel: {}, x: {}", &pixel_being_rendered, &x);
            (x - pixel_being_rendered).abs() <= 1
        })
        .map(|x| match x { true => "#", false => "."});
    let rendered_pixels: Vec<&str> = pixels.collect();
    let lines: Vec<String> = (0..6)
        .map(|x| rendered_pixels[x*40..((x+1) * 40)].join(""))
        .collect();
    return lines.join("\n");
}