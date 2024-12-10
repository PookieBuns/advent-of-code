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
        return HashSet::from_iter(vec![(row, col)].into_iter());
    }
    let cur_h = grid[row][col];
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut ans = HashSet::new();
    for (dx, dy) in dirs {
        let nrow = row as i32 + dx;
        let ncol = col as i32 + dy;
        if nrow < 0 {
            continue;
        }
        if nrow >= grid.len() as i32 {
            continue;
        }
        if ncol < 0 {
            continue;
        }
        if ncol >= grid[nrow as usize].len() as i32 {
            continue;
        }
        let n_h = grid[nrow as usize][ncol as usize];
        if n_h - cur_h != 1 {
            continue;
        }
        println!("{nrow},{ncol}");
        ans.extend(find_trail(&grid, nrow as usize, ncol as usize));
    }
    ans
}

fn part_1(input: &str) -> Option<i32> {
    let grid = convert_to_grid(input);
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                ans += find_trail(&grid, i, j).len();
            }
        }
    }
    (ans as i32).into()
}

fn find_trail_2(grid: &[Vec<i32>], row: usize, col: usize) -> i32 {
    if grid[row][col] == 9 {
        return 1;
    }
    let cur_h = grid[row][col];
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut ans = 0;
    for (dx, dy) in dirs {
        let nrow = row as i32 + dx;
        let ncol = col as i32 + dy;
        if nrow < 0 {
            continue;
        }
        if nrow >= grid.len() as i32 {
            continue;
        }
        if ncol < 0 {
            continue;
        }
        if ncol >= grid[nrow as usize].len() as i32 {
            continue;
        }
        let n_h = grid[nrow as usize][ncol as usize];
        if n_h - cur_h != 1 {
            continue;
        }
        ans += find_trail_2(&grid, nrow as usize, ncol as usize);
    }
    ans
}

fn part_2(input: &str) -> Option<i32> {
    let grid = convert_to_grid(input);
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                ans += find_trail_2(&grid, i, j)
            }
        }
    }
    (ans as i32).into()
}
pub fn solve() -> Answer {
    let cur_dir = Path::new(file!()).parent().unwrap();
    let file_path = cur_dir.join("input.txt");
    let input = std::fs::read_to_string(file_path).unwrap();
    let part_1 = part_1(&input);
    let part_2 = part_2(&input);
    Answer::from_parts(part_1, part_2)
}
