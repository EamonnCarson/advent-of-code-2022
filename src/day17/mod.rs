use std::{path::Path, collections::HashMap};

use crate::utils;

#[derive(Clone, Copy, Debug)]
enum JetDir {
    Left,
    Right,
}

type Point = (i32, i32);

impl From<char> for JetDir {
    fn from(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("{} is not a jet direction", c),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum BlockType {
    Minus,
    Plus,
    Wedge,
    Pipe,
    Square,
}

const MINUS_POINTS: &[Point] = &[(0,0), (1,0), (2,0), (3,0)];
const PLUS_POINTS: &[Point] = &[(1,0), (0,-1), (1,-1), (2,-1), (1,-2)];
const WEDGE_POINTS: &[Point] = &[(2,0), (2,-1), (2,-2), (1,-2), (0,-2)];
const PIPE_POINTS: &[Point] = &[(0,0), (0,-1), (0,-2), (0,-3)];
const BLOCK_POINTS: &[Point] = &[(0,0), (0,-1), (1,0), (1,-1)];

#[derive(Clone, Debug)]
struct Block {
    block_type: BlockType,
    pos: Point, // upper left corner
}

impl Block {

    fn spawn_block(block_type: BlockType, floor: &Floor, spawn_height: i32, spawn_x: i32) -> Block {
        let mut block = Block {
            block_type,
            pos: (spawn_x, 0),
        };
        let ypos = floor.height as i32 + block.height() + spawn_height;
        block.pos = (spawn_x, ypos);
        block
    }

    fn relative_body_points(&self) -> &[Point] {
        match self.block_type {
            BlockType::Minus => &MINUS_POINTS,
            BlockType::Plus => &PLUS_POINTS,
            BlockType::Wedge => &WEDGE_POINTS,
            BlockType::Pipe => &PIPE_POINTS,
            BlockType::Square => &BLOCK_POINTS,
        }
    }

    fn right_x(&self) -> i32 {
        self.pos.0 + self.width() - 1
    }

    fn bottom_y(&self) -> i32 {
        self.pos.0 - self.height() + 1
    }

    fn height(&self) -> i32 {
        match self.block_type {
            BlockType::Minus => 1,
            BlockType::Plus => 3,
            BlockType::Wedge => 3,
            BlockType::Pipe => 4,
            BlockType::Square => 2,
        }
    }

    fn width(&self) -> i32 {
        match self.block_type {
            BlockType::Minus => 4,
            BlockType::Plus => 3,
            BlockType::Wedge => 3,
            BlockType::Pipe => 1,
            BlockType::Square => 2,
        }
    }

    /// Move block the way the wind blows.
    /// returns true if the movement succeeded
    fn move_block_sideways(&mut self, direction: JetDir, floor: &Floor) -> bool {
        let offset = match direction {
            JetDir::Left => (-1, 0),
            JetDir::Right => (1, 0),
        };
        let new_pos = (self.pos.0 + offset.0, self.pos.1 + offset.1);
        let old_pos = self.pos;
        if new_pos.0 >= 0 && new_pos.0 + self.width() - 1 < floor.width.try_into().unwrap() {
            self.pos = new_pos;
            if self.intersecting_floor(floor) {
                self.pos = old_pos;
                return false;
            } else {
                return true;
            }
        } else {
            return false;
        }
    }

    fn move_block_down(&mut self, floor: &Floor) -> bool {
        let new_pos = (self.pos.0, self.pos.1 - 1);
        let old_pos = self.pos;
        self.pos = new_pos;
        if self.intersecting_floor(floor) {
            self.pos = old_pos;
            return false;
        } else {
            return true;
        }
    }

    fn body_points(&self) -> impl Iterator<Item = Point> + '_ {
        self.relative_body_points()
            .iter()
            .map(|offset| (self.pos.0 + offset.0, self.pos.1 + offset.1))
    }

    fn intersecting_floor(&self, floor: &Floor) -> bool {
        self.body_points()
            .any(|point|
                floor.x2heights
                    .get(&point.0)
                    .expect(&format!("block {:?} (point {:?}) is out of bounds!", self, point)) >= &point.1
            )
    }

    fn update_floor(&self, floor: &mut Floor) {
        for point in self.body_points() {
            floor.update(point.0, point.1);
        }
    }
}

#[derive(Clone, Debug)]
struct Floor {
    x2heights: HashMap<i32, i32>,
    width: usize,
    height: usize, // max height of floor
    rows: Vec<String>,
}

impl Floor {
    fn new(width: usize) -> Floor {
        let mut x2heights: HashMap<i32, i32> = HashMap::new();
        for x in 0..width {
            x2heights.insert(x as i32, 0);
        }
        Floor {
            x2heights,
            width,
            height: 0,
            rows: vec!["-".repeat(width)],
        }
    }

    fn print_top_floors(&self, count: Option<usize>) {
        let top_floor_y = self.rows.len();
        let limit = match count {
            Some(i) => 0..i,
            None => 0..top_floor_y,
        };
        for (floor, i) in self.rows.iter().rev().zip(limit) {
            println!("{} {}", top_floor_y - i, floor);
        }
    }

    fn update(&mut self, x: i32, y: i32) {
        let floor_y = self.x2heights
            .get_mut(&x)
            .expect("block is out of bounds!");
        if y > *floor_y {
            *floor_y = y;
            self.height = self.height.max(y.try_into().unwrap());
        }
    }
}

pub fn answer_part_1<P: AsRef<Path>>(path: P) -> String {
    let lines = utils::read_input(path);
    let lines: Vec<String> = lines.collect();
    let mut jet_dir_cycle = lines
        .iter()
        .flat_map(|line| line.chars())
        .map(|char| JetDir::from(char))
        .cycle();
    let block_types = vec![BlockType::Minus, BlockType::Plus, BlockType::Wedge, BlockType::Pipe, BlockType::Square];
    let mut block_type_cycle = block_types.iter().cycle();
    let mut floor = Floor::new(7);
    for _ in 0..2022 {
        let block_type = *block_type_cycle.next().unwrap();
        let mut block = Block::spawn_block(block_type, &floor, 3, 2);
        //println!("Spawned block {:?}", block);
        loop {
            let direction = jet_dir_cycle.next().unwrap();
            block.move_block_sideways(direction, &floor);
            //println!("   block moved over to {:?}", block.pos);
            let floor_not_below = block.move_block_down(&floor);
            if !floor_not_below { break; }
            //println!("   block moved down to {:?}", block.pos);
        }
        block.update_floor(&mut floor);
        println!("Placed block {:?}. New floor: {:?}", block, floor);
    }
    floor.print_top_floors(None);
    floor.height.to_string()
}

pub fn answer_part_2<P: AsRef<Path>>(path: P) -> String {
    let mut lines = utils::read_input(path);
    "n/a".to_string()
}