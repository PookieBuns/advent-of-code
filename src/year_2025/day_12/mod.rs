use std::path::Path;

use crate::Answer;

#[derive(Debug)]
struct Present {
    area: usize,
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    quantities: Vec<usize>,
}

#[derive(Debug)]
struct Problem {
    presents: Vec<Present>,
    regions: Vec<Region>,
}

fn parse_input(input: &str) -> Problem {
    let mut sections: Vec<&str> = input.split("\n\n").collect();
    let regions_section = sections.pop().unwrap();
    let presents_secontion = sections;
    let presents = presents_secontion
        .iter()
        .map(|present_str| {
            let mut lines = present_str.lines();
            let _index_line = lines.next().unwrap();
            let area = lines
                .map(|line| line.chars().filter(|&c| c == '#').count())
                .sum();
            Present { area }
        })
        .collect();
    let regions = regions_section
        .lines()
        .map(|region_str| {
            let (dim_str, quantities_str) = region_str.split_once(':').unwrap();
            let quantities: Vec<usize> = quantities_str
                .trim()
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect();
            let (width_str, height_str) = dim_str.split_once('x').unwrap();
            let width: usize = width_str.parse().unwrap();
            let height: usize = height_str.parse().unwrap();
            Region {
                width,
                height,
                quantities,
            }
        })
        .collect();
    Problem { presents, regions }
}

fn can_fit(region: &Region, presents: &[Present]) -> bool {
    let region_area = region.width * region.height;
    let shape_area = region
        .quantities
        .iter()
        .enumerate()
        .map(|(i, &qty)| presents[i].area * qty)
        .sum::<usize>();
    if shape_area > region_area {
        return false;
    }
    let num_3_by_3 = (region.width / 3) * (region.height / 3);
    if region.quantities.iter().sum::<usize>() < num_3_by_3 {
        return true;
    }
    true
}

fn part_1(input: &str) -> Option<impl std::string::ToString> {
    let problem = parse_input(input);
    let Problem { presents, regions } = problem;
    regions
        .iter()
        .filter(|region| can_fit(region, &presents))
        .count()
        .into()
}

fn part_2(_input: &str) -> Option<impl std::string::ToString> {
    None::<i32>
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
