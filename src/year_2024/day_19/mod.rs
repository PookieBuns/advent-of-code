use std::{collections::HashMap, path::Path};

use crate::Answer;

fn get_patterns(input: &str) -> Vec<Vec<char>> {
    input.split(", ").map(|s| s.chars().collect()).collect()
}

fn get_designs(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn dp(patterns: &[Vec<char>], design: &[char], i: usize, cache: &mut HashMap<usize, bool>) -> bool {
    if i == design.len() {
        return true;
    }
    if let Some(&res) = cache.get(&i) {
        return res;
    }
    for pattern in patterns {
        if design.len() - i < pattern.len() {
            continue;
        }
        if design[i..i + pattern.len()] != *pattern {
            continue;
        }
        if dp(patterns, design, i + pattern.len(), cache) {
            cache.insert(i, true);
            return true;
        }
    }
    cache.insert(i, false);
    false
}

fn is_possible(patterns: &[Vec<char>], design: &[char]) -> bool {
    dp(patterns, design, 0, &mut HashMap::new())
}

fn part_1(input: &str) -> Option<i64> {
    let (patterns_str, design_str) = input.split_once("\n\n").unwrap();
    let patterns = get_patterns(patterns_str);
    let designs = get_designs(design_str);
    (designs
        .into_iter()
        .filter(|design| is_possible(&patterns, design))
        .count() as i64)
        .into()
}

fn dp_2(patterns: &[Vec<char>], design: &[char], i: usize, cache: &mut HashMap<usize, i64>) -> i64 {
    if i == design.len() {
        return 1;
    }
    if let Some(&res) = cache.get(&i) {
        return res;
    }
    let mut res = 0;
    for pattern in patterns {
        if design.len() - i < pattern.len() {
            continue;
        }
        if design[i..i + pattern.len()] != *pattern {
            continue;
        }
        let combos = dp_2(patterns, design, i + pattern.len(), cache);
        res += combos;
    }
    cache.insert(i, res);
    res
}

fn combos(patterns: &[Vec<char>], design: &[char]) -> i64 {
    dp_2(patterns, design, 0, &mut HashMap::new())
}

fn part_2(input: &str) -> Option<i64> {
    let (patterns_str, design_str) = input.split_once("\n\n").unwrap();
    let patterns = get_patterns(patterns_str);
    let designs = get_designs(design_str);
    (designs
        .into_iter()
        .map(|design| combos(&patterns, &design))
        .sum::<i64>())
    .into()
}
pub fn solve() -> Answer {
    let cur_dir = Path::new(file!()).parent().unwrap();
    let file_path = cur_dir.join("input.txt");
    let input = std::fs::read_to_string(file_path).unwrap();
    let part_1_start_time = std::time::Instant::now();
    let part_1 = part_1(&input);
    let part_1_elapsed_time = part_1_start_time.elapsed();
    println!("Done with part 1 in {:?}", part_1_elapsed_time);
    let part_2_start_time = std::time::Instant::now();
    let part_2 = part_2(&input);
    let part_2_elapsed_time = part_2_start_time.elapsed();
    println!("Done with part 2 in {:?}", part_2_elapsed_time);
    Answer::from_parts(part_1, part_2)
}
