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

fn get_matching_number_count(line: &str) -> u32 {
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
        .count() as u32
}

fn part_2(input: &str) -> impl Display {
    let card_count = input.lines().count();
    let mut ans_vec = vec![1; card_count];
    input.lines().enumerate().for_each(|(i, line)| {
        let matching_number_count = get_matching_number_count(line);
        let high = std::cmp::min(i + 1 + matching_number_count as usize, card_count);
        for j in i + 1..high {
            ans_vec[j] += ans_vec[i];
        }
    });
    ans_vec.iter().sum::<u32>()
}

pub fn solve() -> String {
    let input = include_str!("input.txt");
    let part_1_ans = part_1(input);
    let part_2_ans = part_2(input);
    format!("Part 1: {}\nPart 2: {}", part_1_ans, part_2_ans)
}
