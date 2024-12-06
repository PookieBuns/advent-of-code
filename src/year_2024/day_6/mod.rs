use std::path::Path;

use crate::Answer;

fn find_starting_point(chars: &[Vec<char>]) -> (usize, usize) {
    for row in 0..chars.len() {
        for col in 0..chars[row].len() {
            if chars[row][col] == '^' {
                return (row, col);
            }
        }
    }
    (0, 0)
}

fn count_x(chars: &[Vec<char>]) -> i32 {
    let mut c = 0;
    for row in chars {
        for col in row {
            if *col == 'X' {
                c += 1;
            }
        }
    }
    c
}

fn part_1(input: &str) -> Option<i32> {
    let mut chars = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (starting_row, starting_col) = find_starting_point(&chars);
    let dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir_index = 0;
    let mut row = starting_row;
    let mut col = starting_col;
    loop {
        let mut nrow = row as i32 + dirs[dir_index].0;
        let mut ncol = col as i32 + dirs[dir_index].1;
        if nrow < 0 || nrow as usize >= chars.len() {
            break;
        }
        if ncol < 0 || ncol as usize >= chars[0].len() {
            break;
        }
        if chars[nrow as usize][ncol as usize] == '.' {
            chars[nrow as usize][ncol as usize] = 'X';
        }
        if chars[nrow as usize][ncol as usize] == '#' {
            nrow -= dirs[dir_index].0;
            ncol -= dirs[dir_index].1;
            dir_index = (dir_index + 1) % dirs.len();
        }
        row = nrow as usize;
        col = ncol as usize;
    }
    (count_x(&chars) + 1).into()
}

fn part_2(input: &str) -> Option<i32> {
    let mut chars = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (starting_row, starting_col) = find_starting_point(&chars);
    let mut ans = 0;
    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            if i == starting_row && j == starting_col {
                continue;
            }
            let mut chars_copy = chars.clone();
            chars_copy[i][j] = '#';
            let dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
            let mut dir_index = 0;
            let mut row = starting_row;
            let mut col = starting_col;
            let mut seen_walls = Vec::new();
            loop {
                let mut nrow = row as i32 + dirs[dir_index].0;
                let mut ncol = col as i32 + dirs[dir_index].1;
                if nrow < 0 || nrow as usize >= chars_copy.len() {
                    break;
                }
                if ncol < 0 || ncol as usize >= chars_copy[0].len() {
                    break;
                }
                if chars_copy[nrow as usize][ncol as usize] == '.' {
                    chars_copy[nrow as usize][ncol as usize] = 'X';
                }
                if chars_copy[nrow as usize][ncol as usize] == '#' {
                    if seen_walls.contains(&(row, col, nrow, ncol)) {
                        ans += 1;
                        break;
                    }
                    seen_walls.push((row, col, nrow, ncol));
                    nrow -= dirs[dir_index].0;
                    ncol -= dirs[dir_index].1;
                    dir_index = (dir_index + 1) % dirs.len();
                }
                row = nrow as usize;
                col = ncol as usize;
            }
        }
    }
    ans.into()
}
pub fn solve() -> Answer {
    let cur_dir = Path::new(file!()).parent().unwrap();
    let file_path = cur_dir.join("input.txt");
    let input = std::fs::read_to_string(file_path).unwrap();
    let part_1 = part_1(&input);
    let part_2 = part_2(&input);
    Answer { part_1, part_2 }
}
