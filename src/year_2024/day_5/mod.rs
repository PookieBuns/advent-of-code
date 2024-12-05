use std::{collections::HashMap, path::Path};

use crate::Answer;

fn create_adj_mat(input: &str) -> HashMap<i32, Vec<i32>> {
    input
        .lines()
        .map(|line| {
            let (before_str, after_str) = line.split_once("|").unwrap();
            let before: i32 = before_str.parse().unwrap();
            let after: i32 = after_str.parse().unwrap();
            (before, after)
        })
        .fold(HashMap::new(), |mut acc, (before, after)| {
            acc.entry(before).or_default().push(after);
            acc
        })
}

fn part_1(input: &str) -> Option<i32> {
    let (first, second) = input.split_once("\n\n").unwrap();
    let adj_mat = create_adj_mat(first);
    second
        .lines()
        // convert to list of numbers
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        // find valid lists
        .filter(|nums| {
            nums.iter().enumerate().all(|(i, before)| {
                nums.iter()
                    .skip(i + 1)
                    .all(|&after| !adj_mat.get(&after).unwrap_or(&vec![]).contains(before))
            })
        })
        // find the middle number
        .map(|nums| nums[nums.len() / 2])
        .sum::<i32>()
        .into()
}

fn part_2(input: &str) -> Option<i32> {
    let (first, second) = input.split_once("\n\n").unwrap();
    let adj_mat = create_adj_mat(first);
    second
        .lines()
        // convert to list of numbers
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        // find invalid lists
        .filter(|nums| {
            nums.iter().enumerate().any(|(i, before)| {
                nums.iter()
                    .skip(i + 1)
                    .any(|&after| adj_mat.get(&after).unwrap_or(&vec![]).contains(before))
            })
        })
        // reoganize the list to be valid
        .map(|nums| {
            let mut q = nums.clone();
            let mut new_nums = Vec::new();
            while !q.is_empty() {
                let available_nums = q
                    .iter()
                    .filter(|&before| {
                        q.iter()
                            .all(|&after| !adj_mat.get(&after).unwrap_or(&vec![]).contains(before))
                    })
                    .copied()
                    .collect::<Vec<i32>>();
                q.retain(|&x| !available_nums.contains(&x));
                new_nums.extend(available_nums);
            }
            new_nums
        })
        // find the middle number
        .map(|nums| nums[nums.len() / 2])
        .sum::<i32>()
        .into()
}
pub fn solve() -> Answer {
    let cur_dir = Path::new(file!()).parent().unwrap();
    let file_path = cur_dir.join("input.txt");
    let input = std::fs::read_to_string(file_path).unwrap();
    let part_1 = part_1(&input);
    let part_2 = part_2(&input);
    Answer { part_1, part_2 }
}
