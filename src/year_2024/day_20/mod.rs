use std::{collections::VecDeque, path::Path};

use crate::Answer;

fn convert_to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_end(grid: &[Vec<char>]) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, c)| (*c == 'E').then_some((i, j)))
        })
        .unwrap()
}

fn djikstra(grid: &[Vec<char>]) -> Vec<Vec<i32>> {
    let (end_i, end_j) = find_end(grid);
    let (n, m) = (grid.len(), grid[0].len());
    let mut dist = vec![vec![i32::MAX; m]; n];
    let mut q = VecDeque::new();
    q.push_back((end_i, end_j, 0));
    dist[end_i][end_j] = 0;
    while let Some((i, j, d)) = q.pop_front() {
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (di, dj) in dirs {
            let ni = i.overflowing_add_signed(di).0;
            if ni >= n {
                continue;
            }
            let nj = j.overflowing_add_signed(dj).0;
            if nj >= m {
                continue;
            }
            let nd = d + 1;
            if grid[ni][nj] == '#' {
                continue;
            }
            if nd >= dist[ni][nj] {
                continue;
            }
            dist[ni][nj] = nd;
            q.push_back((ni, nj, nd));
        }
    }
    dist
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<i32>]) {
    for row in grid {
        for cell in row {
            if *cell == i32::MAX {
                print!("## ");
            } else {
                print!("{:02} ", cell);
            }
        }
        println!();
    }
}

fn find_cheats(grid: &[Vec<i32>], threshhold: i32) -> i32 {
    let mut cheats = 0;
    let (n, m) = (grid.len(), grid[0].len());
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            if grid[i][j] != i32::MAX {
                continue;
            }
            let above = grid[i - 1][j];
            let below = grid[i + 1][j];
            if above != i32::MAX && below != i32::MAX {
                let time_saved = (above - below).abs() - 2;
                if time_saved >= threshhold {
                    // println!("({}, {}): {}", i, j, time_saved);
                    cheats += 1;
                }
            }
            let left = grid[i][j - 1];
            let right = grid[i][j + 1];
            if left != i32::MAX && right != i32::MAX {
                let time_saved = (left - right).abs() - 2;
                if time_saved >= threshhold {
                    // println!("({}, {}): {}", i, j, time_saved);
                    cheats += 1;
                }
            }
        }
    }
    cheats
}

fn part_1(input: &str) -> Option<i32> {
    let grid = convert_to_grid(input);
    let dist = djikstra(&grid);
    // println!("{:?}", dist);
    // print_grid(&dist);
    find_cheats(&dist, 100).into()
}

fn find_cheats_2(grid: &[Vec<i32>], threshhold: i32, cheat_time: usize) -> i32 {
    let mut cheats = 0;
    let (n, m) = (grid.len(), grid[0].len());
    for from_i in 1..n - 1 {
        for from_j in 1..m - 1 {
            if grid[from_i][from_j] == i32::MAX {
                continue;
            }
            for to_i in from_i..std::cmp::min(from_i + 1 + cheat_time, n - 1) {
                let cheat_time_remain = cheat_time - (to_i - from_i);
                // for to_j in std::..std::cmp::min(from_j + 1 + cheat_time_remain, m - 1) {
                let mut to_j_start = std::cmp::max(1, from_j.saturating_sub(cheat_time_remain));
                if to_i == from_i {
                    to_j_start = from_j;
                }
                let to_j_end = std::cmp::min(from_j + cheat_time_remain, m - 1);
                for to_j in to_j_start..=to_j_end {
                    if grid[to_i][to_j] == i32::MAX {
                        continue;
                    }
                    let manhattan =
                        (to_i as i32 - from_i as i32).abs() + (to_j as i32 - from_j as i32).abs();
                    // println!("({from_i},{from_j})->({to_i},{to_j}), manhattan: {manhattan}");
                    let race_diff = (grid[from_i][from_j] - grid[to_i][to_j]).abs();
                    let time_saved = race_diff - manhattan;
                    if time_saved >= threshhold {
                        cheats += 1;
                    }
                }
            }
        }
    }
    cheats
}

fn part_2(input: &str) -> Option<i32> {
    let grid = convert_to_grid(input);
    let dist = djikstra(&grid);
    // print_grid(&dist);
    find_cheats_2(&dist, 100, 20).into()
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
