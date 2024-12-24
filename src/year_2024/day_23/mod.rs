use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use itertools::Itertools;
use rayon::prelude::*;

use crate::Answer;

#[derive(Debug)]
struct Connection {
    a: String,
    b: String,
}

fn build_graph(connections: &[Connection]) -> HashMap<String, HashSet<String>> {
    let mut graph = HashMap::new();
    for connection in connections {
        graph
            .entry(connection.a.clone())
            .or_insert_with(HashSet::new)
            .insert(connection.b.clone());
        graph
            .entry(connection.b.clone())
            .or_insert_with(HashSet::new)
            .insert(connection.a.clone());
    }
    graph
}

fn find_interconnected_subgraphs(
    graph: &HashMap<String, HashSet<String>>,
    node: &str,
    count: usize,
) -> Vec<HashSet<String>> {
    let neighbors = graph.get(node).unwrap();
    let mut interconnected_subgraphs = Vec::new();
    for combination in neighbors.iter().combinations(count - 1) {
        let mut interconnected = true;
        for &neighbor in &combination {
            let neighbor_neighbors = graph.get(neighbor).unwrap();
            for &other_neighbor in &combination {
                if neighbor == other_neighbor {
                    continue;
                }
                if !neighbor_neighbors.contains(other_neighbor) {
                    interconnected = false;
                    break;
                }
            }
            if !interconnected {
                break;
            }
        }
        if interconnected {
            let mut subgraph = HashSet::new();
            subgraph.insert(node.to_string());
            subgraph.extend(combination.iter().map(|s| s.to_string()));
            interconnected_subgraphs.push(subgraph);
        }
    }
    interconnected_subgraphs
}

fn part_1(input: &str) -> Option<i32> {
    let connections: Vec<Connection> = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("-").unwrap();
            Connection {
                a: left.to_string(),
                b: right.to_string(),
            }
        })
        .collect();
    let graph = build_graph(&connections);
    let mut interconnected_subgraphs = Vec::new();
    for node in graph.keys() {
        if !node.starts_with("t") {
            continue;
        }
        for interconnected_subgraph in find_interconnected_subgraphs(&graph, node, 3) {
            if !interconnected_subgraphs.contains(&interconnected_subgraph) {
                interconnected_subgraphs.push(interconnected_subgraph);
            }
        }
    }
    (interconnected_subgraphs.len() as i32).into()
}

fn is_fully_connected(graph: &HashMap<String, HashSet<String>>, nodes: Vec<&String>) -> bool {
    nodes.iter().all(|&node| {
        let neighbors = graph.get(node).unwrap();
        if neighbors.len() < nodes.len() - 1 {
            println!("{} has too few neighbors", node);
            return false;
        }
        nodes.iter().all(|&other_node| {
            if node == other_node {
                return true;
            }
            neighbors.contains(other_node)
        })
    })
}

fn bron_kerbosch(
    graph: &HashMap<String, HashSet<String>>,
    r: HashSet<String>,
    mut p: HashSet<String>,
    mut x: HashSet<String>,
) -> HashSet<String> {
    if p.is_empty() && x.is_empty() {
        return r;
    }
    let mut max_clique = HashSet::new();
    for node in p.clone() {
        let node_neighbors = graph.get(&node).unwrap();
        let mut new_r = r.clone();
        new_r.insert(node.clone());
        let new_p = &p & node_neighbors;
        let new_x = &x & node_neighbors;
        let reported = bron_kerbosch(graph, new_r, new_p, new_x);
        if reported.len() > max_clique.len() {
            max_clique = reported;
        }
        p.remove(&node);
        x.insert(node);
    }
    max_clique
}

fn part_2(input: &str) -> Option<i32> {
    let connections: Vec<Connection> = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("-").unwrap();
            Connection {
                a: left.to_string(),
                b: right.to_string(),
            }
        })
        .collect();
    let graph = build_graph(&connections);
    let nodes = graph.keys().cloned().collect::<HashSet<String>>();
    let max_clique = bron_kerbosch(&graph, HashSet::new(), nodes.clone(), HashSet::new());
    let mut max_clique_vec = max_clique.into_iter().collect::<Vec<String>>();
    max_clique_vec.sort();
    println!("{}", max_clique_vec.join(","));
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
