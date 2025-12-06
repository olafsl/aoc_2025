use clap::Parser;
use std::fs;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: String,

    #[arg(short, long)]
    example: bool,
}

fn load_input(day: &str, example: bool) -> Vec<String> {
    let data_dir = if example { "examples" } else { "data" };
    let file_path = format!("{}/day_{}.txt", data_dir, day);

    let contents = fs::read_to_string(&file_path).unwrap();
    let lines = contents.lines().map(|s| s.to_string()).collect();
    lines
}

fn run_solution(day: &str, input: Vec<String>) {
    match day {
        "1" => day1::calculate(input).unwrap(),
        "2" => day2::main(input).unwrap(),
        "3" => day3::main(input).unwrap(),
        "4" => day4::main(input).unwrap(),
        "5" => day5::main(input).unwrap(),
        "6" => day6::main(input).unwrap(),
        _ => println!("Day {} not implemented yet", day),
    }
}

fn main() {
    let args = Args::parse();
    let input = load_input(&args.day, args.example);
    run_solution(&args.day, input);
}
