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

fn get_digits_with_letters(line: &str) -> Digits {
    const DIGIT_WORDS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut first = None;
    let mut last = None;
    for (i, char) in line.char_indices() {
        if char.is_ascii_digit() {
            let digit = char.to_digit(10).unwrap();
            if first.is_none() {
                first = Some(digit);
            }
            last = Some(digit);
        } else {
            for (j, &digit_word) in DIGIT_WORDS.iter().enumerate() {
                let digit = (j + 1) as u32;
                if line[i..].starts_with(digit_word) {
                    if first.is_none() {
                        first = Some(digit);
                    }
                    last = Some(digit);
                }
            }
        }
    }
    Digits {
        first: first.unwrap(),
        last: last.unwrap(),
    }
}

fn part_1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let digits = get_digits(line);
        sum += digits.first * 10 + digits.last;
    }
    sum
}

fn part_2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let digits = get_digits_with_letters(line);
        sum += digits.first * 10 + digits.last;
    }
    sum
}

pub fn solve() -> String {
    let input = include_str!("input.txt");
    let part_1_sum = part_1(input);
    let part_2_sum = part_2(input);
    format!("Part 1: {part_1_sum}\nPart 2: {part_2_sum}")
}
