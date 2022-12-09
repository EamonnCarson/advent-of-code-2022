use std::{path::Path, collections::HashSet};

use crate::utils;


type Point = (i32, i32);
type Displacement = (i32, i32);

trait PointLike {
    fn displacement_to(&self, other: Point) -> Displacement;
    fn add(&self, displacement: Displacement) -> Point;
}

trait DisplacementLike {
    fn is_adjacent(&self) -> bool;
    fn is_cardinal(&self) -> bool;
    fn normalize(&self) -> Displacement;
    fn is_zero(&self) -> bool;
}

impl PointLike for Point {
    fn displacement_to(&self, other: Point) -> Displacement {
        (other.0 - self.0, other.1 - self.1)
    }

    fn add(&self, displacement: Displacement) -> Point {
        (self.0 + displacement.0, self.1 + displacement.1)
    }
}

impl DisplacementLike for Displacement {
    /// tells you if the displacement is between adjacent coordinates
    fn is_adjacent(&self) -> bool {
        self.0.abs().max(self.1.abs()) <= 1
    }

    fn is_cardinal(&self) -> bool {
        self.0 == 0 || self.1 == 0
    }

    // not actually normalizing, but kind of like it
    fn normalize(&self) -> Displacement {
        (self.0 / self.0.abs().max(1), self.1 / self.1.abs().max(1))
    }

    fn is_zero(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }
}

enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    fn char_to_direction(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("invalid direction {:?}", c),
        }
    }

    fn displacement(&self) -> Displacement {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

struct Snake {
    segments: Vec<Point>
}

impl Snake {

    fn new(length: usize) -> Self {
        assert!(length >= 2);
        let mut segments = Vec::new();
        for i in 0..length {
            segments.push((0, 0));
        }
        Self { segments: segments }
    }

    fn move_one(&mut self, direction: &Direction) {
        let head = self.segments[0];
        self.segments[0] = head.add(direction.displacement());
        self.move_tails();
    }

    fn move_tails(&mut self) {
        let mut prev = None;
        for i in 0..self.segments.len() {
            let mut curr = self.segments[i];
            match prev {
                Some(prev) => {
                    let disp_to_prev = curr.displacement_to(prev);
                    if !disp_to_prev.is_adjacent() {
                        curr = curr.add(disp_to_prev.normalize());
                        self.segments[i] = curr;
                    }
                },
                None => {},
            }
            prev = Some(curr);
        }
    }
}

pub fn answer_part_1<P: AsRef<Path>>(path: P) -> String {
    let lines = utils::read_input(path);
    let mut snake = Snake::new(2);
    let mut set: HashSet<Point> = HashSet::new();
    for line in lines {
        let mut parts = line.split(" ");
        let direction = Direction::char_to_direction(parts.next().unwrap().chars().next().unwrap());
        let amount = utils::parse_int(parts.next().unwrap()).unwrap();
        for _ in 0..amount {
            snake.move_one(&direction);
            set.insert(snake.segments.last().unwrap().clone());
        }
    }
    set.len().to_string()
}

pub fn answer_part_2<P: AsRef<Path>>(path: P) -> String {
    let lines = utils::read_input(path);
    let mut snake = Snake::new(10);
    let mut set: HashSet<Point> = HashSet::new();
    for line in lines {
        let mut parts = line.split(" ");
        let direction = Direction::char_to_direction(parts.next().unwrap().chars().next().unwrap());
        let amount = utils::parse_int(parts.next().unwrap()).unwrap();
        for _ in 0..amount {
            snake.move_one(&direction);
            set.insert(snake.segments.last().unwrap().clone());
        }
    }
    set.len().to_string()
}