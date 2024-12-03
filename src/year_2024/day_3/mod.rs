use std::path::Path;

use regex::Regex;

use crate::Answer;

fn part_1(input: &str) -> Option<i32> {
    let re = Regex::new(r"mul\((\d{0,3}),(\d{0,3})\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let left: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
            let right: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
            left * right
        })
        .sum::<i32>()
        .into()
}

fn part_2(input: &str) -> Option<i32> {
    let re = Regex::new(r"(?:mul\((\d{0,3}),(\d{0,3})\)|do\(\)|don't\(\))").unwrap();
    re.captures_iter(input)
        .scan(1, |multiplier, cap| {
            if cap.get(0).unwrap().as_str() == "do()" {
                *multiplier = 1;
                Some(0)
            } else if cap.get(0).unwrap().as_str() == "don't()" {
                *multiplier = 0;
                Some(0)
            } else {
                let left: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
                let right: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
                Some(left * right * *multiplier)
            }
        })
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
