struct Digits {
    first: u32,
    last: u32,
}

fn get_digits(line: &str) -> Digits {
    let mut first = None;
    let mut last = None;
    for char in line.chars() {
        if char.is_ascii_digit() {
            if first.is_none() {
                first = Some(char.to_digit(10).unwrap());
            }
            last = Some(char.to_digit(10).unwrap());
        }
    }
    Digits {
        first: first.unwrap(),
        last: last.unwrap(),
    }
}
pub fn solve() -> String {
    let input = include_str!("input.txt");
    let mut sum = 0;
    for line in input.lines() {
        let digits = get_digits(line);
        sum += digits.first * 10 + digits.last;
    }
    sum.to_string()
}
