use good_lp::{default_solver, variable, Expression, ProblemVariables};
use good_lp::{Solution, SolverModel};
use regex::Regex;
use std::path::Path;

const LINE_REGEX: &str = r"^\s*\[([^\]]+)\]\s*((?:\([^)]*\)\s*)+)\{\s*([^}]+)\s*\}$";
const WIRING_REGEX: &str = r"\(([^\)]+)\)";

use crate::Answer;

#[derive(Debug)]
struct LightMachine {
    light: u32,
    wirings: Vec<u32>,
}

impl LightMachine {
    fn least_presses(&self, cur: u32, i: usize) -> u32 {
        if cur == self.light {
            return 0;
        }
        if i >= self.wirings.len() {
            return u32::MAX - 1000;
        }
        // Try not pressing this wiring
        let no_press = self.least_presses(cur, i + 1);
        // Try pressing this wiring
        let with_press = 1 + self.least_presses(cur ^ self.wirings[i], i + 1);
        no_press.min(with_press)
    }
}

fn part_1(input: &str) -> Option<impl std::string::ToString> {
    let machines = input
        .lines()
        .map(|line| {
            let re = Regex::new(LINE_REGEX).unwrap();
            let caps = re.captures(line).unwrap();
            let light = &caps[1];
            let light_bits = light
                .chars()
                .rev()
                .fold(0u32, |acc, c| (acc << 1) | if c == '#' { 1 } else { 0 });
            let wirings = &caps[2];
            let wiring_re = Regex::new(WIRING_REGEX).unwrap();
            let wiring_bits = wiring_re
                .captures_iter(wirings)
                .map(|wiring_cap| {
                    let wiring = &wiring_cap[1];
                    wiring.split(',').rev().fold(0u32, |acc, c| {
                        let num = c.parse::<u32>().unwrap();
                        acc | (1 << num)
                    })
                })
                .collect::<Vec<_>>();
            LightMachine {
                light: light_bits,
                wirings: wiring_bits,
            }
        })
        .collect::<Vec<_>>();
    machines
        .iter()
        .map(|machine| machine.least_presses(0, 0))
        .sum::<u32>()
        .into()
}

#[derive(Debug)]
struct JoltageMachine {
    wirings: Vec<Vec<usize>>,
    requirement: Vec<usize>,
}

impl JoltageMachine {
    fn least_presses(&self) -> u32 {
        // --- variables ---
        let mut problem_vars = ProblemVariables::new();
        let presses: Vec<_> = (0..self.wirings.len())
            .map(|_| problem_vars.add(variable().integer().min(0)))
            .collect();

        let problem = self.requirement.iter().enumerate().fold(
            problem_vars
                .minimise(presses.iter().cloned().sum::<Expression>())
                .using(default_solver),
            |problem, (switch, &req)| {
                let lhs = std::iter::zip(self.wirings.iter(), presses.iter())
                    .filter_map(|(wiring, &press)| wiring.contains(&switch).then_some(press))
                    .sum::<Expression>();
                problem.with(lhs.eq(req as i32))
            },
        );

        // --- solve ---
        let solution = problem.solve().unwrap();
        presses
            .iter()
            .map(|&press| solution.value(press) as u32)
            .sum::<u32>()
    }
}

fn part_2(input: &str) -> Option<impl std::string::ToString> {
    let machines = input
        .lines()
        .map(|line| {
            let re = Regex::new(LINE_REGEX).unwrap();
            let caps = re.captures(line).unwrap();
            let wiring_re = Regex::new(WIRING_REGEX).unwrap();
            let wirings = wiring_re
                .captures_iter(&caps[2])
                .map(|wiring_cap| {
                    let wiring = &wiring_cap[1];
                    wiring
                        .split(',')
                        .map(|c| c.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let requirement = caps[3]
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            JoltageMachine {
                wirings,
                requirement,
            }
        })
        .collect::<Vec<_>>();
    machines
        .into_iter()
        .map(|machine| machine.least_presses())
        .sum::<u32>()
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
