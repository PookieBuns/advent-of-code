use std::fmt::Display;
use std::collections::HashMap;

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

fn part_1(input: &str) -> impl Display {
    let (mut left_nums, mut right_nums) = get_nums(input);
    left_nums.sort();
    right_nums.sort();
    left_nums
        .iter()
        .zip(right_nums.iter())
        .fold(0, |acc, (left, right)| acc + (left - right).abs())
}
fn part_2(input: &str) -> impl Display {
    let (left_nums, right_nums) = get_nums(input);
    let right_counter = right_nums.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });
    left_nums.iter().fold(0, |acc, num| {
        acc + num * right_counter.get(num).unwrap_or(&0)
    })
}
pub fn solve() -> String {
    let input = include_str!("input.txt");
    let part_1_ans = part_1(input);
    let part_2_ans = part_2(input);
    format!("Part 1: {}\nPart 2: {}", part_1_ans, part_2_ans)
}
