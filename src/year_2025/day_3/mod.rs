use std::path::Path;

use crate::Answer;

fn part_1(input: &str) -> Option<impl std::string::ToString> {
    input
        .lines()
        .map(|line| {
            let digits = line.len();
            let nums: Vec<_> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
            let digits_to_use = 2;
            let mut ans = 0;
            let mut left = 0;
            for remain_digits_to_use in (0..digits_to_use).rev() {
                let right = digits - remain_digits_to_use;
                let selectable = &nums[left..right];
                let max = selectable.iter().max().unwrap();
                let max_index = selectable.iter().position(|&x| x == *max).unwrap();
                left += max_index + 1;
                ans = ans * 10 + max;
            }
            ans
        })
        .sum::<u32>()
        .into()
}

fn part_2(input: &str) -> Option<impl std::string::ToString> {
    input
        .lines()
        .map(|line| {
            let digits = line.len();
            let nums: Vec<_> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect();
            let digits_to_use = 12;
            let mut ans = 0;
            let mut left = 0;
            for remain_digits_to_use in (0..digits_to_use).rev() {
                let right = digits - remain_digits_to_use;
                let selectable = &nums[left..right];
                let max = selectable.iter().max().unwrap();
                let max_index = selectable.iter().position(|&x| x == *max).unwrap();
                left += max_index + 1;
                ans = ans * 10 + max;
            }
            ans
        })
        .sum::<u64>()
        .into()
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
