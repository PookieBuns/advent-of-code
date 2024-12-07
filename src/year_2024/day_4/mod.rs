use std::path::Path;

use crate::Answer;

fn search_for_xmas(chars: &[Vec<char>], row: usize, col: usize) -> i32 {
    if chars[row][col] != 'X' {
        return 0;
    }
    let mut count = 0;
    // left
    if col + 3 < chars[row].len()
        && chars[row][col + 1] == 'M'
        && chars[row][col + 2] == 'A'
        && chars[row][col + 3] == 'S'
    {
        count += 1;
    }
    // right
    if col >= 3
        && chars[row][col - 1] == 'M'
        && chars[row][col - 2] == 'A'
        && chars[row][col - 3] == 'S'
    {
        count += 1;
    }
    // down
    if row + 3 < chars.len()
        && chars[row + 1][col] == 'M'
        && chars[row + 2][col] == 'A'
        && chars[row + 3][col] == 'S'
    {
        count += 1;
    }
    // up
    if row >= 3
        && chars[row - 1][col] == 'M'
        && chars[row - 2][col] == 'A'
        && chars[row - 3][col] == 'S'
    {
        count += 1;
    }
    // diagonal down right
    if row + 3 < chars.len()
        && col + 3 < chars[row].len()
        && chars[row + 1][col + 1] == 'M'
        && chars[row + 2][col + 2] == 'A'
        && chars[row + 3][col + 3] == 'S'
    {
            count += 1;
    }
    // diagonal down left
    if row + 3 < chars.len()
        && col >= 3
        && chars[row + 1][col - 1] == 'M'
        && chars[row + 2][col - 2] == 'A'
        && chars[row + 3][col - 3] == 'S'
    {
        count += 1;
    }
    // diagonal up right
    if row >= 3
        && col + 3 < chars[row].len()
        && chars[row - 1][col + 1] == 'M'
        && chars[row - 2][col + 2] == 'A'
        && chars[row - 3][col + 3] == 'S'
    {
        count += 1;
    }
    // diagonal up left
    if row >= 3
        && col >= 3
        && chars[row - 1][col - 1] == 'M'
        && chars[row - 2][col - 2] == 'A'
        && chars[row - 3][col - 3] == 'S'
    {
        count += 1;
    }
    count
}

fn part_1(input: &str) -> Option<i32> {
    let chars = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    (0..chars.len())
        .fold(0, |acc, row| {
            (0..chars[row].len()).fold(acc, |acc, col| acc + search_for_xmas(&chars, row, col))
        })
        .into()
}

fn search_for_xmas_2(chars: &[Vec<char>], row: usize, col: usize) -> i32 {
    if chars[row][col] != 'A' {
        return 0;
    }
    if row == 0 {
        return 0;
    }
    if row == chars.len() - 1 {
        return 0;
    }
    if col == 0 {
        return 0;
    }
    if col == chars[row].len() - 1 {
        return 0;
    }
    let up_left = chars[row - 1][col - 1];
    let up_right = chars[row - 1][col + 1];
    let down_left = chars[row + 1][col - 1];
    let down_right = chars[row + 1][col + 1];
    if up_left == 'M' && up_right == 'M' && down_left == 'S' && down_right == 'S' {
        return 1;
    }
    if up_left == 'M' && down_left == 'M' && up_right == 'S' && down_right == 'S' {
        return 1;
    }
    if down_left == 'M' && down_right == 'M' && up_left == 'S' && up_right == 'S' {
        return 1;
    }
    if up_right == 'M' && down_right == 'M' && up_left == 'S' && down_left == 'S' {
        return 1;
    }
    0
}

fn part_2(input: &str) -> Option<i32> {
    let chars = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    (0..chars.len())
        .fold(0, |acc, row| {
            (0..chars[row].len()).fold(acc, |acc, col| acc + search_for_xmas_2(&chars, row, col))
        })
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
