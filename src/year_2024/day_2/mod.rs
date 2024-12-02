use std::fmt::Display;

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

fn part_1(input: &str) -> impl Display {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|nums| is_safe(nums))
        .count()
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

fn part_2(input: &str) -> impl Display {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|nums| is_safe_with_remove(nums))
        .count()
}
pub fn solve() -> String {
    let input = include_str!("input.txt");
    let part_1_ans = part_1(input);
    let part_2_ans = part_2(input);
    format!("Part 1: {}\nPart 2: {}", part_1_ans, part_2_ans)
}
