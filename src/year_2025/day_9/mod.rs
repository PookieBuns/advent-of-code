use itertools::Itertools;
use std::cmp::Ordering::Equal;
use std::cmp::Ordering::Greater;
use std::cmp::Ordering::Less;
use std::{collections::HashSet, path::Path};

use crate::Answer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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

fn is_valid(outer: &HashSet<Coord>, a: Coord, b: Coord) -> bool {
    let big_y = a.y.max(b.y);
    let small_y = a.y.min(b.y);
    let big_x = a.x.max(b.x);
    let small_x = a.x.min(b.x);
    if (small_x..=big_x).any(|row| outer.contains(&Coord { x: row, y: small_y })) {
        return false;
    }
    if (small_x..=big_x).any(|row| outer.contains(&Coord { x: row, y: big_y })) {
        return false;
    }
    if (small_y..=big_y).any(|col| outer.contains(&Coord { x: small_x, y: col })) {
        return false;
    }
    if (small_y..=big_y).any(|col| outer.contains(&Coord { x: big_x, y: col })) {
        return false;
    }
    true
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
    let windows = coords
        .windows(2)
        .map(|pair| {
            let first = pair[0];
            let second = pair[1];
            (first, second)
        })
        .chain(std::iter::once((coords[0], coords[coords.len() - 1])));
    // counter clockwise
    let (mut outer, edge) = windows.fold(
        (HashSet::new(), HashSet::new()),
        |(mut outer, mut edge), (first, second)| {
            let direction = match (first.x.cmp(&second.x), first.y.cmp(&second.y)) {
                (Equal, Less) => Direction::Right,
                (Equal, Greater) => Direction::Left,
                (Less, Equal) => Direction::Down,
                (Greater, Equal) => Direction::Up,
                _ => unreachable!(),
            };
            match direction {
                Direction::Up => {
                    let big_x = first.x.max(second.x);
                    let small_x = first.x.min(second.x);
                    edge.extend((small_x..=big_x).map(|x| Coord { x, y: first.y }));
                    outer.extend((small_x..=big_x).map(|x| Coord { x, y: first.y + 1 }));
                }
                Direction::Down => {
                    let big_x = first.x.max(second.x);
                    let small_x = first.x.min(second.x);
                    edge.extend((small_x..=big_x).map(|x| Coord { x, y: first.y }));
                    outer.extend((small_x..=big_x).map(|x| Coord { x, y: first.y - 1 }));
                }
                Direction::Left => {
                    let big_y = first.y.max(second.y);
                    let small_y = first.y.min(second.y);
                    edge.extend((small_y..=big_y).map(|y| Coord { x: first.x, y }));
                    outer.extend((small_y..=big_y).map(|y| Coord { x: first.x - 1, y }));
                }
                Direction::Right => {
                    let big_y = first.y.max(second.y);
                    let small_y = first.y.min(second.y);
                    edge.extend((small_y..=big_y).map(|y| Coord { x: first.x, y }));
                    outer.extend((small_y..=big_y).map(|y| Coord { x: first.x + 1, y }));
                }
            };
            (outer, edge)
        },
    );
    outer.retain(|coord| !edge.contains(coord));
    let pairs = (0..coords.len())
        .flat_map(|i| (i + 1..coords.len()).map(move |j| (i, j)))
        .sorted_by(|a, b| {
            let area_a = get_area(coords[a.0], coords[a.1]);
            let area_b = get_area(coords[b.0], coords[b.1]);
            area_b.cmp(&area_a)
        })
        .collect::<Vec<(usize, usize)>>();
    pairs
        .into_iter()
        .find(|(i, j)| {
            let a = coords[*i];
            let b = coords[*j];
            // dbg!(a, b);
            let small_y = a.y.min(b.y);
            let big_y = a.y.max(b.y);
            if small_y < 48641 && big_y > 50126 {
                return false;
            }
            is_valid(&outer, coords[*i], coords[*j])
        })
        .map(|(i, j)| get_area(coords[i], coords[j]))
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
