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

pub fn solve() -> String {
    let input = include_str!("input.txt");
    let mut ans = 0;
    for (i, line) in input.lines().enumerate() {
        if is_possible(line) {
            ans += i + 1;
        }
    }
    ans.to_string()
}
