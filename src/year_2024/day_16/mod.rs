use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    path::Path,
};

use crate::Answer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Reindeer {
    score: i32,
    row: usize,
    col: usize,
    direction: Direction,
}

impl Reindeer {
    fn get_moves(&self, grid: &[Vec<char>]) -> Vec<Reindeer> {
        let turn_left = Reindeer {
            score: self.score + 1000,
            row: self.row,
            col: self.col,
            direction: self.direction.get_left(),
        };
        let turn_right = Reindeer {
            score: self.score + 1000,
            row: self.row,
            col: self.col,
            direction: self.direction.get_right(),
        };
        let mut moves = vec![turn_left, turn_right];
        let (dx, dy) = self.direction.get_dx_dy();
        let (nrow, ncol) = (
            self.row.overflowing_add_signed(dx).0,
            self.col.overflowing_add_signed(dy).0,
        );
        if nrow < grid.len() && ncol < grid[nrow].len() {
            let next_char = grid[nrow][ncol];
            if matches!(next_char, '.' | 'S' | 'E') {
                moves.push(Reindeer {
                    score: self.score + 1,
                    row: nrow,
                    col: ncol,
                    direction: self.direction,
                });
            }
        }
        moves
    }

    fn get_moves_with_path(
        &self,
        grid: &[Vec<char>],
        cur: &[(usize, usize)],
    ) -> Vec<(Reindeer, Vec<(usize, usize)>)> {
        let moves = self.get_moves(grid);
        moves
            .into_iter()
            .map(|reindeer| {
                let mut extended = cur.to_vec();
                if reindeer.row != self.row || reindeer.col != self.col {
                    extended.push((reindeer.row, reindeer.col));
                }
                (reindeer, extended)
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_dx_dy(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn get_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn get_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn convert_to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part_1(input: &str) -> Option<i32> {
    let grid = convert_to_grid(input);
    let start_row = grid.len() - 2;
    let start_col = 1;
    assert_eq!(grid[start_row][start_col], 'S');
    let mut scores = HashMap::new();
    scores.insert((start_row, start_col, Direction::Right), 0);
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(Reindeer {
        score: 0,
        row: start_row,
        col: start_col,
        direction: Direction::Right,
    }));
    while let Some(Reverse(reindeer)) = heap.pop() {
        if grid[reindeer.row][reindeer.col] == 'E' {
            return Some(reindeer.score);
        }
        for next_reindeer in reindeer.get_moves(&grid) {
            if let Some(&prev_score) = scores.get(&(
                next_reindeer.row,
                next_reindeer.col,
                next_reindeer.direction,
            )) {
                if prev_score <= next_reindeer.score {
                    continue;
                }
            }
            scores.insert(
                (
                    next_reindeer.row,
                    next_reindeer.col,
                    next_reindeer.direction,
                ),
                next_reindeer.score,
            );
            heap.push(Reverse(next_reindeer));
        }
    }
    None
}

fn part_2(input: &str) -> Option<i32> {
    let grid = convert_to_grid(input);
    let start_row = grid.len() - 2;
    let start_col = 1;
    assert_eq!(grid[start_row][start_col], 'S');
    let mut scores = HashMap::new();
    scores.insert((start_row, start_col, Direction::Right), 0);
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((
        Reindeer {
            score: 0,
            row: start_row,
            col: start_col,
            direction: Direction::Right,
        },
        vec![(start_row, start_col)],
    )));
    while let Some(Reverse((reindeer, path))) = heap.pop() {
        if grid[reindeer.row][reindeer.col] == 'E' {
            let mut paths = vec![path];
            while let Some(Reverse((other_reindeer, other_path))) = heap.pop() {
                if grid[other_reindeer.row][other_reindeer.col] != 'E' {
                    break;
                }
                paths.push(other_path);
            }
            let paths_set: HashSet<_> = HashSet::from_iter(paths.into_iter().flatten());
            return Some(paths_set.len() as i32);
        }
        for (next_reindeer, next_path) in reindeer.get_moves_with_path(&grid, &path) {
            if let Some(&prev_score) = scores.get(&(
                next_reindeer.row,
                next_reindeer.col,
                next_reindeer.direction,
            )) {
                if prev_score < next_reindeer.score {
                    continue;
                }
            }
            scores.insert(
                (
                    next_reindeer.row,
                    next_reindeer.col,
                    next_reindeer.direction,
                ),
                next_reindeer.score,
            );
            heap.push(Reverse((next_reindeer, next_path)));
        }
    }
    None
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
