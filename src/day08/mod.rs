use std::{path::Path, fmt::{Display, Debug}};

use crate::utils::{self, parse_int};

type Tree = i32;

struct Grid<T> {
    data: Vec<T>,
    height: usize,
    width: usize,
}

enum Direction {
    North,
    South,
    East,
    West,
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

    fn new_from_columns(data: Vec<Vec<T>>) -> Self {
        let width = data.len();
        let height = data
            .get(0)
            .and_then(|x| Some(x.len()))
            .expect("data isn't empty");
        let grid = Self::new_from_rows(data);
        let data: Vec<T> = grid.data
            .into_iter()
            .collect();
        todo!();
        Self {height, width, data}
    }

    fn transpose(&mut self) {
        let height = self.width;
        let width = self.height;
        let mut data = self.data.as_slice();
        todo!();
    }

    fn get_ix(&self, row: usize, col: usize) -> usize {
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

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        let ix = self.get_ix(row, col);
        &mut self.data[ix]
    }

    fn indices_from_direction(&self, direction: Direction) -> impl Iterator<Item = (usize, usize)> {
        // what the heck
        // why is the rust standard library incapable of making this stuff simple
        let width = self.width;
        let rows = match direction {
            Direction::South => Box::new((0..self.height).rev()) as Box<dyn Iterator<Item = usize>>,
            _ => Box::new(0..self.height) as Box<dyn Iterator<Item = usize>>,
        };
        rows.flat_map(move |x| std::iter::repeat(x).zip(
            match direction {
                Direction::West => Box::new((0..width).rev()) as Box<dyn Iterator<Item = usize>>,
                _ => Box::new(0..width) as Box<dyn Iterator<Item = usize>>,
            }
        ))
    }

    fn iter_row<'a>(&'a self, row: usize) -> GridSliceIter<'a, T> {
        match row < self.height {
            true => GridSliceIter { 
                grid: &self, 
                front_ix: 0, 
                back_ix: self.width, 
                constant_ix: row,
                direction: GridDirection::Row,
            },
            false => panic!("row out of range of grid"),
        } 
    }

    fn iter_rows<'a>(&'a self) -> impl DoubleEndedIterator<Item = GridSliceIter<'a, T>> {
        (0..self.height)
            .map(|row| self.iter_row(row))
    }

    fn iter_col<'a>(&'a self, col: usize) -> GridSliceIter<'a, T> {
        match col < self.width {
            true => GridSliceIter { 
                grid: &self, 
                front_ix: 0, 
                back_ix: self.height, 
                constant_ix: col,
                direction: GridDirection::Column,
            },
            false => panic!("row out of range of grid"),
        } 
    }

    fn iter_cols<'a>(&'a self) -> impl DoubleEndedIterator<Item = GridSliceIter<'a, T>> {
        (0..self.width)
            .map(|col| self.iter_col(col))
    }

}

enum GridDirection { Row, Column }

struct GridSliceIter<'a, T> {
    grid: &'a Grid<T>,
    front_ix: usize,
    back_ix: usize, // for double ended iteration
    constant_ix: usize,
    direction: GridDirection,
}

impl<'a, T> Iterator for GridSliceIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front_ix == self.back_ix {
            None
        } else {
            let (row, col) = match self.direction {
                GridDirection::Row => (self.front_ix, self.constant_ix),
                GridDirection::Column => (self.constant_ix, self.front_ix),
            };
            self.front_ix += 1;
            Some(self.grid.get(row, col))
        }
    }
}

impl<'a, T> DoubleEndedIterator for GridSliceIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front_ix == self.back_ix {
            None
        } else {
            let (row, col) = match self.direction {
                GridDirection::Row => (self.back_ix, self.constant_ix),
                GridDirection::Column => (self.constant_ix, self.back_ix),
            };
            self.back_ix -= 1;
            Some(self.grid.get(row, col))
        }
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            let start = row * self.width;
            let end = start + self.width;
            f.write_str(&format!("{:#?}\n", &self.data[start..end]))?;
        }
        Ok(())
    }
}

type VisibilityGrid = Grid<bool>;
type Forest = Grid<Tree>;

impl VisibilityGrid {
    fn new_visibility_grid(height: usize, width: usize) -> Self {
        Self::new_from_rows(
            std::iter::repeat(false).take(height)
                .map(|_| std::iter::repeat(false)
                    .take(width)
                    .collect()
                )
                .collect()
        )
    }

    fn num_true(&self) -> i32 {
        self.data
            .iter()
            .map(|b| match b {true => 1, false => 0})
            .sum()
    }
}

impl Forest {
    fn parse_forest(lines: impl Iterator<Item = String>) -> Forest {
        let data: Vec<Vec<Tree>> = lines
            .map(|line| {
                let vec: Vec<Tree> = line
                    .trim()
                    .chars()
                    .map(|c| c.to_string())
                    .map(parse_int)
                    .map(|maybe_int| maybe_int.unwrap())
                    .collect();
                vec
            })
            .collect();
        Forest::new_from_rows(data)
    }

    fn update_visible(&self, from_direction: Direction, visibility_grid: &mut VisibilityGrid) {
        let prev_col = self.width + 1; // some impossible col number
        let mut highest_tree_yet = &-1;


        for row in 0..self.height {
            let visible_forward = get_directional_visible_trees(self.iter_row(row));
            let visible_backward = get_directional_visible_trees(self.iter_row(row).rev());
            let visible = visible_forward
                .zip(visible_backward.rev())
                .map(|(v1, v2)| v1 || v2);
            let row_iter = self.iter_row(row);
        }
    }

    fn visible_trees(&self) -> VisibilityGrid {
        let mut visible = VisibilityGrid::new_visibility_grid(self.height, self.width);
        self.update_visible(Direction::North, &mut visible);
        self.update_visible(Direction::South, &mut visible);
        self.update_visible(Direction::East, &mut visible);
        self.update_visible(Direction::West, &mut visible);
        visible
    }

}

fn get_directional_visible_trees<'a, I>(trees: I) -> impl DoubleEndedIterator<Item = bool> + 'a
    where I: DoubleEndedIterator<Item = &'a Tree> + 'a {
    let mut max_tree_so_far: &Tree = &-1;
    return trees
        .into_iter()
        .map(move |tree| {
            max_tree_so_far = max_tree_so_far.max(tree);
            tree > max_tree_so_far
        });
}

pub fn answer_part_1<P: AsRef<Path>>(path: P) -> String {
    let mut lines = utils::read_input(path);
    let forest = Forest::parse_forest(lines);
    let visible = forest.visible_trees();
    visible.num_true().to_string()
}

pub fn answer_part_2<P: AsRef<Path>>(path: P) -> String {
    let mut lines = utils::read_input(path);
    "n/a".to_string()
}

