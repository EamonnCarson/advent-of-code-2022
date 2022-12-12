use std::{path::Path, iter::repeat, collections::BinaryHeap};

use crate::utils;

type Point = (usize, usize);

#[derive(Debug)]
struct Grid<T> {
    data: Vec<T>,
    height: usize,
    width: usize,
}

impl<T> Grid<T> {
    fn new_from_rows(data: Vec<Vec<T>>) -> Self {
        let height = data.len();
        let width = data
            .get(0)
            .and_then(|x| Some(x.len()))
            .expect("data isn't empty");
        let data: Vec<T> = data
            .into_iter()
            .flat_map(|vec| vec.into_iter())
            .collect();
        Self {height, width, data}
    }

    fn check_row_col(&self, row: usize, col: usize) {
        match (row < self.height, col < self.width) {
            (true, true) => {},
            (true, false) => panic!("col {} out of bounds {}", col, self.width),
            (false, true) => panic!("row {} out of bounds {}", row, self.height),
            (false, false) => panic!("Both row {} and col {} out of bounds {} and {}", row, col, self.height, self.width),
        }
    }

    fn get_ix(&self, row: usize, col: usize) -> usize {
        self.check_row_col(row, col);
        col + row * self.width
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        let ix = self.get_ix(row, col);
        self.data[ix] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        let ix = self.get_ix(row, col);
        &self.data[ix]
    }

    pub fn get_point(&self, point: Point) -> &T {
        let ix = self.get_ix(point.1, point.0);
        &self.data[ix]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        let ix = self.get_ix(row, col);
        &mut self.data[ix]
    }

    pub fn get_point_mut(&mut self, point: Point) -> &mut T {
        let ix = self.get_ix(point.1, point.0);
        &mut self.data[ix]
    }

}

fn parse_grid(lines: impl Iterator<Item = String>) -> (Grid<BFSEntry>, Point, Point) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut nrows = 0;
    let mut ncols = 0;
    let data: Vec<BFSEntry> = lines
        .enumerate()
        .flat_map(|(row, line)| line.chars().zip(repeat(row)).enumerate().collect::<Vec<_>>())
        .map(|(col, (char, row))| (char, row, col))
        .map(|(char, row, col)| {
            let height = match char {
                'a' => (0, false),
                'b' => (1, false),
                'c' => (2, false),
                'd' => (3, false),
                'e' => (4, false),
                'f' => (5, false),
                'g' => (6, false),
                'h' => (7, false),
                'i' => (8, false),
                'j' => (9, false),
                'k' => (10, false),
                'l' => (11, false),
                'm' => (12, false),
                'n' => (13, false),
                'o' => (14, false),
                'p' => (15, false),
                'q' => (16, false),
                'r' => (17, false),
                's' => (18, false),
                't' => (19, false),
                'u' => (20, false),
                'v' => (21, false),
                'w' => (22, false),
                'x' => (23, false),
                'y' => (24, false),
                'z' => (25, false),
                'S' => {
                    start = (col, row);
                    (0, true)
                },
                'E' => {
                    end = (col, row);
                    (25, true)
                },
                _ => panic!("invalid char"),
            };
            nrows = nrows.max(row + 1);
            ncols = ncols.max(col + 1);
            return BFSEntry { height: height.0, distance: i32::MAX };
        })
        .collect();
    let grid = Grid {
        data,
        height: nrows,
        width: ncols,
    };
    (grid, start, end)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum GridDirections { 
    N, S, E, W
}

impl GridDirections {
    fn offset(&self) -> (i32, i32) {
        match self {
            GridDirections::N => (0, 1),
            GridDirections::S => (0, -1),
            GridDirections::E => (1, 0),
            GridDirections::W => (-1, 0),
        }
    }

    fn from_point<T>(&self, point: &Point, grid: &Grid<T>) -> Option<Point> {
        let offset  = self.offset();
        let new_point = (point.0 as i32 + offset.0, point.1 as i32 + offset.1);
        if new_point.0 < 0 || new_point.0 >= grid.width.try_into().unwrap() || new_point.1 < 0 || new_point.1 >= grid.height.try_into().unwrap() {
            return None;
        } else {
            return Some((new_point.0 as usize, new_point.1 as usize));
        }
    }

    fn iter() -> impl Iterator<Item = GridDirections> {
        vec![Self::N, Self::S, Self::E, Self::W].into_iter()
    }
}

#[derive(Debug)]
struct BFSEntry {
    height: i32,
    distance: i32,
}

fn run_breadth_first_search2(start: Point, end: Point, grid: &mut Grid<BFSEntry>) -> i32 {
    let mut priority_queue: BinaryHeap<(i32, Point)> = BinaryHeap::new();
    priority_queue.push((0, start));
    while !priority_queue.is_empty() {
        let next = priority_queue.pop().unwrap();
        //println!("Processing {:?}", next);
        let (dist, point) = next;
        let dist = -dist; // we reversed so we got lowest dist first in queue
        // check to see if we update the node
        let bfs_entry_distance = grid.get_point(point).distance;
        if bfs_entry_distance > dist {
            let bfs_entry = grid.get_point_mut(point);
            bfs_entry.distance = dist;
            let bfs_entry = grid.get_point(point);
            for dir in GridDirections::iter() {
                let next_point = dir.from_point(&point, &grid);
                if let Some(next_point) = next_point {
                    let next_bfs_entry = grid.get_point(next_point);
                    if bfs_entry.height + 1 >= next_bfs_entry.height {
                        priority_queue.push((-(dist + 1), next_point))
                    }
                }
            }
        }
    };
    return grid.get_point(end).distance;
}

fn run_breadth_first_search_reverse(start: Point, end: Point, grid: &mut Grid<BFSEntry>) -> i32 {
    let mut priority_queue: BinaryHeap<(i32, Point)> = BinaryHeap::new();
    priority_queue.push((0, start));
    while !priority_queue.is_empty() {
        let next = priority_queue.pop().unwrap();
        //println!("Processing {:?}", next);
        let (dist, point) = next;
        let dist = -dist; // we reversed so we got lowest dist first in queue
        // check to see if we update the node
        let bfs_entry_distance = grid.get_point(point).distance;
        if bfs_entry_distance > dist {
            let bfs_entry = grid.get_point_mut(point);
            bfs_entry.distance = dist;
            let bfs_entry = grid.get_point(point);
            for dir in GridDirections::iter() {
                let next_point = dir.from_point(&point, &grid);
                if let Some(next_point) = next_point {
                    let next_bfs_entry = grid.get_point(next_point);
                    if bfs_entry.height - 1 <= next_bfs_entry.height {
                        if next_bfs_entry.height == 23 && dist <= 1 {
                            println!("Making a connection from {:?} to {:?}", bfs_entry, next_bfs_entry);
                        }
                        priority_queue.push((-(dist + 1), next_point))
                    }
                }
            }
        }
    };
    return grid.get_point(end).distance;
}

pub fn answer_part_1<P: AsRef<Path>>(path: P) -> String {
    let lines = utils::read_input(path);
    let (mut grid, start, end) = parse_grid(lines);
    //println!("{:?}, {:?}, {:?}", grid, start, end);
    let answer = run_breadth_first_search2(start, end, &mut grid);
    answer.to_string()
}

pub fn answer_part_2<P: AsRef<Path>>(path: P) -> String {
    let lines = utils::read_input(path);
    let (mut grid, end, start) = parse_grid(lines);
    //println!("{:?}, {:?}, {:?}", grid, start, end);
    let answer = run_breadth_first_search_reverse(start, end, &mut grid);
    let min_distance = grid.data.iter()
        .filter(|entry| entry.height == 0)
        .min_by(|x, y| x.distance.cmp(&y.distance));
    println!("{:#?}", min_distance);
    let distances: Vec<i32> = grid.data.iter().map(|e| e.distance).collect();
    let heights: Vec<i32> = grid.data.iter().map(|e| e.height).collect();
    //println!("{:?}, {:?}, {:?}", distances, start, end);
    //println!("{:?}, {:?}, {:?}", heights, start, end);
    min_distance.unwrap().distance.to_string()
}