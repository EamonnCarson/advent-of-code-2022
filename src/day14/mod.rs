use std::{path::Path, iter, fmt::{Display, Write}};

use crate::utils::{self, parse_usize, grid::Grid};

type Point = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq)]
enum SandState {
    Rock,
    Sand,
    Air,
}

impl Display for SandState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SandState::Rock => f.write_char('#'),
            SandState::Sand => f.write_char('o'),
            SandState::Air => f.write_char('.')
        }?;
        Ok(())
    }
}

fn parse_input(lines: impl Iterator<Item = String>) -> Vec<Vec<Point>> {
    let points = lines.map(|line| line.split(" -> ").map(|point_str| {
        let (x, y) = point_str.split_once(",").unwrap();
        return (parse_usize(x).unwrap(), parse_usize(y).unwrap())
    }).collect())
    .collect();
    points
}

fn draw_rocks(point_lists: Vec<Vec<Point>>, grid: &mut Grid<SandState>) {
    for point_list in point_lists {
        let mut prev_point: Option<Point> = None;
        for point in point_list {
            if let Some(prev_point) = prev_point {
                if point.0 == prev_point.0 {
                    let start_y = point.1.min(prev_point.1);
                    let end_y = point.1.max(prev_point.1);
                    (start_y..end_y+1)
                        .map(|y| (prev_point.0, y))
                        .map(|point| {
                            let grid_entry = grid.get_xy_point_mut(point);
                            *grid_entry = SandState::Rock;
                        })
                        .count();
                } else {
                    let start_x = point.0.min(prev_point.0);
                    let end_x = point.0.max(prev_point.0);
                    (start_x..end_x+1)
                        .map(|x| (x, prev_point.1))
                        .map(|point| {
                            let grid_entry = grid.get_xy_point_mut(point);
                            *grid_entry = SandState::Rock;
                        })
                        .count();
                }
            }
            prev_point = Some(point);
        }
    }
}

fn simulate_sand(start_point: Point, grid: &mut Grid<SandState>) -> usize {
    let mut num_sand_placed = 0;
    let mut path_stack: Vec<Point> = vec![start_point];
    while !path_stack.is_empty() {
        while let Some(point) = path_stack.last() {
            if !grid.xy_point_is_inbounds(point) {
                return num_sand_placed;
            } else {
                let point_below = (point.0, point.1 + 1);
                let point_below_left = (point.0.wrapping_sub(1), point.1 + 1);
                let point_below_right = (point.0.wrapping_add(1), point.1 + 1);
                if !grid.xy_point_is_inbounds(&point_below) || grid.get_xy_point(point_below) == &SandState::Air {
                    path_stack.push(point_below);
                } else if !grid.xy_point_is_inbounds(&point_below_left) || grid.get_xy_point(point_below_left) == &SandState::Air {
                    path_stack.push(point_below_left);
                } else if !grid.xy_point_is_inbounds(&point_below_right) || grid.get_xy_point(point_below_right) == &SandState::Air {
                    path_stack.push(point_below_right);
                } else {
                    // all options are blocked, so place sand here and pop the stack back one
                    *grid.get_xy_point_mut(*point) = SandState::Sand;
                    num_sand_placed += 1;
                    if point == &start_point {
                        return num_sand_placed;
                    }
                    path_stack.pop();
                }
            }
        }
    }
    return num_sand_placed;
}

pub fn answer_part_1<P: AsRef<Path>>(path: P) -> String {
    let lines = utils::read_input(path);
    let point_lists = parse_input(lines);
    let max_x = point_lists.iter()
        .flat_map(|line_points| line_points.iter())
        .map(|point| point.0)
        .max()
        .unwrap();
    let max_y = point_lists.iter()
        .flat_map(|line_points| line_points.iter())
        .map(|point| point.1)
        .max()
        .unwrap();
    let rows: Vec<Vec<SandState>> = (0..max_y+1).map(
            |_| (0..max_x+1)
                .zip(iter::repeat(SandState::Air))
                .map(|(_, state)| state)
                .collect()
        ).collect();
    let mut grid = Grid::new_from_rows(rows);
    draw_rocks(point_lists, &mut grid);
    let num_sand_placed = simulate_sand((500, 0), &mut grid);
    num_sand_placed.to_string()
}

pub fn answer_part_2<P: AsRef<Path>>(path: P) -> String {
    // Issue: this can give the wrong answer because the infinite wall at the bottom is not infinite
    // This would be best solved offsetting all of the coordinates so it'd be guaranteed
    // that the grid could simulator far enough
    // But I don't want to take the time to do that.
    let lines = utils::read_input(path);
    let mut point_lists = parse_input(lines);
    let max_x = point_lists.iter()
        .flat_map(|line_points| line_points.iter())
        .map(|point| point.0)
        .max()
        .unwrap();
    let max_y = point_lists.iter()
        .flat_map(|line_points| line_points.iter())
        .map(|point| point.1)
        .max()
        .unwrap();
    point_lists.push(vec![(0, max_y + 2), (max_x + max_y + 2, max_y + 2)]);
    let max_y = max_y + 2;
    let max_x = max_x + max_y + 2;
    let rows: Vec<Vec<SandState>> = (0..max_y+1).map(
            |_| (0..max_x+1)
                .zip(iter::repeat(SandState::Air))
                .map(|(_, state)| state)
                .collect()
        ).collect();
    let mut grid = Grid::new_from_rows(rows);
    draw_rocks(point_lists, &mut grid);
    let num_sand_placed = simulate_sand((500, 0), &mut grid);
    num_sand_placed.to_string()
}