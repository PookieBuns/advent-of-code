use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug)]
struct Next {
    left: String,
    right: String,
}

fn parse_node(node: &str) -> (String, Next) {
    let (from, mut to) = node.split_once(" = ").unwrap();
    to = to.trim_matches('(').trim_matches(')');
    let (left, right) = to.split_once(", ").unwrap();
    let to = Next {
        left: left.to_string(),
        right: right.to_string(),
    };
    (from.to_string(), to)
}

fn parse_nodes(nodes: &str) -> HashMap<String, Next> {
    let nodes = nodes.split('\n');
    let map: HashMap<String, Next> = nodes.map(parse_node).collect();
    map
}

fn part_1(input: &str) -> impl Display {
    let input = input.trim();
    let (directions, nodes) = input.split_once("\n\n").unwrap();
    let directions: Vec<char> = directions.chars().collect();
    let map = parse_nodes(nodes);
    let mut curr = "AAA";
    let mut step = 0;
    while curr != "ZZZ" {
        let next = map.get(curr).unwrap();
        let direction = directions[step % directions.len()];
        match direction {
            'L' => curr = &next.left,
            'R' => curr = &next.right,
            _ => panic!("Invalid direction"),
        }
        step += 1;
    }
    step
}
fn part_2(input: &str) -> impl Display {
    "Unimplemented"
}
pub fn solve() -> String {
    let input = include_str!("input.txt");
    let part_1_ans = part_1(input);
    let part_2_ans = part_2(input);
    format!("Part 1: {}\nPart 2: {}", part_1_ans, part_2_ans)
}
