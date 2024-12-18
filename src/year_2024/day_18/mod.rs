use std::{collections::VecDeque, path::Path};

use rayon::prelude::*;

use crate::Answer;

const GRID_SIZE: usize = 70 + 1;
const POPULATE_BYTES: usize = 1024;

#[derive(Debug)]
struct Coordinate {
    row: usize,
    col: usize,
}

fn get_coordinates(input: &str) -> Vec<Coordinate> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            let row: usize = y.parse().unwrap();
            let col: usize = x.parse().unwrap();
            Coordinate { row, col }
        })
        .collect()
}

fn populate_grid(coordinates: &[Coordinate], count: usize) -> Vec<Vec<i32>> {
    let mut grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    for coordinate in coordinates.iter().take(count) {
        grid[coordinate.row][coordinate.col] = 1;
    }
    grid
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<i32>]) {
    for row in grid.iter() {
        for cell in row.iter() {
            print!("{cell}");
        }
        println!();
    }
}

fn find_path(grid: &[Vec<i32>]) -> i32 {
    let mut q = VecDeque::new();
    q.push_back((0, 0, 0));
    let mut visited = vec![vec![false; GRID_SIZE]; GRID_SIZE];
    visited[0][0] = true;
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    while let Some((row, col, steps)) = q.pop_front() {
        if row == GRID_SIZE - 1 && col == GRID_SIZE - 1 {
            return steps;
        }
        for (dx, dy) in dirs.iter() {
            let nrow = row.overflowing_add_signed(*dx).0;
            let ncol = col.overflowing_add_signed(*dy).0;
            if nrow >= GRID_SIZE || ncol >= GRID_SIZE {
                continue;
            }
            if grid[nrow][ncol] == 1 {
                continue;
            }
            if visited[nrow][ncol] {
                continue;
            }
            visited[nrow][ncol] = true;
            q.push_back((nrow, ncol, steps + 1));
        }
    }
    i32::MAX
}

fn part_1(input: &str) -> Option<i32> {
    let coordinates = get_coordinates(input);
    let grid = populate_grid(&coordinates, POPULATE_BYTES);
    let path = find_path(&grid);
    path.into()
}

fn part_2(input: &str) -> Option<i32> {
    let coordinates = get_coordinates(input);
    let first_block_i = (1..=coordinates.len())
        .into_par_iter()
        .find_first(|populate_bytes| {
            let grid = populate_grid(&coordinates, *populate_bytes);
            let path = find_path(&grid);
            path == i32::MAX
        })
        .unwrap();
    let blocking_byte = &coordinates[first_block_i - 1];
    println!("Blocking byte: {blocking_byte:?}");
    (first_block_i as i32).into()
}
pub fn solve() -> Answer {
    let cur_dir = Path::new(file!()).parent().unwrap();
    let file_path = cur_dir.join("input.txt");
    let input = std::fs::read_to_string(file_path).unwrap();
    let part_1_start_time = std::time::Instant::now();
    let part_1 = part_1(&input);
    let part_1_elapsed_time = part_1_start_time.elapsed();
    println!("Done with part 1 in {:?}", part_1_elapsed_time);
    let part_2_start_time = std::time::Instant::now();
    let part_2 = part_2(&input);
    let part_2_elapsed_time = part_2_start_time.elapsed();
    println!("Done with part 2 in {:?}", part_2_elapsed_time);
    Answer::from_parts(part_1, part_2)
}
