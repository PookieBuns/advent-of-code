use advent_of_code::route;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    year: u16,
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let args = Args::parse();
    let result = route(args.year, args.day);
    println!("{}", result);
}
