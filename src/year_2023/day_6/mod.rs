use std::fmt::Display;

fn get_nums(line: &str) -> Vec<i64> {
    let (_name, line) = line.split_once(": ").unwrap();
    line.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn get_distance(hold_time: i64, total_time: i64) -> i64 {
    let travel_time = total_time - hold_time;
    let speed = hold_time;
    speed * travel_time
}

fn get_ways(time: i64, max_distance: i64) -> i64 {
    let median_time = (time + 1) / 2;
    let mut l = 0;
    let mut r = median_time;
    while l < r {
        let m = (l + r) / 2;
        let distance = get_distance(m, time);
        if distance <= max_distance {
            l = m + 1;
        } else {
            r = m;
        }
    }
    let ans = median_time - l;
    if time % 2 == 0 {
        ans * 2 + 1
    } else {
        ans * 2
    }
}

fn part_1(input: &str) -> impl Display {
    let mut lines = input.lines();
    let time_line = lines.next().unwrap();
    let distance_line = lines.next().unwrap();
    let times = get_nums(time_line);
    let distances = get_nums(distance_line);
    times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| get_ways(*t, *d))
        .product::<i64>()
}

fn get_num(line: &str) -> i64 {
    let (_name, line) = line.split_once(": ").unwrap();
    line.replace(' ', "").parse::<i64>().unwrap()
}

fn part_2(input: &str) -> impl Display {
    let mut lines = input.lines();
    let time_line = lines.next().unwrap();
    let distance_line = lines.next().unwrap();
    let time = get_num(time_line);
    let distance = get_num(distance_line);
    get_ways(time, distance)
}

pub fn solve() -> String {
    let input = include_str!("input.txt");
    let part_1_ans = part_1(input);
    let part_2_ans = part_2(input);
    format!("Part 1: {}\nPart 2: {}", part_1_ans, part_2_ans)
}
