use std::path::Path;

use advent_of_code::route;
use aoc_client::AocClient;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    year: i32,
    #[arg(short, long)]
    day: u32,
    #[arg(short, long)]
    submit: bool,
}

fn main() {
    let args = Args::parse();
    let cur_dir = Path::new(file!()).parent().unwrap();
    let puzzle_dir = cur_dir.join(format!("year_{}/day_{}", args.year, args.day));
    let input_path = puzzle_dir.join("input.txt");
    let puzzle_path = puzzle_dir.join("puzzle.md");
    let client = AocClient::builder()
        .session_cookie_from_default_locations()
        .expect("Failed to get session cookie")
        .year(args.year)
        .expect("Failed to get year")
        .day(args.day)
        .expect("Failed to get day")
        .input_filename(input_path.clone())
        .puzzle_filename(puzzle_path.clone())
        .build()
        .expect("Failed to build client");
    if !puzzle_path.exists() {
        println!("Puzzle file does not exist. Creating it now.");
        client
            .save_puzzle_markdown()
            .expect("Failed to save puzzle markdown");
    }
    if !input_path.exists() {
        println!("Input file does not exist. Creating it now.");
        client.save_input().expect("Failed to save input");
    }
    let answer = route(args.year, args.day);
    println!("{answer}");
    if args.submit {
        println!("Submitting answers\n");
        answer.part_1.map(|p1| {
            println!("Submitting {p1} for part 1");
            client.submit_answer_and_show_outcome(1, p1)
        });
        answer.part_2.map(|p2| {
            println!("Submitting {p2} for part 2");
            client.submit_answer_and_show_outcome(2, p2)
        });
    }
}
