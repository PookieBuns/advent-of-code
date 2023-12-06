use rayon::prelude::*;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Range;
use tracing::{info, warn};

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

fn extract_ranges(section: &str) -> Vec<Range<usize>> {
    let (_, ranges) = section.split_once(": ").unwrap();
    let ranges: Vec<usize> = ranges
        .split(' ')
        .map(|seed| seed.parse::<usize>().unwrap())
        .collect();
    ranges
        .chunks(2)
        .map(|range| range[0]..range[0] + range[1])
        .collect()
}

fn convert_section_to_map_reverse(section: &str) -> HashMap<Range<usize>, i64> {
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
        let range = dest..dest + range;
        map.insert(range, -diff);
    }
    map
}

fn map_to_seed(location: i64, section_maps: &Vec<HashMap<Range<usize>, i64>>) -> i64 {
    let mut location = location;
    for section_map in section_maps {
        for (range, diff) in section_map {
            if range.contains(&(location as usize)) {
                location += diff;
                break;
            }
        }
    }
    location
}

fn find_lowest_in_chunk(
    start: usize,
    end: usize,
    section_maps: &Vec<HashMap<Range<usize>, i64>>,
    seed_ranges: &Vec<Range<usize>>,
) -> Option<i64> {
    for i in start..end {
        let seed = map_to_seed(i as i64, section_maps);
        for range in seed_ranges {
            if range.contains(&(seed as usize)) {
                return Some(i as i64);
            }
        }
    }
    None
}

fn find_lowest(
    section_maps: &Vec<HashMap<Range<usize>, i64>>,
    seed_ranges: &Vec<Range<usize>>,
) -> i64 {
    let chunk_size = 10_000;
    let chunk_count = 100;
    let mut offset = 0;
    for _ in 0.. {
        info!(
            "searching from {} to {}",
            offset,
            offset + chunk_count * chunk_size
        );
        let mut search_ranges = Vec::new();
        for chunk in 0..chunk_count {
            let start = offset + chunk * chunk_size;
            let end = offset + (chunk + 1) * chunk_size;
            search_ranges.push((start, end));
        }
        let results = search_ranges
            .par_iter()
            .filter_map(|(start, end)| {
                find_lowest_in_chunk(*start, *end, section_maps, seed_ranges)
            })
            .min();
        if let Some(result) = results {
            return result;
        }
        warn!("no results found");
        offset += chunk_count * chunk_size;
    }
    0
}

fn part_2(input: &str) -> impl Display {
    let mut sections = input.split("\n\n");
    let seed_section = sections.next().unwrap();
    let seed_ranges = extract_ranges(seed_section);
    let mut section_maps: Vec<_> = sections.map(convert_section_to_map_reverse).collect();
    section_maps.reverse();
    find_lowest(&section_maps, &seed_ranges)
}
pub fn solve() -> String {
    let input = include_str!("input.txt");
    let part_1_ans = part_1(input);
    let part_2_ans = part_2(input);
    format!("Part 1: {}\nPart 2: {}", part_1_ans, part_2_ans)
}
