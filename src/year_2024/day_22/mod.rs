use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use rayon::prelude::*;

use crate::Answer;

fn generate_next_secret_number(secret_number: i64) -> i64 {
    let mut secret = secret_number;
    // step 1
    let mut result = secret * 64;
    let mut mixed = result ^ secret;
    let mut pruned = mixed % 16777216;
    secret = pruned;
    // step 2
    result = secret / 32;
    mixed = result ^ secret;
    pruned = mixed % 16777216;
    secret = pruned;
    // step 3
    result = secret * 2048;
    mixed = result ^ secret;
    pruned = mixed % 16777216;
    secret = pruned;
    secret
}

fn generate_nth_secret_number(secret_number: i64, n: usize) -> i64 {
    (0..n).fold(secret_number, |secret, _| {
        generate_next_secret_number(secret)
    })
}

fn part_1(input: &str) -> Option<i64> {
    input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .map(|secret_number| generate_nth_secret_number(secret_number, 2000))
        .sum::<i64>()
        .into()
}

fn generate_prices(secret_number: i64, n: usize) -> Vec<i64> {
    let mut prices = Vec::new();
    prices.push(secret_number % 10);
    let next_prices: Vec<i64> = (0..n)
        .scan(secret_number, |state, _| {
            let s = generate_next_secret_number(*state);
            *state = s;
            Some(s % 10)
        })
        .collect();
    prices.extend(next_prices);
    prices
}

fn part_2(input: &str) -> Option<i64> {
    let window_bananas = input
        .par_lines()
        .map(|line| line.parse::<i64>().unwrap())
        .map(|secret_number| {
            let prices = generate_prices(secret_number, 2000);
            let changes = prices.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i64>>();
            (prices, changes)
        })
        .fold(
            HashMap::new,
            |mut local_window_bananas, (prices, changes)| {
                let mut seen_windows = HashSet::new();
                for window in changes.windows(4) {
                    let window_owned = window.to_vec();
                    if !seen_windows.insert(window_owned.clone()) {
                        continue;
                    }
                    let i = changes.windows(4).position(|w| w == window_owned);
                    let price = i.map(|i| prices[i + 4]).unwrap_or(0);
                    *local_window_bananas.entry(window_owned).or_insert(0) += price;
                }
                local_window_bananas
            },
        )
        .reduce(HashMap::new, |mut acc, local_window_bananas| {
            for (window, bananas) in local_window_bananas {
                *acc.entry(window).or_insert(0) += bananas;
            }
            acc
        });

    window_bananas.values().copied().max()
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
