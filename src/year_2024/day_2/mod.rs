use std::path::Path;

use crate::Answer;

fn is_safe(nums: &[i32]) -> bool {
    let first = nums[0];
    let second = nums[1];
    let increasing = first < second;
    nums.windows(2).all(|pair| {
        let left = pair[0];
        let right = pair[1];
        if increasing {
            left < right && right - left <= 3
        } else {
            left > right && left - right <= 3
        }
    })
}

fn part_1(input: &str) -> Option<i32> {
    i32::try_from(
        input
            .lines()
            .map(|line| {
                line.split(" ")
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<i32>>()
            })
            .filter(|nums| is_safe(nums))
            .count(),
    )
    .expect("Failed to convert to i32")
    .into()
}

fn is_safe_with_remove(nums: &[i32]) -> bool {
    if is_safe(nums) {
        return true;
    }
    for skip_index in 0..nums.len() {
        let mut new_nums = nums.to_vec();
        new_nums.remove(skip_index);
        if is_safe(&new_nums) {
            return true;
        }
    }
    false
}

fn part_2(input: &str) -> Option<i32> {
    i32::try_from(
        input
            .lines()
            .map(|line| {
                line.split(" ")
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<i32>>()
            })
            .filter(|nums| is_safe_with_remove(nums))
            .count(),
    )
    .expect("Failed to convert to i32")
    .into()
}
pub fn solve() -> Answer {
    let cur_dir = Path::new(file!()).parent().unwrap();
    let input = std::fs::read_to_string(cur_dir.join("input.txt")).unwrap();
    let part_1 = part_1(&input);
    let part_2 = part_2(&input);
    Answer { part_1, part_2 }
}
