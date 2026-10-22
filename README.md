# Advent of Code 2025

Rust solutions for [Advent of Code 2025](https://adventofcode.com/2025).

## Requirements

- Rust and Cargo (edition 2025)

## Run

Run a day and part with Cargo. The default input is loaded from `inputs/dayXX.txt`:

```sh
cargo run -- {day}{part}
```

You can also use the `day` prefix or provide a custom input path:

```sh
cargo run -- day01a
cargo run -- 1a path/to/input.txt
```

## Solutions

| Day | Part A | Part B |
| --- | :----: | :----: |
| 01  | ✅ | ✅ |
| 02  | ✅ | ✅ |
| 03  | ✅ | ✅ |
| 04  | ✅ | ✅ |

Solutions live in `src/days/dayXX.rs` and are registered in `src/days/mod.rs`.
Add each day’s default input as `inputs/dayXX.txt`.

Run the Day 04 tests with:

```sh
cargo test day04
```

Day 04 counts rolls of paper that can be removed by a forklift from the grid. Part B repeats the removal process until no more rolls are accessible.

## Development

```sh
cargo fmt
cargo test
cargo build
```
