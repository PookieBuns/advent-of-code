use std::collections::{HashMap, HashSet};

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn look_for_symbol(i: usize, j: usize, char_vec: &Vec<Vec<char>>) -> bool {
    for (dx, dy) in DIRECTIONS {
        let x = i as i32 + dx;
        let y = j as i32 + dy;
        if x < 0 || y < 0 || x >= char_vec.len() as i32 || y >= char_vec[0].len() as i32 {
            continue;
        }
        let char = char_vec[x as usize][y as usize];
        if !char.is_ascii_digit() && char != '.' {
            return true;
        }
    }
    false
}

fn part_1(input: &str) -> u32 {
    let char_vec = convert_to_char_vector(input);
    let mut ans = 0;
    let mut number = None;
    let mut found_symbol = false;
    for (i, line) in char_vec.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if char.is_ascii_digit() {
                let digit = char.to_digit(10).unwrap();
                match number {
                    None => number = Some(digit),
                    Some(n) => number = Some(n * 10 + digit),
                }
                found_symbol |= look_for_symbol(i, j, &char_vec);
            } else {
                if found_symbol {
                    ans += number.unwrap();
                }
                number = None;
                found_symbol = false;
            }
        }
    }
    ans
}

struct Number {
    value: u32,
    gear: HashSet<Gear>,
}

#[derive(Hash, Eq, PartialEq)]
struct Gear {
    x: i32,
    y: i32,
}

fn get_gears(i: usize, j: usize, char_vec: &Vec<Vec<char>>) -> HashSet<Gear> {
    let mut gears = HashSet::new();
    for (dx, dy) in DIRECTIONS {
        let x = i as i32 + dx;
        let y = j as i32 + dy;
        if x < 0 || y < 0 || x >= char_vec.len() as i32 || y >= char_vec[0].len() as i32 {
            continue;
        }
        let char = char_vec[x as usize][y as usize];
        if char == '*' {
            gears.insert(Gear { x, y });
        }
    }
    gears
}

fn parse_numbers(numbers: Vec<Number>) -> u32 {
    let mut ans = 0;
    let mut gears = HashMap::new();
    for number in &numbers {
        for gear in &number.gear {
            gears.entry(gear).or_insert_with(Vec::new).push(number.value);
        }
    }
    for (_, numbers) in gears {
        if numbers.len() == 2 {
            let first = numbers[0];
            let second = numbers[1];
            ans += first * second;
        }
    }
    ans
}

fn part_2(input: &str) -> u32 {
    let char_vec = convert_to_char_vector(input);
    let mut number = None;
    let mut gears = HashSet::new();
    let mut numbers = Vec::new();
    for (i, line) in char_vec.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if char.is_ascii_digit() {
                let digit = char.to_digit(10).unwrap();
                match number {
                    None => number = Some(digit),
                    Some(n) => number = Some(n * 10 + digit),
                }
                gears.extend(get_gears(i, j, &char_vec));
            } else {
                if let Some(number) = number {
                    numbers.push(Number {
                        value: number,
                        gear: gears,
                    });
                }
                number = None;
                gears = HashSet::new();
            }
        }
    }
    parse_numbers(numbers)
}

fn convert_to_char_vector(input: &str) -> Vec<Vec<char>> {
    let mut vec = Vec::new();
    for line in input.lines() {
        let mut cur_vec = Vec::new();
        for char in line.chars() {
            cur_vec.push(char);
        }
        vec.push(cur_vec);
    }
    vec
}
pub fn solve() -> String {
    let input = include_str!("input.txt");
    let part_1_ans = part_1(input);
    let part_2_ans = part_2(input);
    format!("Part 1: {}\nPart 2: {}", part_1_ans, part_2_ans)
}
