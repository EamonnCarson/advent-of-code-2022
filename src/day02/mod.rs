use std::{path::Path, fs::{File, read}, io::{self, BufRead}, fmt::Display};

enum RPSResult {
    Win,
    Tie,
    Loss,
}

impl RPSResult {
    fn score(&self) -> i32 {
        match self {
            RPSResult::Win => 6,
            RPSResult::Tie => 3,
            RPSResult::Loss => 0,
        }
    }

    fn new_from_letter<S>(letter: S) -> Self where S: std::fmt::Debug + AsRef<str> {
        match letter.as_ref().to_lowercase().as_str() {
            "a" => RPSResult::Loss,
            "b" => RPSResult::Tie,
            "c" => RPSResult::Win,
            "x" => RPSResult::Loss,
            "y" => RPSResult::Tie,
            "z" => RPSResult::Win,
            _ => panic!("Letter {:?} has no associated RPSResult", letter)
        }
    }
}

enum RPSAction {
    Rock,
    Paper,
    Scissors,
}

impl RPSAction {

    fn new_from_letter<S>(letter: S) -> Self where S: std::fmt::Debug + AsRef<str> {
        match letter.as_ref().to_lowercase().as_str() {
            "a" => RPSAction::Rock,
            "b" => RPSAction::Paper,
            "c" => RPSAction::Scissors,
            "x" => RPSAction::Rock,
            "y" => RPSAction::Paper,
            "z" => RPSAction::Scissors,
            _ => panic!("Letter {:?} has no associated RPSAction", letter)
        }
    }

    fn id(&self) -> i8 {
        match self {
            RPSAction::Rock => 0,
            RPSAction::Paper => 1,
            RPSAction::Scissors => 2, 
        }
    }

    fn new_from_id(id: i8) -> Self {
        match id {
            0 => RPSAction::Rock,
            1 => RPSAction::Paper,
            2 => RPSAction::Scissors,
            _ => panic!("id {:?} has no associated RPSAction", id)
        }
    }

    fn beats(&self, other: &Self) -> RPSResult {
        // plus three to keep the difference positive.
        let diff = (self.id() - other.id() + 3) % 3;
        match diff {
            // 2 = -1 mod 3 -> self is before other in (rock paper scissors) -> self loses
            2 => RPSResult::Loss, 
            // 1 = 1 mod 3 -> self is after other in (rock paper scissors) -> self wins
            1 => RPSResult::Win,
            // obvious
            0 => RPSResult::Tie,
            _ => panic!("{:?} is not 0 1 or 2.", diff)
        }
    }

    fn action_that_would(&self, result: &RPSResult) -> Self {
        let id_modifier = match result {
            RPSResult::Win => 1, // action came after ours
            RPSResult::Tie => 0, // action same as ours
            RPSResult::Loss => -1, // action came before ours
        };
        Self::new_from_id((self.id() + id_modifier + 3) % 3)
    }

    fn score(&self) -> i32 {
        match self {
            RPSAction::Rock => 1,
            RPSAction::Paper => 2,
            RPSAction::Scissors => 3, 
        }
    }

    fn score_against(&self, other: &Self) -> i32 {
        self.beats(other).score() + self.score()
    }
}

fn read_rps_rounds_part_1<P: AsRef<Path>>(path: P) -> Vec<(RPSAction, RPSAction)> {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();
    let rounds = lines
        .filter(|line| line.is_ok()) 
        .map(|line| line.expect("already filtered out errors"))
        .map(|line| {
            let mut characters = line.split(' ');
            let opponent_action= RPSAction::new_from_letter(characters.next().unwrap());
            let our_action= RPSAction::new_from_letter(characters.next().unwrap());
            (opponent_action, our_action)
        })
        .collect();
    rounds
}

fn read_rps_rounds_part_2<P: AsRef<Path>>(path: P) -> Vec<(RPSAction, RPSAction)> {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();
    let rounds = lines
        .filter(|line| line.is_ok()) 
        .map(|line| line.expect("already filtered out errors"))
        .map(|line| {
            let mut characters = line.split(' ');
            let opponent_action= RPSAction::new_from_letter(characters.next().unwrap());
            let round_result = RPSResult::new_from_letter(characters.next().unwrap());
            let our_action = opponent_action.action_that_would(&round_result);
            (opponent_action, our_action)
        })
        .collect();
    rounds
}

pub fn answer_part_1<P: AsRef<Path>>(path: P) {
    let rounds = read_rps_rounds_part_1(path);
    let total_score = rounds.iter()
        .map(|(opponent_action, our_action)| our_action.score_against(opponent_action))
        .reduce(|a, b| a + b);
    println!("Part 1 total score: {:?}", total_score);
}

pub fn answer_part_2<P: AsRef<Path>>(path: P) {
    let rounds = read_rps_rounds_part_2(path);
    let total_score = rounds.iter()
        .map(|(opponent_action, our_action)| our_action.score_against(opponent_action))
        .reduce(|a, b| a + b);
    println!("Part 2 total score: {:?}", total_score);
}