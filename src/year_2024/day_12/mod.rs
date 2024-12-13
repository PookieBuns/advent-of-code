use std::{collections::HashSet, path::Path};

use crate::Answer;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Border {
    row: usize,
    col: usize,
    direction: Direction,
}

fn convert_to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn create_borders(row: usize, col: usize) -> HashSet<Border> {
    let top = Border {
        row,
        col,
        direction: Direction::Up,
    };
    let bottom = Border {
        row,
        col,
        direction: Direction::Down,
    };
    let left = Border {
        row,
        col,
        direction: Direction::Left,
    };
    let right = Border {
        row,
        col,
        direction: Direction::Right,
    };
    HashSet::from([top, bottom, left, right])
}

fn calculate(
    grid: &mut [Vec<char>],
    c: char,
    row: usize,
    col: usize,
    seen: &mut HashSet<(usize, usize)>,
) -> (i32, HashSet<Border>) {
    // println!("{row},{col}");
    if grid[row][col] != c {
        return Default::default();
    }
    if seen.contains(&(row, col)) {
        return Default::default();
    }
    seen.insert((row, col));
    grid[row][col] = c.to_ascii_lowercase();
    let mut borders = create_borders(row, col);
    let mut area = 1;
    for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let nrow = row.overflowing_add_signed(dr).0;
        let ncol = col.overflowing_add_signed(dc).0;
        if nrow >= grid.len() {
            continue;
        }
        if ncol >= grid[nrow].len() {
            continue;
        }
        let (n_area, n_borders) = calculate(grid, c, nrow, ncol, seen);
        for border in n_borders {
            match border.direction {
                Direction::Up => {
                    let facing_border = Border {
                        row: border.row.overflowing_sub(1).0,
                        col: border.col,
                        direction: Direction::Down,
                    };
                    if borders.contains(&facing_border) {
                        borders.remove(&facing_border);
                    } else {
                        borders.insert(border);
                    }
                }
                Direction::Down => {
                    let facing_border = Border {
                        row: border.row.overflowing_add(1).0,
                        col: border.col,
                        direction: Direction::Up,
                    };
                    if borders.contains(&facing_border) {
                        borders.remove(&facing_border);
                    } else {
                        borders.insert(border);
                    }
                }
                Direction::Left => {
                    let facing_border = Border {
                        row: border.row,
                        col: border.col.overflowing_sub(1).0,
                        direction: Direction::Right,
                    };
                    if borders.contains(&facing_border) {
                        borders.remove(&facing_border);
                    } else {
                        borders.insert(border);
                    }
                }
                Direction::Right => {
                    let facing_border = Border {
                        row: border.row,
                        col: border.col.overflowing_add(1).0,
                        direction: Direction::Left,
                    };
                    if borders.contains(&facing_border) {
                        borders.remove(&facing_border);
                    } else {
                        borders.insert(border);
                    }
                }
            }
        }
        area += n_area;
    }
    (area, borders)
}

fn part_1(input: &str) -> Option<i32> {
    let mut grid = convert_to_grid(input);
    let mut price = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let c = grid[row][col];
            if c.is_ascii_lowercase() {
                continue;
            }
            let (area, borders) = calculate(&mut grid, c, row, col, &mut HashSet::new());
            price += area * borders.len() as i32;
        }
    }
    price.into()
}

fn part_2(input: &str) -> Option<i32> {
    let mut grid = convert_to_grid(input);
    let mut price = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let c = grid[row][col];
            if c.is_ascii_lowercase() {
                continue;
            }
            let (area, mut borders) = calculate(&mut grid, c, row, col, &mut HashSet::new());
            let mut sides = borders.len() as f64;
            for border in &borders {
                match border.direction {
                    Direction::Up => {
                        let left_border = Border {
                            row: border.row,
                            col: border.col.overflowing_sub(1).0,
                            direction: Direction::Up,
                        };
                        if borders.contains(&left_border) {
                            sides -= 0.5;
                        }
                        let right_border = Border {
                            row: border.row,
                            col: border.col.overflowing_add(1).0,
                            direction: Direction::Up,
                        };
                        if borders.contains(&right_border) {
                            sides -= 0.5;
                        }
                    }
                    Direction::Down => {
                        let left_border = Border {
                            row: border.row,
                            col: border.col.overflowing_sub(1).0,
                            direction: Direction::Down,
                        };
                        if borders.contains(&left_border) {
                            sides -= 0.5;
                        }
                        let right_border = Border {
                            row: border.row,
                            col: border.col.overflowing_add(1).0,
                            direction: Direction::Down,
                        };
                        if borders.contains(&right_border) {
                            sides -= 0.5;
                        }
                    }
                    Direction::Left => {
                        let top_border = Border {
                            row: border.row.overflowing_sub(1).0,
                            col: border.col,
                            direction: Direction::Left,
                        };
                        if borders.contains(&top_border) {
                            sides -= 0.5;
                        }
                        let bottom_border = Border {
                            row: border.row.overflowing_add(1).0,
                            col: border.col,
                            direction: Direction::Left,
                        };
                        if borders.contains(&bottom_border) {
                            sides -= 0.5;
                        }
                    }
                    Direction::Right => {
                        let top_border = Border {
                            row: border.row.overflowing_sub(1).0,
                            col: border.col,
                            direction: Direction::Right,
                        };
                        if borders.contains(&top_border) {
                            sides -= 0.5;
                        }
                        let bottom_border = Border {
                            row: border.row.overflowing_add(1).0,
                            col: border.col,
                            direction: Direction::Right,
                        };
                        if borders.contains(&bottom_border) {
                            sides -= 0.5;
                        }
                    }
                }
            }
            println!("c={c}, sides={:?}", sides);
            price += (area as f64 * sides) as i32;
        }
    }
    price.into()
}
pub fn solve() -> Answer {
    let cur_dir = Path::new(file!()).parent().unwrap();
    let file_path = cur_dir.join("input.txt");
    let input = std::fs::read_to_string(file_path).unwrap();
    let part_1 = part_1(&input);
    let part_2 = part_2(&input);
    Answer::from_parts(part_1, part_2)
}
