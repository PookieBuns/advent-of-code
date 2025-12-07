use std::path::Path;

use crate::Answer;

fn part_1(input: &str) -> Option<impl std::string::ToString> {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut splits = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    for row in 0..rows - 1 {
        for col in 0..cols {
            let current = grid[row][col];
            if current == 'S' || current == '|' {
                let below = grid[row + 1][col];
                if below == '^' {
                    splits += 1;
                    grid[row + 1][col - 1] = '|';
                    grid[row + 1][col + 1] = '|';
                } else {
                    grid[row + 1][col] = '|';
                }
            }
        }
    }
    splits.into()
}

fn part_2(input: &str) -> Option<impl std::string::ToString> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let start_row = 0;
    let start_col = grid[start_row].iter().position(|&c| c == 'S').unwrap();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dp = vec![vec![0; cols]; rows];
    for c in &mut dp[rows - 1] {
        *c = 1;
    }
    for row in (0..rows - 1).rev() {
        for col in 0..cols {
            let below = grid[row + 1][col];
            if below == '^' {
                if col > 0 {
                    dp[row][col] += dp[row + 1][col - 1];
                }
                if col < cols - 1 {
                    dp[row][col] += dp[row + 1][col + 1];
                }
            } else {
                dp[row][col] += dp[row + 1][col];
            }
        }
    }
    dp[start_row][start_col].into()
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
