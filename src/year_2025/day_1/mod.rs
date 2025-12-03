use std::path::Path;
use std::str::FromStr;

use crate::Answer;

fn get_turn(line: &str) -> i32 {
    let (direction, num_str) = line.split_at(1);
    let num = i32::from_str(num_str).unwrap();
    match direction {
        "L" => -num,
        "R" => num,
        _ => panic!("Unknown direction"),
    }
}

fn part_1(input: &str) -> Option<usize> {
    input
        .lines()
        .scan(50, |dial, line| {
            let turn = get_turn(line);
            *dial = (*dial + turn).rem_euclid(100);
            Some(*dial)
        })
        .filter(|&x| x == 0)
        .count()
        .into()
}

fn part_2(input: &str) -> Option<i32> {
    input
        .lines()
        .scan(50, |dial, line| {
            let turn = get_turn(line);
            let start = *dial;
            let end = start + turn;
            let mut clicks = end.div_euclid(100).abs();
            let end_normalized = end.rem_euclid(100);
            if turn < 0 {
                if start == 0 {
                    clicks -= 1;
                }
                if end_normalized == 0 {
                    clicks += 1;
                }
            }
            *dial = end_normalized;
            Some(clicks)
        })
        .sum::<i32>()
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
