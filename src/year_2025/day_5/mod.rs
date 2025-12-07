use std::path::Path;

use crate::Answer;

fn part_1(input: &str) -> Option<impl std::string::ToString> {
    let (first, second) = input.split_once("\n\n").unwrap();
    let intervals = first
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            (start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();
    let ingredients = second
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    ingredients
        .iter()
        .filter(|i| {
            intervals
                .iter()
                .any(|(start, end)| *i >= start && *i <= end)
        })
        .count()
        .into()
}

fn merge_intervals(mut intervals: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    intervals.sort();
    intervals.into_iter().fold(Vec::new(), |mut acc, (start, end)| {
        if let Some((_, last_end)) = acc.last_mut() {
            if start <= *last_end {
                *last_end = (*last_end).max(end);               
                return acc;
            }
        }       
        acc.push((start, end));
        acc

    })
}

fn part_2(input: &str) -> Option<impl std::string::ToString> {
    let (first, _second) = input.split_once("\n\n").unwrap();
    let intervals = first
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            (start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();
    let merged = merge_intervals(intervals);
    merged.iter().map(|(start, end)| end - start + 1).sum::<i64>().into()
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
