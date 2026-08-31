use std::{env, fs, process};

mod days;

fn main() {
    let selection = env::args()
        .nth(1)
        .unwrap_or_else(|| usage("missing day and part"));
    let selection = selection.strip_prefix("day").unwrap_or(&selection);
    let (day, part) = selection.split_at(selection.len().saturating_sub(1));
    let day: u8 = day
        .parse()
        .unwrap_or_else(|_| usage("use a selection such as 1a or 1b"));
    if !matches!(part, "a" | "b") {
        usage("part must be a or b");
    }

    let path = env::args()
        .nth(2)
        .unwrap_or_else(|| format!("inputs/day{day:02}.txt"));
    let input = fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("could not read {path}: {e}");
        process::exit(1);
    });

    let answer = match (day, part) {
        (1, "a") => days::day01::part_a(&input),
        (1, "b") => days::day01::part_b(&input),
        _ => usage("that day and part has not been implemented yet"),
    };
    println!("{answer}");
}

fn usage(message: &str) -> ! {
    eprintln!(
        "Error: {message}
Usage: cargo run -- <day><part> [input-file]
Example: cargo run -- 1a"
    );
    process::exit(1);
}
