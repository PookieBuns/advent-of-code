use rayon::prelude::*;
use std::path::Path;

use crate::Answer;

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

fn get_area(a: Coord, b: Coord) -> usize {
    let xdist = a.x.abs_diff(b.x) + 1;
    let ydist = a.y.abs_diff(b.y) + 1;
    xdist * ydist
}

fn part_1(input: &str) -> Option<impl std::string::ToString> {
    let coords: Vec<Coord> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Coord {
                x: x.parse::<usize>().unwrap(),
                y: y.parse::<usize>().unwrap(),
            }
        })
        .collect();
    (0..coords.len())
        .flat_map(|i| (i + 1..coords.len()).map(move |j| (i, j)))
        .map(|(i, j)| get_area(coords[i], coords[j]))
        .max()
}

fn flood(grid: &mut [Vec<char>], start: Coord) {
    let mut deck = vec![start];
    let dirs = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    while let Some(coord) = deck.pop() {
        println!("{coord:?}");
        if coord.x >= grid.len() {
            continue;
        }
        if coord.y >= grid[0].len() {
            continue;
        }
        if matches!(grid[coord.x][coord.y], 'O' | 'X' | '#') {
            continue;
        }
        grid[coord.x][coord.y] = 'O';
        for (dx, dy) in dirs {
            let nx = coord.x.wrapping_add_signed(dx);
            let ny = coord.y.wrapping_add_signed(dy);
            if nx >= grid.len() {
                continue;
            }
            if ny >= grid[0].len() {
                continue;
            }
            if matches!(grid[nx][ny], 'O' | 'X' | '#') {
                continue;
            }
            let neighbors = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];
            if neighbors.iter().any(|(ddx, ddy)| {
                let nnx = nx.wrapping_add_signed(*ddx);
                let nny = ny.wrapping_add_signed(*ddy);
                if nnx >= grid.len() || nny >= grid[0].len() {
                    return false;
                }
                matches!(grid[nnx][nny], 'X' | '#')
            }) {
                deck.push(Coord { x: nx, y: ny });
            }
        }
    }
}

fn is_valid(grid: &[Vec<char>], a: Coord, b: Coord) -> bool {
    let big_y = a.y.max(b.y);
    let small_y = a.y.min(b.y);
    let big_x = a.x.max(b.x);
    let small_x = a.x.min(b.x);
    (small_x..=big_x)
        .flat_map(|i| (small_y..=big_y).map(move |j| (i, j)))
        .all(|(i, j)| matches!(grid[i][j], '.' | 'X' | '#'))
}

fn print_grid(grid: &[Vec<char>]) {
    grid.iter().for_each(|row| println!("{row:?}"));
}

fn part_2(input: &str) -> Option<impl std::string::ToString> {
    let coords: Vec<Coord> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Coord {
                x: x.parse::<usize>().unwrap(),
                y: y.parse::<usize>().unwrap(),
            }
        })
        .collect();
    let max_row_coord = coords.iter().max_by_key(|coord| coord.x).unwrap();
    let rows = max_row_coord.x;
    let cols = coords.iter().max_by_key(|coord| coord.y).unwrap().y;
    let mut grid = vec![vec!['.'; cols + 2]; rows + 2];
    coords.windows(2).for_each(|pair| {
        let first = pair[0];
        let second = pair[1];
        grid[first.x][first.y] = '#';
        grid[second.x][second.y] = '#';
        match (first.x.abs_diff(second.x), first.y.abs_diff(second.y)) {
            (0, _) => {
                let big_y = first.y.max(second.y);
                let small_y = first.y.min(second.y);
                (small_y + 1..big_y).for_each(|y| grid[first.x][y] = 'X');
            }
            (_, 0) => {
                let big_x = first.x.max(second.x);
                let small_x = first.x.min(second.x);
                (small_x + 1..big_x).for_each(|x| grid[x][first.y] = 'X');
            }
            _ => unreachable!(),
        }
    });
    let first = coords[0];
    let second = coords[coords.len() - 1];
    match (first.x.abs_diff(second.x), first.y.abs_diff(second.y)) {
        (0, _) => {
            let big_y = first.y.max(second.y);
            let small_y = first.y.min(second.y);
            (small_y + 1..big_y).for_each(|y| grid[first.x][y] = 'X');
        }
        (_, 0) => {
            let big_x = first.x.max(second.x);
            let small_x = first.x.min(second.x);
            (small_x + 1..big_x).for_each(|x| grid[x][first.y] = 'X');
        }
        _ => unreachable!(),
    }
    flood(
        &mut grid,
        Coord {
            x: max_row_coord.x + 1,
            y: max_row_coord.y + 1,
        },
    );
    println!("Done flood");
    // print_grid(&grid);
    let pairs = (0..coords.len())
        .flat_map(|i| (i + 1..coords.len()).map(move |j| (i, j))).collect::<Vec<(usize, usize)>>();
    let max_pair = pairs
        .par_iter()
        .max_by(|a, b| {
            let a_area = get_area(coords[a.0], coords[a.1]);
            let b_area = get_area(coords[b.0], coords[b.1]);
            if a_area > b_area && is_valid(&grid, coords[a.0], coords[a.1]) {
                return std::cmp::Ordering::Greater;
            }
            std::cmp::Ordering::Less
        })
        .unwrap();
    get_area(coords[max_pair.0], coords[max_pair.1]).into()
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
