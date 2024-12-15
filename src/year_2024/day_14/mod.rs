use std::{cmp::Ordering, path::Path};

use itertools::Itertools;
use regex::Regex;

use crate::Answer;

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

#[derive(Debug)]
enum Quadrant {
    A,
    B,
    C,
    D,
}

const ROWS: i32 = 103;
const COLS: i32 = 101;

fn get_robots(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let re = Regex::new(r"p=(\d+),(\d+) v=(-*\d+),(-*\d+)").unwrap();
            let captures = re.captures(line).unwrap();
            let x = captures[1].parse().unwrap();
            let y = captures[2].parse().unwrap();
            let vx = captures[3].parse().unwrap();
            let vy = captures[4].parse().unwrap();
            Robot { x, y, vx, vy }
        })
        .collect()
}

fn part_1(input: &str) -> Option<i32> {
    let robots = get_robots(input);
    // println!("{:?}", robots);
    let quadrants = robots
        .into_iter()
        .map(|robot| {
            let mut x = robot.x;
            let mut y = robot.y;
            for _ in 0..100 {
                x += robot.vx + COLS;
                x %= COLS;
                y += robot.vy + ROWS;
                y %= ROWS;
            }
            Robot {
                x,
                y,
                vx: robot.vx,
                vy: robot.vy,
            }
        })
        .filter_map(|robot| {
            let mid_row = ROWS / 2;
            let mid_col = COLS / 2;
            match (robot.x.cmp(&mid_col), robot.y.cmp(&mid_row)) {
                (Ordering::Less, Ordering::Less) => Some(Quadrant::A),
                (Ordering::Less, Ordering::Greater) => Some(Quadrant::B),
                (Ordering::Greater, Ordering::Less) => Some(Quadrant::C),
                (Ordering::Greater, Ordering::Greater) => Some(Quadrant::D),
                (_, _) => None,
            }
        })
        .fold((0, 0, 0, 0), |acc, quadrant| match quadrant {
            Quadrant::A => (acc.0 + 1, acc.1, acc.2, acc.3),
            Quadrant::B => (acc.0, acc.1 + 1, acc.2, acc.3),
            Quadrant::C => (acc.0, acc.1, acc.2 + 1, acc.3),
            Quadrant::D => (acc.0, acc.1, acc.2, acc.3 + 1),
        });
    (quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3).into()
}

fn print_grid(robots: &[Robot]) -> bool {
    let mut grid = vec![vec!['.'; COLS as usize]; ROWS as usize];
    for robot in robots {
        if grid[robot.y as usize][robot.x as usize] == '#' {
            return false;
        }
        grid[robot.y as usize][robot.x as usize] = '#';
    }
    for row in grid {
        println!("{}", row.iter().format(""));
    }
    println!();
    true
}

fn part_2(input: &str) -> Option<i32> {
    let start_robots = get_robots(input);
    let mut robots = start_robots;
    let mut i = 0;
    loop {
        i += 1;
        robots = robots
            .into_iter()
            .map(|robot| Robot {
                x: (robot.x + robot.vx + COLS) % COLS,
                y: (robot.y + robot.vy + ROWS) % ROWS,
                vx: robot.vx,
                vy: robot.vy,
            })
            .collect();
        if print_grid(&robots) {
            break;
        }
    }
    i.into()
}
pub fn solve() -> Answer {
    let cur_dir = Path::new(file!()).parent().unwrap();
    let file_path = cur_dir.join("input.txt");
    let input = std::fs::read_to_string(file_path).unwrap();
    let part_1 = part_1(&input);
    let part_2 = part_2(&input);
    Answer::from_parts(part_1, part_2)
}
