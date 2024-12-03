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
}

fn main() {
    // tracing_subscriber::fmt::init();
    let args = Args::parse();
    // let result = route(args.year, args.day);
    // println!("{}", result);
    let client = AocClient::builder()
        .session_cookie_from_default_locations()
        .expect("Failed to get session cookie")
        .year(args.year)
        .expect("Failed to get year")
        .day(args.day)
        .expect("Failed to get day")
        .build()
        .expect("Failed to build client");
    let cur_dir = Path::new(file!()).parent().unwrap();
    let file_path = cur_dir.join(format!("year_{}/day_{}/input.txt", args.year, args.day));
    if !file_path.exists() {
        println!("Input file does not exist. Creating it now.");
        let input = client.get_input().expect("Failed to get input");
        std::fs::write(&file_path, input).expect("Failed to write input to file");
    }
    let answer = route(args.year, args.day);
    println!("{answer}");
}
