mod dayrunner;
mod generator;

use dayrunner::dayrunner::rundays;
use generator::generator::generate;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut mode, mut day) = parse_args()?;
    // mode = "run".to_string();
    // day = 1;
    run_mode(mode, day)
}
fn parse_args() -> Result<(String, u32), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <mode> <day>", args[0]);
        std::process::exit(1);
    }

    let mode = args[1].clone();
    let day: u32 = args[2].parse().expect("Day must be a number");

    Ok((mode, day))
}

fn run_mode(mode: String, day: u32) -> Result<(), Box<dyn std::error::Error>> {
    match mode.as_str() {
        "generate" => generate(r"C:\Users\thiba\Desktop\aoc\aoc2026\src\dayrunner", day),
        "run" => rundays(r"C:\Users\thiba\Desktop\aoc\aoc2026\input", "test", day),
        _ => {
            eprintln!("Invalid mode. Use 'generate' or 'run'.");
            std::process::exit(1);
        }
    }
}
