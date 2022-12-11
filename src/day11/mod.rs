use std::{path::Path, collections::VecDeque};

use crate::utils::{self, AOCError, parse_i64, parse_usize};

type WorryItem = i64;

#[derive(Debug)]
enum Op {
    Multiply(i64),
    MultiplyOld,
    Add(i64),
    AddOld,
}

impl Op {
    fn apply(&self, old: WorryItem) -> WorryItem {
        match self {
            Op::Multiply(x) => old * x,
            Op::MultiplyOld => old * old,
            Op::Add(x) => old + x,
            Op::AddOld => old + old,
        }
    }

    fn apply_mod(&self, old: WorryItem, modulo: i64) -> WorryItem {
        self.apply(old) % modulo
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<WorryItem>,
    operation: Op,
    test_divisible_by: i64,
    monkey_on_true: usize,
    monkey_on_false: usize,
    num_examinations: usize,
    modulo_space: Option<i64>,
    do_worry_div_three: bool,
}

impl Monkey {

    fn parse_monkeys(lines: &mut impl Iterator<Item = String>, do_worry_div_three: bool) -> Vec<Self> {
        let mut monkeys: Vec<Self> = Vec::new();
        loop {
            let monkey = Self::parse_monkey(lines, do_worry_div_three);
            if let Ok(monkey) = monkey {
                monkeys.push(monkey);
            } else {
                return monkeys;
            }
        } 
    }

    fn parse_monkey(lines: &mut impl Iterator<Item = String>, do_worry_div_three: bool) -> Result<Self, AOCError> {
        match lines.next() {
            Some(_) => Ok(()),
            None => return Err(AOCError::new("out of data")),
        }?;
        let starting_items: VecDeque<i64> = lines.next()
            .unwrap()
            .split("Starting items: ")
            .flat_map(|substr| substr.split(", "))
            .map(parse_i64)
            .filter(|x| x.is_some())
            .map(|x| x.expect("already filtered out Nones"))
            .collect();
        let op_string_line = lines.next().unwrap();
        let mut op_string_parts = op_string_line
            .split("new = old ")
            .last()
            .expect("must be present")
            .split(" ");
        let op = op_string_parts.next().unwrap();
        let operand = parse_i64(op_string_parts.next().unwrap());
        let operation = match (op, operand) {
            ("*", None) => Op::MultiplyOld,
            ("*", Some(i)) => Op::Multiply(i),
            ("+", None) => Op::AddOld,
            ("+", Some(i)) => Op::Add(i),
            _ => panic!("Unknown operation '{}'", op),
        };
        let test_divisible_by = lines.next().unwrap().split(" ").map(parse_i64).last().unwrap().unwrap();
        let monkey_on_true = lines.next().unwrap().split(" ").map(parse_usize).last().unwrap().unwrap();
        let monkey_on_false = lines.next().unwrap().split(" ").map(parse_usize).last().unwrap().unwrap();
        lines.next(); // consume newline (or not, idc)
        Ok(Self {
            items: starting_items,
            operation,
            test_divisible_by,
            monkey_on_true,
            monkey_on_false,
            num_examinations: 0,
            modulo_space: None,
            do_worry_div_three,
        })
    }

    fn examine_items(&mut self) -> Vec<(WorryItem, usize)> {
        let mut items_to_send = Vec::new();
        for _ in 0..self.items.len() {
            let item = self.items.pop_front().unwrap();
            let item = match self.modulo_space {
                Some(modulo) => self.operation.apply_mod(item, modulo),
                None => self.operation.apply(item),
            };
            let item = match self.do_worry_div_three { true => item / 3, false => item };
            let reciever_monkey = match item % self.test_divisible_by {
                0 => self.monkey_on_true,
                _ => self.monkey_on_false,
            };
            items_to_send.push((item, reciever_monkey));
            self.num_examinations += 1;
        }
        items_to_send
    }
}

fn perform_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        let item_distribution = monkeys.get_mut(i)
            .expect("index ok")
            .examine_items();
        for (item, reciever) in item_distribution {
            let monkey = monkeys.get_mut(reciever).expect("index ok");
            monkey.items.push_back(item);
        }
    }
}

pub fn answer_part_1<P: AsRef<Path>>(path: P) -> String {
    let mut lines = utils::read_input(path);
    let mut monkeys = Monkey::parse_monkeys(&mut lines, true);
    for _ in 0..20 {
        perform_round(&mut monkeys);
    }
    let mut num_examinations: Vec<usize> = monkeys.iter()
        .map(|monkey| monkey.num_examinations)
        .collect();
    num_examinations.sort();
    let monkey_business = num_examinations.into_iter()
        .rev()
        .take(2)
        .reduce(|a, b| a * b)
        .unwrap();
    monkey_business.to_string()
}

pub fn answer_part_2<P: AsRef<Path>>(path: P) -> String {
    let mut lines = utils::read_input(path);
    let mut monkeys = Monkey::parse_monkeys(&mut lines, false);
    let modulo_space: i64 = monkeys.iter()
        .map(|monkey| monkey.test_divisible_by)
        .reduce(|a, b| a * b)
        .unwrap();
    println!("modulo space: {}", modulo_space);
    // set the modulo space to prevent overflow
    for i in 0..monkeys.len() {
        monkeys.get_mut(i).unwrap().modulo_space = Some(modulo_space);
    }
    for _ in 0..10000 {
        perform_round(&mut monkeys);
    }
    let mut num_examinations: Vec<usize> = monkeys.iter()
        .map(|monkey| monkey.num_examinations)
        .collect();
    num_examinations.sort();
    let monkey_business = num_examinations.into_iter()
        .rev()
        .take(2)
        .reduce(|a, b| a * b)
        .unwrap();
    monkey_business.to_string()
}