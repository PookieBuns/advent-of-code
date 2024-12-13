use std::path::Path;

use cached::proc_macro::cached;
use regex::Regex;

use crate::Answer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Button {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Prize {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Machine {
    a: Button,
    b: Button,
    prize: Prize,
}

fn parse_to_machine(input: &str) -> Machine {
    let lines = input.lines().collect::<Vec<&str>>();
    let regex_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let regex_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let regex_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let a_captures = regex_a.captures(lines[0]).unwrap();
    let b_captures = regex_b.captures(lines[1]).unwrap();
    let prize_captures = regex_prize.captures(lines[2]).unwrap();
    let a = Button {
        x: a_captures[1].parse().unwrap(),
        y: a_captures[2].parse().unwrap(),
    };
    let b = Button {
        x: b_captures[1].parse().unwrap(),
        y: b_captures[2].parse().unwrap(),
    };
    let prize = Prize {
        x: prize_captures[1].parse().unwrap(),
        y: prize_captures[2].parse().unwrap(),
    };
    Machine { a, b, prize }
}

#[cached]
fn dp(x: i64, y: i64, machine: Machine) -> Option<i64> {
    if x > machine.prize.x || y > machine.prize.y {
        return None;
    }
    if x == machine.prize.x && y == machine.prize.y {
        return Some(0);
    }
    // button a
    let x_a = machine.a.x + x;
    let y_a = machine.a.y + y;
    let a = dp(x_a, y_a, machine).map(|x| x + 3);
    // button b
    let x_b = machine.b.x + x;
    let y_b = machine.b.y + y;
    let b = dp(x_b, y_b, machine).map(|x| x + 1);
    match (a, b) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn get_tokens(machine: Machine) -> Option<i64> {
    let a_y_a_x_ratio = machine.a.y as f64 / machine.a.x as f64;
    let b_x_scaled = machine.b.x as f64 * a_y_a_x_ratio;
    let machine_x_scaled = machine.prize.x as f64 * a_y_a_x_ratio;
    let x2_diff = machine.b.y as f64 - b_x_scaled;
    let machine_diff = machine.prize.y as f64 - machine_x_scaled;
    let x2 = machine_diff / x2_diff;
    let x2 = x2.round() as i64;
    let x1 = (machine.prize.x - x2 * machine.b.x) / machine.a.x;
    // verify that the solution is valid
    if machine.a.x * x1 + machine.b.x * x2 != machine.prize.x {
        return None;
    }
    if machine.a.y * x1 + machine.b.y * x2 != machine.prize.y {
        return None;
    }
    Some(x1 * 3 + x2)
}

fn part_1(input: &str) -> Option<i64> {
    input
        .split("\n\n")
        .map(parse_to_machine)
        .filter_map(get_tokens)
        .sum::<i64>()
        .into()
}

fn part_2(input: &str) -> Option<i64> {
    input
        .split("\n\n")
        .map(parse_to_machine)
        .map(|machine| Machine {
            a: Button {
                x: machine.a.y,
                y: machine.a.x,
            },
            b: Button {
                x: machine.b.y,
                y: machine.b.x,
            },
            prize: Prize {
                x: machine.prize.y + 10000000000000,
                y: machine.prize.x + 10000000000000,
            },
        })
        .filter_map(get_tokens)
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
