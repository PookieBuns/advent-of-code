const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn is_possible(line: &str) -> bool {
    let (_, games) = line.split_once(": ").expect("Invalid input");
    for round in games.split("; ") {
        for ball in round.split(", ") {
            let (number_str, color) = ball.split_once(' ').expect("Invalid input");
            let number: u32 = number_str
                .parse()
                .unwrap_or_else(|_| panic!("Invalid input {number_str}"));
            match color {
                "red" => {
                    if number > MAX_RED {
                        return false;
                    }
                }
                "green" => {
                    if number > MAX_GREEN {
                        return false;
                    }
                }
                "blue" => {
                    if number > MAX_BLUE {
                        return false;
                    }
                }
                _ => panic!("Invalid input"),
            }
        }
    }
    true
}

fn part_1(input: &str) -> u32 {
    let mut ans = 0;
    for (i, line) in input.lines().enumerate() {
        if is_possible(line) {
            ans += i + 1;
        }
    }
    ans as u32
}

fn power_of_cubes(line: &str) -> u32 {
    let (_, games) = line.split_once(": ").expect("Invalid input");
    let mut max_blue = 0;
    let mut max_green = 0;
    let mut max_red = 0;
    for round in games.split("; ") {
        for ball in round.split(", ") {
            let (number_str, color) = ball.split_once(' ').expect("Invalid input");
            let number: u32 = number_str
                .parse()
                .unwrap_or_else(|_| panic!("Invalid input {number_str}"));
            match color {
                "red" => {
                    max_red = std::cmp::max(max_red, number);
                }
                "green" => {
                    max_green = std::cmp::max(max_green, number);
                }
                "blue" => {
                    max_blue = std::cmp::max(max_blue, number);
                }
                _ => panic!("Invalid input"),
            }
        }
    }
    max_red * max_green * max_blue
}

fn part_2(input: &str) -> u32 {
    let mut ans = 0;
    for line in input.lines() {
        ans += power_of_cubes(line);
    }
    ans
}

pub fn solve() -> String {
    let input = include_str!("input.txt");
    let part_1_ans = part_1(input);
    let part_2_ans = part_2(input);
    format!("Part 1: {}\nPart 2: {}", part_1_ans, part_2_ans)
}
