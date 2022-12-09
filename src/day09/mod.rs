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
    head: Point,
    tail: Point,
    segments: Vec<Point>
}

impl Snake {

    fn new() -> Self {
        Self { head: (0, 0), tail: (0, 0)}
    }

    fn move_one(&mut self, direction: &Direction) {
        self.head = self.head.add(direction.displacement());
        self.move_tail();
    }

    fn move_tail(&mut self) {
        let disp_to_head = self.tail.displacement_to(self.head);
        if !disp_to_head.is_adjacent() {
            self.tail = self.tail.add(disp_to_head.normalize());
        }
    }
}

pub fn answer_part_1<P: AsRef<Path>>(path: P) -> String {
    let lines = utils::read_input(path);
    let mut snake = Snake::new();
    let mut set: HashSet<Point> = HashSet::new();
    for line in lines {
        let mut parts = line.split(" ");
        let direction = Direction::char_to_direction(parts.next().unwrap().chars().next().unwrap());
        let amount = utils::parse_int(parts.next().unwrap()).unwrap();
        for _ in 0..amount {
            snake.move_one(&direction);
            set.insert(snake.tail.clone());
        }
    }
    set.len().to_string()
}

pub fn answer_part_2<P: AsRef<Path>>(path: P) -> String {
    let mut lines = utils::read_input(path);
    "n/a".to_string()
}