use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use crate::Answer;

fn convert_to_vec(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_antennas(grid: &[Vec<char>]) -> HashMap<char, Vec<(usize, usize)>> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, cell)| *cell != '.')
                .map(move |(j, cell)| (*cell, (i, j)))
        })
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.entry(key).or_default().push(value);
            acc
        })
}

fn part_1(input: &str) -> Option<i32> {
    let grid = convert_to_vec(input);
    let antennas = get_antennas(&grid);
    let mut antinodes = HashSet::new();
    for value in antennas.values() {
        for i in 0..value.len() {
            for j in i + 1..value.len() {
                let (x1, y1) = value[i];
                let (x2, y2) = value[j];
                let xdist = x2 as i32 - x1 as i32;
                let ydist = y2 as i32 - y1 as i32;
                let antinode_1 = (x1 as i32 - xdist, y1 as i32 - ydist);
                let antinode_2 = (x2 as i32 + xdist, y2 as i32 + ydist);
                if antinode_1.0 >= 0
                    && antinode_1.0 < grid.len() as i32
                    && antinode_1.1 >= 0
                    && antinode_1.1 < grid[0].len() as i32
                {
                    antinodes.insert(antinode_1);
                }
                if antinode_2.0 >= 0
                    && antinode_2.0 < grid.len() as i32
                    && antinode_2.1 >= 0
                    && antinode_2.1 < grid[0].len() as i32
                {
                    antinodes.insert(antinode_2);
                }
            }
        }
    }
    (antinodes.len() as i32).into()
}

fn part_2(input: &str) -> Option<i32> {
    let grid = convert_to_vec(input);
    let antennas = get_antennas(&grid);
    let mut antinodes = HashSet::new();
    for value in antennas.values() {
        for i in 0..value.len() {
            for j in i + 1..value.len() {
                let (x1, y1) = value[i];
                let (x2, y2) = value[j];
                antinodes.insert((x1 as i32, y1 as i32));
                antinodes.insert((x2 as i32, y2 as i32));
                let xdist = x2 as i32 - x1 as i32;
                let ydist = y2 as i32 - y1 as i32;
                let mut antinode_1 = (x1 as i32 - xdist, y1 as i32 - ydist);
                while antinode_1.0 >= 0
                    && antinode_1.0 < grid.len() as i32
                    && antinode_1.1 >= 0
                    && antinode_1.1 < grid[0].len() as i32
                {
                    antinodes.insert(antinode_1);
                    antinode_1.0 -= xdist;
                    antinode_1.1 -= ydist;
                }
                let mut antinode_2 = (x2 as i32 + xdist, y2 as i32 + ydist);
                while antinode_2.0 >= 0
                    && antinode_2.0 < grid.len() as i32
                    && antinode_2.1 >= 0
                    && antinode_2.1 < grid[0].len() as i32
                {
                    antinodes.insert(antinode_2);
                    antinode_2.0 += xdist;
                    antinode_2.1 += ydist;
                }
            }
        }
    }
    (antinodes.len() as i32).into()
}
pub fn solve() -> Answer {
    let cur_dir = Path::new(file!()).parent().unwrap();
    let file_path = cur_dir.join("input.txt");
    let input = std::fs::read_to_string(file_path).unwrap();
    let part_1 = part_1(&input);
    let part_2 = part_2(&input);
    Answer::from_parts(part_1, part_2)
}
