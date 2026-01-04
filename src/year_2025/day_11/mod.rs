use std::collections::HashMap;
use std::path::Path;

use crate::Answer;

fn paths_to_out(
    adj_mat: &HashMap<String, Vec<String>>,
    cur: &str,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    if let Some(&paths) = cache.get(cur) {
        return paths;
    }
    let paths = adj_mat
        .get(cur)
        .unwrap_or(&vec![])
        .iter()
        .map(|neighbor| paths_to_out(adj_mat, neighbor, cache))
        .sum();
    cache.insert(cur.to_string(), paths);
    paths
}

fn part_1(input: &str) -> Option<impl std::string::ToString> {
    let adj_mat = input
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(':').unwrap();
            let from = from.trim().to_string();
            let to_nodes = to
                .trim()
                .split(' ')
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>();
            (from, to_nodes)
        })
        .collect::<HashMap<_, _>>();
    paths_to_out(
        &adj_mat,
        "you",
        &mut HashMap::from_iter([("out".to_string(), 1)]),
    )
    .into()
}

#[derive(Debug, Clone, Copy, Default)]
struct Ans {
    fft: u64,
    dac: u64,
    both: u64,
    paths: u64,
}

fn paths_to_out_2(
    adj_mat: &HashMap<String, Vec<String>>,
    cur: &str,
    // state: &mut Vec<String>,
    cache: &mut HashMap<String, Ans>,
) -> Ans {
    if let Some(&ans) = cache.get(cur) {
        return ans;
    }
    let mut ans = adj_mat
        .get(cur)
        .unwrap()
        .iter()
        .fold(Ans::default(), |ans, neighbor| {
            let neighbor_ans = paths_to_out_2(adj_mat, neighbor, cache);
            Ans {
                fft: ans.fft + neighbor_ans.fft,
                dac: ans.dac + neighbor_ans.dac,
                both: ans.both + neighbor_ans.both,
                paths: ans.paths + neighbor_ans.paths,
            }
        });
    if cur == "fft" {
        ans.fft += ans.paths;
        ans.both += ans.dac;
    }
    if cur == "dac" {
        ans.dac += ans.paths;
        ans.both += ans.fft;
    }
    cache.insert(cur.to_string(), ans);
    ans
}

fn part_2(input: &str) -> Option<impl std::string::ToString> {
    let adj_mat = input
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(':').unwrap();
            let from = from.trim().to_string();
            let to_nodes = to
                .trim()
                .split(' ')
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>();
            (from, to_nodes)
        })
        .collect::<HashMap<_, _>>();
    let ans = paths_to_out_2(
        &adj_mat,
        "svr",
        &mut HashMap::from_iter([(
            "out".to_string(),
            Ans {
                fft: 0,
                dac: 0,
                both: 0,
                paths: 1,
            },
        )]),
    );
    ans.both.into()
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
