use std::{collections::HashMap, path::Path};

use crate::Answer;

fn get_nums(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("   ").unwrap();
            let left_num: i32 = left.parse().unwrap();
            let right_num: i32 = right.parse().unwrap();
            (left_num, right_num)
        })
        .unzip()
}

fn part_1(input: &str) -> Option<i32> {
    let (mut left_nums, mut right_nums) = get_nums(input);
    left_nums.sort();
    right_nums.sort();
    left_nums
        .iter()
        .zip(right_nums.iter())
        .fold(0, |acc, (left, right)| acc + (left - right).abs())
        .into()
}
fn part_2(input: &str) -> Option<i32> {
    let (left_nums, right_nums) = get_nums(input);
    let right_counter = right_nums.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });
    left_nums
        .iter()
        .fold(0, |acc, num| {
            acc + num * right_counter.get(num).unwrap_or(&0)
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
