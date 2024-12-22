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

fn find_cheats(grid: &[Vec<i32>], threshhold: i32, cheat_time: usize) -> i32 {
    let mut cheats = 0;
    for from_i in 1..grid.len() - 1 {
        for from_j in 1..grid[from_i].len() - 1 {
            if grid[from_i][from_j] == i32::MAX {
                continue;
            }
            let to_i_min = std::cmp::max(1, from_i.saturating_sub(cheat_time));
            let to_i_max = std::cmp::min(from_i + cheat_time, grid.len() - 1);
            for to_i in to_i_min..=to_i_max {
                let cheat_time_remain =
                    cheat_time - (to_i as isize - from_i as isize).unsigned_abs();
                let to_j_min = std::cmp::max(1, from_j.saturating_sub(cheat_time_remain));
                let to_j_max = std::cmp::min(from_j + cheat_time_remain, grid[to_i].len() - 1);
                for to_j in to_j_min..=to_j_max {
                    if grid[to_i][to_j] == i32::MAX {
                        continue;
                    }
                    let manhattan =
                        (to_i as i32 - from_i as i32).abs() + (to_j as i32 - from_j as i32).abs();
                    // println!("({from_i},{from_j})->({to_i},{to_j}), manhattan: {manhattan}");
                    let race_diff = grid[from_i][from_j] - grid[to_i][to_j];
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

fn part_1(input: &str) -> Option<i32> {
    let grid = convert_to_grid(input);
    let dist = djikstra(&grid);
    find_cheats(&dist, 100, 2).into()
}

fn part_2(input: &str) -> Option<i32> {
    let grid = convert_to_grid(input);
    let dist = djikstra(&grid);
    find_cheats(&dist, 100, 20).into()
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
