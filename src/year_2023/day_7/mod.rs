use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<char>,
    groups: Vec<i32>,
    bid: i32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..self.groups.len() {
            if self.groups[i] != other.groups[i] {
                return self.groups[i].cmp(&other.groups[i]);
            }
        }
        for i in 0..self.cards.len() {
            if self.cards[i] != other.cards[i] {
                return get_points(self.cards[i]).cmp(&get_points(other.cards[i]));
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_points(char: char) -> i32 {
    match char {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => char.to_digit(10).unwrap() as i32,
    }
}

fn parse_to_hand(line: &str) -> Hand {
    let (hand_str, bid_str) = line.split_once(' ').unwrap();
    let cards = hand_str.chars().collect::<Vec<char>>();
    let bid = bid_str.parse::<i32>().unwrap();
    let mut char_count = HashMap::new();
    for c in &cards {
        *char_count.entry(c).or_insert(0) += 1;
    }
    let mut groups: Vec<i32> = char_count.values().copied().collect();
    groups.sort();
    groups.reverse();
    Hand { cards, groups, bid }
}

fn part_1(input: &str) -> impl Display {
    let mut hands: Vec<Hand> = input.lines().map(parse_to_hand).collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i as i32 + 1))
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
