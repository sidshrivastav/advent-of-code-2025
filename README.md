# Advent of Code 2025

Rust solutions for [Advent of Code 2025](https://adventofcode.com/2025).

## Requirements

- Rust and Cargo (edition 2024)

## Run

Run a day and part with Cargo. The default input is loaded from `inputs/dayXX.txt`:

```sh
cargo run -- 1a
cargo run -- 1b
```

You can also use the `day` prefix or provide a custom input path:

```sh
cargo run -- day01a
cargo run -- 1a path/to/input.txt
```

The input file is a list of rotations, one per line, such as `R48` or `L2`.

## Solutions

| Day | Part A | Part B |
| --- | :----: | :----: |
| 01  | ✅ | ✅ |

Solutions live in `src/days/dayXX.rs` and are registered in `src/days/mod.rs`.
Add each day’s default input as `inputs/dayXX.txt`.

## Development

```sh
cargo fmt
cargo test
cargo build
```
