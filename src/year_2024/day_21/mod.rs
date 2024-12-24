use std::{cmp::Ordering, path::Path};

use cached::proc_macro::cached;

use crate::Answer;

const NUMERIC_GRID: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['.', '0', 'A'],
];

const DIRECTIONAL_GRID: [[char; 3]; 2] = [['.', '^', 'A'], ['<', 'v', '>']];
const DIRECTIONAL_GRID_START: (usize, usize) = (0, 2);

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_grid_pos(&self) -> (usize, usize) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (1, 1),
            Direction::Left => (1, 0),
            Direction::Right => (1, 2),
        }
    }
}

fn get_num_grid_pos(c: char) -> (usize, usize) {
    match c {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '0' => (3, 1),
        'A' => (3, 2),
        _ => unreachable!(),
    }
}

#[cached]
fn get_moves_num_grid(
    start: (usize, usize),
    end: (usize, usize),
    sub_grid_pos: (usize, usize),
    indirection: i64,
) -> i64 {
    let di = end.0 as i64 - start.0 as i64;
    let dj = end.1 as i64 - start.1 as i64;
    if di == 0 && dj == 0 {
        // move sub_grid_back to A
        let move_to_a = get_moves_dir_grid(
            sub_grid_pos,
            DIRECTIONAL_GRID_START,
            DIRECTIONAL_GRID_START,
            indirection,
        );
        return move_to_a;
    }
    let mut ans = i64::MAX;
    match di.cmp(&0) {
        Ordering::Greater => {
            // move down
            if NUMERIC_GRID[start.0 + 1][start.1] != '.' {
                let sub_grid_target_pos = Direction::Down.get_grid_pos();
                let move_down = get_moves_dir_grid(
                    sub_grid_pos,
                    sub_grid_target_pos,
                    DIRECTIONAL_GRID_START,
                    indirection,
                );
                let remain = get_moves_num_grid(
                    (start.0 + 1, start.1),
                    end,
                    sub_grid_target_pos,
                    indirection,
                );
                ans = ans.min(move_down + remain);
            }
        }
        Ordering::Less => {
            // move up
            if NUMERIC_GRID[start.0 - 1][start.1] != '.' {
                let sub_grid_target_pos = Direction::Up.get_grid_pos();
                let move_up = get_moves_dir_grid(
                    sub_grid_pos,
                    sub_grid_target_pos,
                    DIRECTIONAL_GRID_START,
                    indirection,
                );
                let remain = get_moves_num_grid(
                    (start.0 - 1, start.1),
                    end,
                    sub_grid_target_pos,
                    indirection,
                );
                ans = ans.min(move_up + remain);
            }
        }
        Ordering::Equal => {}
    };
    match dj.cmp(&0) {
        Ordering::Greater => {
            // move right
            if NUMERIC_GRID[start.0][start.1 + 1] != '.' {
                let sub_grid_target_pos = Direction::Right.get_grid_pos();
                let move_right = get_moves_dir_grid(
                    sub_grid_pos,
                    sub_grid_target_pos,
                    DIRECTIONAL_GRID_START,
                    indirection,
                );
                let remain = get_moves_num_grid(
                    (start.0, start.1 + 1),
                    end,
                    sub_grid_target_pos,
                    indirection,
                );
                ans = ans.min(move_right + remain);
            }
        }
        Ordering::Less => {
            // move left
            if NUMERIC_GRID[start.0][start.1 - 1] != '.' {
                let sub_grid_target_pos = Direction::Left.get_grid_pos();
                let move_left = get_moves_dir_grid(
                    sub_grid_pos,
                    sub_grid_target_pos,
                    DIRECTIONAL_GRID_START,
                    indirection,
                );
                let remain = get_moves_num_grid(
                    (start.0, start.1 - 1),
                    end,
                    sub_grid_target_pos,
                    indirection,
                );
                ans = ans.min(move_left + remain);
            }
        }
        Ordering::Equal => {}
    };
    // println!("num grid start = {start:?}, end = {end:?}, returning {}", ans);
    ans
}

#[cached]
fn get_moves_dir_grid(
    start: (usize, usize),
    end: (usize, usize),
    sub_grid_pos: (usize, usize),
    indirection: i64,
) -> i64 {
    let di = end.0 as i64 - start.0 as i64;
    let dj = end.1 as i64 - start.1 as i64;
    if indirection == 1 {
        // println!("indirection = 0 start = {start:?}, end = {end:?}, returning {}", di.abs() + dj.abs() + 1);
        return di.abs() + dj.abs() + 1;
    }
    if di == 0 && dj == 0 {
        // move sub_grid_back to A
        let move_to_a = get_moves_dir_grid(
            sub_grid_pos,
            DIRECTIONAL_GRID_START,
            DIRECTIONAL_GRID_START,
            indirection - 1,
        );
        return move_to_a;
    }
    let mut ans = i64::MAX;
    match di.cmp(&0) {
        Ordering::Greater => {
            // move down
            if DIRECTIONAL_GRID[start.0 + 1][start.1] != '.' {
                let sub_grid_target_pos = Direction::Down.get_grid_pos();
                let move_down = get_moves_dir_grid(
                    sub_grid_pos,
                    sub_grid_target_pos,
                    DIRECTIONAL_GRID_START,
                    indirection - 1,
                );
                let remain = get_moves_dir_grid(
                    (start.0 + 1, start.1),
                    end,
                    sub_grid_target_pos,
                    indirection,
                );
                // println!("move_down = {move_down}, remain = {remain}");
                ans = ans.min(move_down + remain);
            }
        }
        Ordering::Less => {
            // move up
            if DIRECTIONAL_GRID[start.0 - 1][start.1] != '.' {
                let sub_grid_target_pos = Direction::Up.get_grid_pos();
                let move_up = get_moves_dir_grid(
                    sub_grid_pos,
                    sub_grid_target_pos,
                    DIRECTIONAL_GRID_START,
                    indirection - 1,
                );
                let remain = get_moves_dir_grid(
                    (start.0 - 1, start.1),
                    end,
                    sub_grid_target_pos,
                    indirection,
                );
                // println!("move_up = {move_up}, remain = {remain}");
                ans = ans.min(move_up + remain);
            }
        }
        Ordering::Equal => {}
    };
    match dj.cmp(&0) {
        Ordering::Greater => {
            // move right
            if DIRECTIONAL_GRID[start.0][start.1 + 1] != '.' {
                let sub_grid_target_pos = Direction::Right.get_grid_pos();
                let move_right = get_moves_dir_grid(
                    sub_grid_pos,
                    sub_grid_target_pos,
                    DIRECTIONAL_GRID_START,
                    indirection - 1,
                );
                let remain = get_moves_dir_grid(
                    (start.0, start.1 + 1),
                    end,
                    sub_grid_target_pos,
                    indirection,
                );
                // println!("move_right = {move_right}, remain = {remain}");
                ans = ans.min(move_right + remain);
            }
        }
        Ordering::Less => {
            // move left
            if DIRECTIONAL_GRID[start.0][start.1 - 1] != '.' {
                let sub_grid_target_pos = Direction::Left.get_grid_pos();
                let move_left = get_moves_dir_grid(
                    sub_grid_pos,
                    sub_grid_target_pos,
                    DIRECTIONAL_GRID_START,
                    indirection - 1,
                );
                let remain = get_moves_dir_grid(
                    (start.0, start.1 - 1),
                    end,
                    sub_grid_target_pos,
                    indirection,
                );
                // println!("move_left = {move_left}, remain = {remain}");
                ans = ans.min(move_left + remain);
            }
        }
        Ordering::Equal => {}
    };
    // println!("dir grid indirection = {indirection} start = {start:?}, end = {end:?}, returning {}", ans);
    ans
}

fn get_complexity(code: &str, indirection: i64) -> i64 {
    let code_value: i64 = code[0..3].parse().unwrap();
    code.chars()
        .fold((0, get_num_grid_pos('A')), |(ans, start_grid_pos), c| {
            let end_grid_pos = get_num_grid_pos(c);
            let moves = get_moves_num_grid(
                start_grid_pos,
                end_grid_pos,
                DIRECTIONAL_GRID_START,
                indirection,
            );
            (ans + moves, end_grid_pos)
        })
        .0
        * code_value
}

fn part_1(input: &str) -> Option<i64> {
    input
        .lines()
        .map(|line| get_complexity(line, 2))
        .sum::<i64>()
        .into()
}

fn part_2(input: &str) -> Option<i64> {
    input
        .lines()
        .map(|line| get_complexity(line, 25))
        .sum::<i64>()
        .into()
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
