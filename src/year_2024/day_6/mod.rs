use std::path::Path;

use crate::Answer;

fn find_starting_point(chars: &[Vec<char>]) -> (usize, usize) {
    chars
        .iter()
        .enumerate()
        .find_map(|(row, line)| line.iter().position(|&c| c == '^').map(|col| (row, col)))
        .unwrap()
}

fn count_x(chars: &[Vec<char>]) -> i32 {
    chars.iter().flatten().filter(|&&c| c == 'X').count() as i32
}

fn part_1(input: &str) -> Option<i32> {
    let mut chars = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (starting_row, starting_col) = find_starting_point(&chars);
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    std::iter::successors(
        Some((starting_row as i32, starting_col as i32, 0)),
        |&(row, col, dir)| {
            let (drow, dcol) = dirs[dir];
            let nrow = row + drow;
            let ncol = col + dcol;
            if nrow < 0 || nrow >= chars.len() as i32 {
                return None;
            }
            if ncol < 0 || ncol >= chars[0].len() as i32 {
                return None;
            }
            match chars[nrow as usize][ncol as usize] {
                '.' => {
                    chars[nrow as usize][ncol as usize] = 'X';
                    Some((nrow, ncol, dir))
                }
                '#' => Some((row, col, (dir + 1) % dirs.len())),
                _ => Some((nrow, ncol, dir)),
            }
        },
    )
    .last();
    (count_x(&chars) + 1).into()
}

fn part_2(input: &str) -> Option<i32> {
    let chars = input
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
            let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
            let mut seen_walls = Vec::new();
            std::iter::successors(
                Some((starting_row as i32, starting_col as i32, 0)),
                |&(row, col, dir)| {
                    let (drow, dcol) = dirs[dir];
                    let nrow = row + drow;
                    let ncol = col + dcol;
                    if nrow < 0 || nrow >= chars_copy.len() as i32 {
                        return None;
                    }
                    if ncol < 0 || ncol >= chars_copy[0].len() as i32 {
                        return None;
                    }
                    match chars_copy[nrow as usize][ncol as usize] {
                        '.' => {
                            chars_copy[nrow as usize][ncol as usize] = 'X';
                            Some((nrow, ncol, dir))
                        }
                        '#' => {
                            if seen_walls.contains(&(row, col, nrow, ncol)) {
                                ans += 1;
                                None
                            } else {
                                seen_walls.push((row, col, nrow, ncol));
                                Some((row, col, (dir + 1) % dirs.len()))
                            }
                        }
                        _ => Some((nrow, ncol, dir)),
                    }
                },
            )
            .last();
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
    Answer::from_parts(part_1, part_2)
}
