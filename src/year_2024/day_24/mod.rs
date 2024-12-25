use std::{
    collections::{HashMap, VecDeque},
    path::Path,
};

use crate::Answer;

fn get_wire_map(starting_wires_str: &str) -> HashMap<&str, u8> {
    starting_wires_str
        .lines()
        .map(|line| {
            let (wire, val) = line.split_once(": ").unwrap();
            (wire, val.parse().unwrap())
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Gate<'a> {
    left: &'a str,
    right: &'a str,
    op: Op,
    output: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Op {
    And,
    Or,
    Xor,
}

#[allow(clippy::type_complexity)]
fn create_gate_graph(
    gates_str: &str,
) -> (
    HashMap<&str, Vec<&str>>,
    HashMap<&str, i64>,
    HashMap<&str, Gate>,
) {
    gates_str
        .lines()
        .map(|line| {
            let (input, output) = line.split_once(" -> ").unwrap();
            if let Some((left, right)) = input.split_once(" XOR ") {
                Gate {
                    left,
                    right,
                    op: Op::Xor,
                    output,
                }
            } else if let Some((left, right)) = input.split_once(" AND ") {
                Gate {
                    left,
                    right,
                    op: Op::And,
                    output,
                }
            } else if let Some((left, right)) = input.split_once(" OR ") {
                Gate {
                    left,
                    right,
                    op: Op::Or,
                    output,
                }
            } else {
                unreachable!()
            }
        })
        .fold(
            (HashMap::new(), HashMap::new(), HashMap::new()),
            |(mut adj_mat, mut indeg, mut gate_map), gate| {
                adj_mat
                    .entry(gate.left)
                    .or_insert_with(Vec::new)
                    .push(gate.output);
                adj_mat
                    .entry(gate.right)
                    .or_insert_with(Vec::new)
                    .push(gate.output);
                *indeg.entry(gate.output).or_insert(0) += 2;
                indeg.entry(gate.left).or_insert(0);
                indeg.entry(gate.right).or_insert(0);
                gate_map.insert(gate.output, gate);
                (adj_mat, indeg, gate_map)
            },
        )
}

fn part_1(input: &str) -> Option<i64> {
    let (starting_wires_str, gates_str) = input.split_once("\n\n").unwrap();
    let mut wire_map = get_wire_map(starting_wires_str);
    let (adj_mat, mut indeg, gate_map) = create_gate_graph(gates_str);
    let mut q: VecDeque<_> = indeg
        .iter()
        .filter_map(|(&wire, &val)| (val == 0).then_some(wire))
        .collect();
    while let Some(wire) = q.pop_front() {
        if !wire_map.contains_key(wire) {
            let gate = gate_map.get(wire).unwrap();
            let left = wire_map.get(gate.left).unwrap();
            let right = wire_map.get(gate.right).unwrap();
            let val = match gate.op {
                Op::And => left & right,
                Op::Or => left | right,
                Op::Xor => left ^ right,
            };
            wire_map.insert(gate.output, val);
        }
        for &next_wire in adj_mat.get(wire).unwrap_or(&Vec::new()) {
            let next_indeg = indeg.get_mut(next_wire).unwrap();
            *next_indeg -= 1;
            if *next_indeg == 0 {
                q.push_back(next_wire);
            }
        }
    }
    let mut output_wires = wire_map
        .keys()
        .filter(|&&wire| wire.starts_with("z"))
        .collect::<Vec<_>>();
    output_wires.sort_by(|a, b| b.cmp(a));
    let final_str = output_wires
        .iter()
        .map(|&wire| wire_map[wire].to_string())
        .collect::<String>();
    let final_val = i64::from_str_radix(&final_str, 2).unwrap();
    final_val.into()
}

fn part_2(input: &str) -> Option<i64> {
    let (starting_wires_str, gates_str) = input.split_once("\n\n").unwrap();
    let mut wire_map = get_wire_map(starting_wires_str);
    let (adj_mat, mut indeg, gate_map) = create_gate_graph(gates_str);
    let mut q: VecDeque<_> = indeg
        .iter()
        .filter_map(|(&wire, &val)| (val == 0).then_some(wire))
        .collect();
    while let Some(wire) = q.pop_front() {
        if !wire_map.contains_key(wire) {
            let gate = gate_map.get(wire).unwrap();
            let left = wire_map.get(gate.left).unwrap();
            let right = wire_map.get(gate.right).unwrap();
            let val = match gate.op {
                Op::And => left & right,
                Op::Or => left | right,
                Op::Xor => left ^ right,
            };
            wire_map.insert(gate.output, val);
        }
        for &next_wire in adj_mat.get(wire).unwrap_or(&Vec::new()) {
            let next_indeg = indeg.get_mut(next_wire).unwrap();
            *next_indeg -= 1;
            if *next_indeg == 0 {
                q.push_back(next_wire);
            }
        }
    }
    let mut x_wires = wire_map
        .keys()
        .filter(|&&wire| wire.starts_with("x"))
        .collect::<Vec<_>>();
    x_wires.sort_by(|a, b| b.cmp(a));
    let x = i64::from_str_radix(
        &x_wires
            .iter()
            .map(|&wire| wire_map[wire].to_string())
            .collect::<String>(),
        2,
    )
    .unwrap();
    let mut y_wires = wire_map
        .keys()
        .filter(|&&wire| wire.starts_with("y"))
        .collect::<Vec<_>>();
    y_wires.sort_by(|a, b| b.cmp(a));
    let y = i64::from_str_radix(
        &y_wires
            .iter()
            .map(|&wire| wire_map[wire].to_string())
            .collect::<String>(),
        2,
    )
    .unwrap();
    let mut z_wires = wire_map
        .keys()
        .filter(|&&wire| wire.starts_with("z"))
        .collect::<Vec<_>>();
    z_wires.sort_by(|a, b| b.cmp(a));
    let z = x + y;
    let z_bin = format!("{:b}", z);
    for (i, _) in z_bin.chars().rev().enumerate() {
        let wire = format!("z{:02}", i);
        // println!("{wire}");
        // check if it is an XOR
        let gate = gate_map.get(&*wire).unwrap();
        if i == z_bin.len() - 1 {
            continue;
        }
        if gate.op != Op::Xor {
            println!("{gate:?} is not xor");
            println!();
            continue;
        }
        if i == 0 {
            continue;
        }
        // one of the left or right must be x XOR y
        let left_gate = gate_map.get(gate.left).unwrap();
        let right_gate = gate_map.get(gate.right).unwrap();
        let exist_left_xor = left_gate.op == Op::Xor
            && (left_gate.left == format!("x{:02}", i) || left_gate.right == format!("x{:02}", i));
        let exist_right_xor = right_gate.op == Op::Xor
            && (right_gate.left == format!("x{:02}", i)
                || right_gate.right == format!("x{:02}", i));
        if !(exist_left_xor || exist_right_xor) {
            println!("{gate:?} does not have x xor y");
            println!("left: {left_gate:?}");
            println!("right: {right_gate:?}");
            println!();
            continue;
        }
        if i == 1 {
            continue;
        }
        // the other must be the carry bit
        let carry_gate = if exist_left_xor {
            right_gate
        } else {
            left_gate
        };
        // carry gate must be an OR
        if carry_gate.op != Op::Or {
            println!("{gate:?} carry gate is not or");
            println!("carry gate {carry_gate:?}");
            println!();
            continue;
        }
        // one of the carry gate's left or right must be x AND y
        let carry_gate_left = gate_map.get(carry_gate.left).unwrap();
        let carry_gate_right = gate_map.get(carry_gate.right).unwrap();
        let exist_left_and = carry_gate_left.op == Op::And
            && (carry_gate_left.left == format!("x{:02}", i - 1)
                || carry_gate_left.right == format!("x{:02}", i - 1));
        let exist_right_and = carry_gate_right.op == Op::And
            && (carry_gate_right.left == format!("x{:02}", i - 1)
                || carry_gate_right.right == format!("x{:02}", i - 1));
        if !(exist_left_and || exist_right_and) {
            println!("{gate:?} carry gate does not have x and y");
            println!("carry gate: {carry_gate:?}");
            println!("carry gate left: {:?}", carry_gate_left);
            println!("carry gate right: {:?}", carry_gate_right);
            println!();
            continue;
        }
    }
    let mut swaps = ["z12", "qdg", "dck", "fgn", "nvh", "z37", "z19", "vvf"];
    swaps.sort();
    println!("{}", swaps.join(","));
    None
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
