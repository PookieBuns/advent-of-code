use std::{collections::HashSet, path::Path};

use crate::Answer;

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn get_direction(&self) -> (isize, isize) {
        match self {
            Move::Up => (-1, 0),
            Move::Down => (1, 0),
            Move::Left => (0, -1),
            Move::Right => (0, 1),
        }
    }
}

fn convert_to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_starting_pos(grid: &[Vec<char>]) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '@' {
                return (i, j);
            }
        }
    }
    unreachable!()
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<char>]) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn push(grid: &mut [Vec<char>], i: usize, j: usize, m: Move) -> (usize, usize) {
    // println!("{:?}", m);
    let (di, dj) = m.get_direction();
    let mut push_blocks = 0;
    let mut cur_i = i;
    let mut cur_j = j;
    loop {
        push_blocks += 1;
        let ni = cur_i.overflowing_add_signed(di).0;
        let nj = cur_j.overflowing_add_signed(dj).0;
        // println!("{},{}", ni, nj);
        if grid[ni][nj] == '#' {
            return (i, j);
        }
        if grid[ni][nj] == '.' {
            break;
        }
        cur_i = ni;
        cur_j = nj;
    }
    for block in (0..push_blocks).rev() {
        let source_i = i.overflowing_add_signed(di * block).0;
        let source_j = j.overflowing_add_signed(dj * block).0;
        let target_i = source_i.overflowing_add_signed(di).0;
        let target_j = source_j.overflowing_add_signed(dj).0;
        // println!("{},{} -> {},{}", source_i, source_j, target_i, target_j);
        grid[target_i][target_j] = grid[source_i][source_j];
    }
    grid[i][j] = '.';
    (
        i.overflowing_add_signed(di).0,
        j.overflowing_add_signed(dj).0,
    )
}

fn part_1(input: &str) -> Option<i32> {
    let (map_str, moves_str) = input.split_once("\n\n").unwrap();
    let mut map = convert_to_grid(map_str);
    let moves: Vec<Move> = moves_str
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '^' => Move::Up,
                'v' => Move::Down,
                '<' => Move::Left,
                '>' => Move::Right,
                _ => unreachable!(),
            })
        })
        .collect();
    let (mut i, mut j) = get_starting_pos(&map);
    for m in moves {
        // push(&mut map, i, j, m);
        let (ni, nj) = push(&mut map, i, j, m);
        i = ni;
        j = nj;
        // print_grid(&map);
    }
    let mut ans = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'O' {
                ans += i * 100 + j;
            }
        }
    }
    Some(ans as i32)
}

fn resize_map(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut resized_map = Vec::new();
    for row in map {
        let mut new_row = Vec::new();
        for c in row {
            match c {
                '#' => {
                    new_row.push('#');
                    new_row.push('#')
                }
                '.' => {
                    new_row.push('.');
                    new_row.push('.')
                }
                'O' => {
                    new_row.push('[');
                    new_row.push(']')
                }
                '@' => {
                    new_row.push('@');
                    new_row.push('.')
                }
                _ => unreachable!(),
            }
        }
        resized_map.push(new_row);
    }
    resized_map
}

fn push_wide(grid: &mut [Vec<char>], row: usize, col: usize, m: Move) -> (usize, usize) {
    // use regular push for left and right
    if matches!(m, Move::Left | Move::Right) {
        return push(grid, row, col, m);
    }
    let (di, dj) = m.get_direction();
    let mut push_q = vec![(row, col)];
    let mut all_push = HashSet::from([(row, col)]);
    while let Some((i, j)) = push_q.pop() {
        let ni = i.overflowing_add_signed(di).0;
        let nj = j.overflowing_add_signed(dj).0;
        match grid[ni][nj] {
            '#' => return (row, col),
            '.' => {}
            '[' => {
                push_q.push((ni, nj));
                push_q.push((ni, nj + 1));
                all_push.insert((ni, nj));
                all_push.insert((ni, nj + 1));
            }
            ']' => {
                push_q.push((ni, nj));
                push_q.push((ni, nj - 1));
                all_push.insert((ni, nj));
                all_push.insert((ni, nj - 1));
            }
            _ => unreachable!(),
        }
    }
    let mut all_push_vec = Vec::from_iter(all_push);
    if matches!(m, Move::Up) {
        all_push_vec.sort();
    } else {
        // reverse
        all_push_vec.sort_by(|a, b| b.cmp(a));
    }
    for (source_i, source_j) in all_push_vec {
        let target_i = source_i.overflowing_add_signed(di).0;
        let target_j = source_j.overflowing_add_signed(dj).0;
        grid[target_i][target_j] = grid[source_i][source_j];
        grid[source_i][source_j] = '.';
    }
    grid[row][col] = '.';
    (
        row.overflowing_add_signed(di).0,
        col.overflowing_add_signed(dj).0,
    )
}

fn part_2(input: &str) -> Option<i32> {
    let (map_str, moves_str) = input.split_once("\n\n").unwrap();
    let mut map = convert_to_grid(map_str);
    let moves: Vec<Move> = moves_str
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '^' => Move::Up,
                'v' => Move::Down,
                '<' => Move::Left,
                '>' => Move::Right,
                _ => unreachable!(),
            })
        })
        .collect();
    map = resize_map(map);
    let (mut i, mut j) = get_starting_pos(&map);
    for m in moves {
        // push(&mut map, i, j, m);
        let (ni, nj) = push_wide(&mut map, i, j, m);
        i = ni;
        j = nj;
        // print_grid(&map);
    }
    let mut ans = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '[' {
                ans += i * 100 + j;
            }
        }
    }
    Some(ans as i32)
}
pub fn solve() -> Answer {
    let cur_dir = Path::new(file!()).parent().unwrap();
    let file_path = cur_dir.join("input.txt");
    let input = std::fs::read_to_string(file_path).unwrap();
    let part_1 = part_1(&input);
    let part_2 = part_2(&input);
    Answer::from_parts(part_1, part_2)
}
