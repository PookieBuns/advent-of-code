use std::collections::HashMap;
use std::fmt::Display;
use rayon::prelude::*;

fn extract_seeds(section: &str) -> Vec<usize> {
    let (_, seeds) = section.split_once(": ").unwrap();
    seeds
        .split(' ')
        .map(|seed| seed.parse::<usize>().unwrap())
        .collect()
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct SourceRange {
    start: usize,
    end: usize,
}

fn convert_section_to_map(section: &str) -> HashMap<SourceRange, i64> {
    let lines = section.split('\n').skip(1);
    let mut map = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let directions: Vec<usize> = line
            .split(' ')
            .map(|dir| dir.parse::<usize>().unwrap())
            .collect();
        let (dest, source, range) = (directions[0], directions[1], directions[2]);
        let diff = dest as i64 - source as i64;
        let range = SourceRange {
            start: source,
            end: source + range,
        };
        map.insert(range, diff);
    }
    map
}

fn map_to_destination(source_val: usize, map: &HashMap<SourceRange, i64>) -> usize {
    for (range, diff) in map {
        if range.start <= source_val && source_val < range.end {
            return (source_val as i64 + diff) as usize;
        }
    }
    source_val
}

fn part_1(input: &str) -> impl Display {
    let mut sections = input.split("\n\n");
    let seed_section = sections.next().unwrap();
    let mut seeds = extract_seeds(seed_section);
    for section in sections {
        let section_map = convert_section_to_map(section);
        seeds = seeds
            .into_iter()
            .map(|seed| map_to_destination(seed, &section_map))
            .collect();
    }
    seeds.into_iter().min().unwrap()
}

fn extract_seeds_part_2(section: &str) -> Vec<usize> {
    let (_, ranges) = section.split_once(": ").unwrap();
    let ranges: Vec<usize> = ranges
        .split(' ')
        .map(|seed| seed.parse::<usize>().unwrap())
        .collect();
    let seeds: Vec<usize> = ranges.par_chunks(2).flat_map(|range| {
        let (low, high) = (range[0], range[0] + range[1]);
        let mut seeds = Vec::new();
        for i in low..high {
            seeds.push(i);
        }
        seeds
    }).collect();
    println!("seed len: {}", seeds.len());
    seeds
}

fn part_2(input: &str) -> impl Display {
    let mut sections = input.split("\n\n");
    let seed_section = sections.next().unwrap();
    let mut seeds = extract_seeds_part_2(seed_section);
    for section in sections {
        println!("mapping seeds");
        let section_map = convert_section_to_map(section);
        seeds = seeds
            .into_par_iter()
            .map(|seed| map_to_destination(seed, &section_map))
            .collect();
        println!("done mapping seeds");
    }
    seeds.into_par_iter().min().unwrap()
}
pub fn solve() -> String {
    let input = include_str!("input.txt");
    let part_1_ans = part_1(input);
    let part_2_ans = part_2(input);
    format!("Part 1: {}\nPart 2: {}", part_1_ans, part_2_ans)
}
