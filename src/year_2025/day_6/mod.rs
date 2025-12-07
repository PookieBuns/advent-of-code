use std::path::Path;

use crate::Answer;

fn part_1(input: &str) -> Option<impl std::string::ToString> {
    let lines = input.lines().collect::<Vec<_>>();
    let nums = lines[0..lines.len() - 1]
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let transposed = (0..nums[0].len())
        .map(|i| nums.iter().map(|row| row[i]).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let operators = lines[lines.len() - 1]
        .split_whitespace()
        .collect::<Vec<_>>();
    std::iter::zip(operators, transposed)
        .map(|(op, nums)| match op {
            "*" => nums.iter().product::<i64>(),
            "+" => nums.iter().sum::<i64>(),
            _ => unreachable!(),
        })
        .sum::<i64>()
        .into()
}

fn part_2(input: &str) -> Option<impl std::string::ToString> {
    let lines = input.lines().collect::<Vec<_>>();
    let num_rows = lines.len();
    let (mut start_indices, operators): (Vec<_>, Vec<_>) = lines[num_rows - 1]
        .chars()
        .enumerate()
        .filter(|(_i, c)| !c.is_whitespace())
        .unzip();
    start_indices.push(lines[0].len() + 1);
    let num_lines = &lines[0..lines.len() - 1];
    let nums = start_indices
        .windows(2)
        .map(|w| {
            let (start, end) = (w[0], w[1]);
            (start..end - 1)
                .map(|j| {
                    // dbg!(j);
                    num_lines.iter().fold(0, |acc, line| {
                        // dbg!(&line);
                        match line.chars().nth(j).unwrap() {
                            ' ' => acc,
                            _ => {
                                acc * 10 + line.chars().nth(j).unwrap().to_digit(10).unwrap() as i64
                            }
                        }
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    std::iter::zip(operators, nums)
        .map(|(op, nums)| match op {
            '*' => nums.iter().product::<i64>(),
            '+' => nums.iter().sum::<i64>(),
            _ => unreachable!(),
        })
        .sum::<i64>()
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
