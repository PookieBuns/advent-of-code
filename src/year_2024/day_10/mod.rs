use std::{collections::HashSet, path::Path};

use crate::Answer;

fn convert_to_grid(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn find_trail(grid: &[Vec<i32>], row: usize, col: usize) -> HashSet<(usize, usize)> {
    if grid[row][col] == 9 {
        return HashSet::from_iter([(row, col)]);
    }
    let cur_h = grid[row][col];
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    dirs.iter().fold(HashSet::new(), |mut acc, &(dx, dy)| {
        let nrow = row as i32 + dx;
        if nrow < 0 {
            return acc;
        }
        let nrow = nrow as usize;
        let ncol = col as i32 + dy;
        if ncol < 0 {
            return acc;
        }
        let ncol = ncol as usize;
        if nrow >= grid.len() {
            return acc;
        }
        if ncol >= grid[nrow].len() {
            return acc;
        }
        let n_h = grid[nrow][ncol];
        if n_h - cur_h != 1 {
            return acc;
        }
        acc.extend(find_trail(grid, nrow, ncol));
        acc
    })
}

fn part_1(input: &str) -> Option<i32> {
    let grid = convert_to_grid(input);
    (0..grid.len())
        .flat_map(|i| (0..grid[i].len()).map(move |j| (i, j)))
        .filter(|&(i, j)| grid[i][j] == 0)
        .map(|(i, j)| find_trail(&grid, i, j).len() as i32)
        .sum::<i32>()
        .into()
}

fn find_trail_2(grid: &[Vec<i32>], row: usize, col: usize) -> i32 {
    if grid[row][col] == 9 {
        return 1;
    }
    let cur_h = grid[row][col];
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    dirs.iter()
        .map(|(dx, dy)| {
            let nrow = row as i32 + dx;
            if nrow < 0 {
                return 0;
            }
            let nrow = nrow as usize;
            let ncol = col as i32 + dy;
            if ncol < 0 {
                return 0;
            }
            let ncol = ncol as usize;
            if nrow >= grid.len() {
                return 0;
            }
            if ncol >= grid[nrow].len() {
                return 0;
            }
            let n_h = grid[nrow][ncol];
            if n_h - cur_h != 1 {
                return 0;
            }
            find_trail_2(grid, nrow, ncol)
        })
        .sum::<i32>()
}

fn part_2(input: &str) -> Option<i32> {
    let grid = convert_to_grid(input);
    (0..grid.len())
        .flat_map(|i| (0..grid[i].len()).map(move |j| (i, j)))
        .filter(|&(i, j)| grid[i][j] == 0)
        .map(|(i, j)| find_trail_2(&grid, i, j))
        .sum::<i32>()
        .into()
}
pub fn solve() -> Answer {
    let cur_dir = Path::new(file!()).parent().unwrap();
    let file_path = cur_dir.join("input.txt");
    let input = std::fs::read_to_string(file_path).unwrap();
    let part_1 = part_1(&input);
    let part_2 = part_2(&input);
    Answer::from_parts(part_1, part_2)
}
