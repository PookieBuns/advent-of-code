use std::path::Path;

use crate::Answer;
use std::str::FromStr;

fn part_1(input: &str) -> Option<impl std::string::ToString> {
    input
        .trim()
        .split(',')
        .filter_map(|s| s.split_once('-'))
        .map(|(a, b)| (i64::from_str(a).unwrap()..=i64::from_str(b).unwrap()))
        .map(|range| {
            range
                .filter(|num| {
                    let digits = num.ilog10() + 1;
                    if digits % 2 != 0 {
                        return false;
                    }
                    let center = digits / 2;
                    let num_str = num.to_string();
                    let first_half = &num_str[..center as usize];
                    let second_half = &num_str[center as usize..];
                    first_half == second_half
                })
                .sum::<i64>()
        })
        .sum::<i64>()
        .into()
}

fn part_2(input: &str) -> Option<impl std::string::ToString> {
    input
        .trim()
        .split(',')
        .filter_map(|s| s.split_once('-'))
        .map(|(a, b)| (i64::from_str(a).unwrap()..=i64::from_str(b).unwrap()))
        .map(|range| {
            range
                .filter(|num| {
                    let num_str = num.to_string();
                    let digits = (num.ilog10() + 1) as usize;
                    (1..=digits / 2)
                        .filter(|seq_len| digits % seq_len == 0)
                        .any(|seq_len| {
                            let seq_times = digits / seq_len;
                            let first_substr = &num_str[..seq_len];
                            (1..seq_times)
                                .map(|i| &num_str[i * seq_len..(i + 1) * seq_len])
                                .all(|substr| substr == first_substr)
                        })
                })
                .sum::<i64>()
        })
        .sum::<i64>()
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
