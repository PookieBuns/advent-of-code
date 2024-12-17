use std::{path::Path, str::FromStr};

// use rayon::prelude::*;

use crate::Answer;

#[derive(Debug)]
struct Register {
    a: i64,
    b: i64,
    c: i64,
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();
        let a: i64 = lines[0].split_whitespace().last().unwrap().parse().unwrap();
        let b: i64 = lines[1].split_whitespace().last().unwrap().parse().unwrap();
        let c: i64 = lines[2].split_whitespace().last().unwrap().parse().unwrap();
        Ok(Register { a, b, c })
    }
}

fn get_combo_operand(operand: i64, register: &Register) -> i64 {
    match operand {
        0..=3 => operand,
        4 => register.a,
        5 => register.b,
        6 => register.c,
        _ => panic!("Invalid operand"),
    }
}

fn run_program(register: Register, program: &[i64]) -> Vec<i64> {
    let mut output = Vec::new();
    let mut instruction_pointer = 0;
    let mut register = register;
    while instruction_pointer < program.len() - 1 {
        let instruction = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];
        match instruction {
            0 => {
                let numerator = register.a;
                let denominator = 2_i64.pow(get_combo_operand(operand, &register) as u32);
                register.a = numerator / denominator;
            }
            1 => {
                register.b ^= operand;
            }
            2 => {
                let combo_operand = get_combo_operand(operand, &register);
                register.b = combo_operand % 8;
            }
            3 => {
                if register.a != 0 {
                    instruction_pointer = operand as usize;
                    continue;
                }
            }
            4 => {
                register.b ^= register.c;
            }
            5 => {
                let combo_operand = get_combo_operand(operand, &register);
                output.push(combo_operand % 8);
            }
            6 => {
                let numerator = register.a;
                let denominator = 2_i64.pow(get_combo_operand(operand, &register) as u32);
                register.b = numerator / denominator;
            }
            7 => {
                let numerator = register.a;
                let denominator = 2_i64.pow(get_combo_operand(operand, &register) as u32);
                register.c = numerator / denominator;
            }
            _ => panic!("Invalid instruction"),
        }
        instruction_pointer += 2;
    }
    output
}

fn part_1(input: &str) -> Option<i64> {
    let (register_str, program_str) = input.split_once("\n\n").unwrap();
    let register: Register = register_str.parse().unwrap();
    let program: Vec<i64> = program_str
        .split_whitespace()
        .last()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let output = run_program(register, &program);
    let comma_joined_output = output.iter().map(|&i| i.to_string()).collect::<Vec<_>>();
    println!("{:?}", comma_joined_output.join(","));
    // concat output
    // let output_str = output.iter().map(|&i| i.to_string()).collect::<String>();
    None
}

fn is_same(register: Register, program: &[i64]) -> bool {
    let mut program_pointer = 0;
    let mut instruction_pointer = 0;
    let mut register = register;
    while instruction_pointer < program.len() - 1 {
        let instruction = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];
        match instruction {
            0 => {
                let numerator = register.a;
                let denominator = 2_i64.pow(get_combo_operand(operand, &register) as u32);
                register.a = numerator / denominator;
            }
            1 => {
                register.b ^= operand;
            }
            2 => {
                let combo_operand = get_combo_operand(operand, &register);
                register.b = combo_operand % 8;
            }
            3 => {
                if register.a != 0 {
                    instruction_pointer = operand as usize;
                    continue;
                }
            }
            4 => {
                register.b ^= register.c;
            }
            5 => {
                let combo_operand = get_combo_operand(operand, &register);
                let to_push = combo_operand % 8;
                if program_pointer >= program.len() {
                    return false;
                }
                if to_push != program[program_pointer] {
                    return false;
                }
                program_pointer += 1;
            }
            6 => {
                let numerator = register.a;
                let denominator = 2_i64.pow(get_combo_operand(operand, &register) as u32);
                register.b = numerator / denominator;
            }
            7 => {
                let numerator = register.a;
                let denominator = 2_i64.pow(get_combo_operand(operand, &register) as u32);
                register.c = numerator / denominator;
            }
            _ => panic!("Invalid instruction"),
        }
        instruction_pointer += 2;
    }
    return program_pointer == program.len();
}

fn part_2(input: &str) -> Option<i64> {
    let (register_str, program_str) = input.split_once("\n\n").unwrap();
    let register: Register = register_str.parse().unwrap();
    let program: Vec<i64> = program_str
        .split_whitespace()
        .last()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    // let mut x = HashMap::new();
    // // for a in 0..1000 {
    //     let a_mod_16 = a % 32;
    //     let register = Register { a, ..register };
    //     let output = run_program(register, program.clone());
    //     x.entry(a_mod_16).or_insert_with(Vec::new).push(output[0]);
    // }
    // for (k, v) in x.iter() {
    //     println!("{k}");
    //     println!("{v:?}");
    // }
    // None
    // let mut range_start = None;
    // let mut range_end = None;
    // for i in 0..(i64::MAX / 100000000) {
    //     let a = i * 100000000;
    //     println!("{:?}", a);
    //     let register = Register { a, ..register };
    //     let output = run_program(register, program.clone());
    //     if output.len() == program.len() {
    //         if range_start.is_none() {
    //             range_start = Some(a - 100000000);
    //         }
    //     } else if range_start.is_some() {
    //         range_end = Some(a);
    //         break;
    //     }
    // }
    let range_start: i64 = 142279579;
    let range_end: i64 = 281475000000000;
    println!("{:?}", range_start);
    println!("{:?}", range_end);
    // let mut other_incs = vec![];
    let incs = [2, 4094, 2, 4094, 2, 16769022];
    let mut incs_iter = incs.iter().cycle();
    let mut a = range_start;
    // let mut prev_a = 0;
    loop {
        let register = Register { a, ..register };
        // let output = run_program(register, &program);
        // assert_eq!(output[0..7], program[0..7]);
        // println!("{:?}", output);
        // println!("{:?}", program);
        // if output.len() >= 12 && (output[0..12] == program[0..12]) {
        //     other_incs.push(a - prev_a);
        //     println!("{:?}", other_incs);
        //     prev_a = a;
        //     // return None;
        // }
        if is_same(register, &program) {
            return Some(a);
        }
        a += incs_iter.next().unwrap();
    }
    // None
    // (range_start.unwrap()..range_end.unwrap())
    //     .into_par_iter()
    //     .filter(|&a| !matches!(a % 8, 6 | 7))
    //     .find_any(|a| {
    //         let register = Register { a: *a, ..register };
    //         is_same(register, &program)
    //     })
    // let mut incs = vec![];
    // let mut prev_a = 0;
    // for a in 0..i64::MAX {
    //     let register = Register { a, ..register };
    //     let output = run_program(register, &program);
    //     if output.len() < 7 {
    //         continue;
    //     }
    //     if output[0..7] == program[0..7] {
    //         println!("{:?}", a);
    //         incs.push(a - prev_a);
    //         prev_a = a;
    //         println!("{:?}", incs);
    //     }
    // }
    // None
}
pub fn solve() -> Answer {
    let cur_dir = Path::new(file!()).parent().unwrap();
    let file_path = cur_dir.join("input.txt");
    let input = std::fs::read_to_string(file_path).unwrap();
    let part_1 = part_1(&input);
    let part_2 = part_2(&input);
    Answer::from_parts(part_1, part_2)
}
