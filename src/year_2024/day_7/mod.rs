use std::path::Path;

use crate::Answer;

#[derive(Debug)]
struct Equation {
    total: i64,
    parts: Vec<i64>,
}

fn convert_to_equation(line: &str) -> Equation {
    let (left, right) = line.split_once(": ").unwrap();
    let total: i64 = left.parse().unwrap();
    let parts: Vec<i64> = right
        .split(" ")
        .map(|sub| sub.parse::<i64>().unwrap())
        .collect();
    Equation { total, parts }
}

fn backtrack(cur: i64, target: i64, arr: &[i64], i: usize, include_concat: bool) -> bool {
    if cur > target {
        return false;
    }
    if i == arr.len() {
        return cur == target;
    }
    let op1 = backtrack(cur + arr[i], target, arr, i + 1, include_concat);
    if op1 {
        return true;
    }
    let op2 = backtrack(cur * arr[i], target, arr, i + 1, include_concat);
    if op2 {
        return true;
    }
    if include_concat {
        let op3 = backtrack(
            cur * 10i64.pow(arr[i].to_string().len() as u32) + arr[i],
            target,
            arr,
            i + 1,
            include_concat,
        );
        if op3 {
            return true;
        }
    }
    false
}

fn can_work(equation: &Equation, include_concat: bool) -> bool {
    backtrack(0, equation.total, &equation.parts, 0, include_concat)
}

fn part_1(input: &str) -> Option<i64> {
    input
        .lines()
        .map(convert_to_equation)
        .filter(|equation| can_work(equation, false))
        .map(|equation| equation.total)
        .sum::<i64>()
        .into()
}

fn part_2(input: &str) -> Option<i64> {
    input
        .lines()
        .map(convert_to_equation)
        .filter(|equation| can_work(equation, true))
        .map(|equation| equation.total)
        .sum::<i64>()
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
