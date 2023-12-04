use std::fmt::Display;

fn part_1(input: &str) -> impl Display {
    "Unimplemented"
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
