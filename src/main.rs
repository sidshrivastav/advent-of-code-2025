use std::{env, fs, process};
mod days;
fn main() {
    let day = env::args().nth(1).unwrap_or_else(|| usage("missing day"));
    let day = day.strip_prefix("day").unwrap_or(&day);
    let day: u8 = day
        .parse()
        .unwrap_or_else(|_| usage("day must be a number"));
    let path = env::args()
        .nth(2)
        .unwrap_or_else(|| format!("inputs/day{day:02}.txt"));
    let input = fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("could not read {path}: {e}");
        process::exit(1);
    });
    let answer = match day {
        1 => days::day01::solve(&input),
        _ => usage("that day has not been implemented yet"),
    };
    println!("{answer}");
}
fn usage(message: &str) -> ! {
    eprintln!("Error: {message}\nUsage: cargo run -- <day> [input-file]");
    process::exit(1);
}
