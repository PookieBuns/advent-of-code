use std::collections::HashSet;
use std::fmt::Display;

fn get_score(line: &str) -> u32 {
    let (_card_title, card_info) = line.split_once(": ").unwrap();
    let (winning_numbers, my_ticket) = card_info.split_once(" | ").unwrap();
    let winning_numbers: HashSet<u32> = winning_numbers
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let my_ticket: HashSet<u32> = my_ticket
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    my_ticket
        .iter()
        .filter(|num| winning_numbers.contains(num))
        .fold(0, |acc, _| match acc {
            0 => 1,
            _ => acc * 2,
        })
}

fn part_1(input: &str) -> impl Display {
    let mut ans = 0;
    for line in input.lines() {
        ans += get_score(line);
    }
    ans
}
fn part_2(input: &str) -> impl Display {
    "Unimplemented"
}
pub fn solve() -> String {
    let input = include_str!("input.txt");
    let part_1_ans = part_1(input);
    let part_2_ans = part_2(input);
    format!("Part 1: {}\nPart 2: {}", part_1_ans, part_2_ans)
}
