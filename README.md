# Advent of Code 2023 - Rust Solutions

This repository contains my solutions for the Advent of Code 2023 challenges,
written in Rust. Each day's solution is organized into separate crates for
clarity and ease of navigation.

## Structure

The repository is structured as follows:

```
/AdventOfCode2023
|-- day01
|   |-- src
|   |   |-- lib.rs
|   |   |-- part1.rs
|   |   |-- part2.rs
|   |   |-- main.rs
|   |   |-- ...
|   |-- Cargo.toml
|-- day02
|   |-- src
|   |   |-- lib.rs
|   |   |-- part1.rs
|   |   |-- part2.rs
|   |   |-- main.rs
|   |   |-- ...
|   |-- Cargo.toml
|-- ...
|-- Cargo.toml
|-- README.md
```

- Each day's solution resides in its own crate (e.g., `day01`, `day02`).
- Every day consist of two separate modules for the two parts.
- Additionally every day have four text files with all different inputs from.
- The `main.rs` file contains rust code for displaying result of part 1 and part
  2 respectively.

## Running the Solutions

Navigate to the root directory and execute the following command:

```bash
cargo run --bin dayXX
```

Replace `XX` with the date to run. Additionally, it is required to add the
puzzle input (from AOC) in a text file with name `input.txt` in the root of the
crate `dayXX`.

## Disclaimer

These solutions are my personal attempts at solving the challenges and may not
represent the most optimized or efficient solutions. Feel free to explore,
learn, and share your own improvements!

Happy coding and good luck with the Advent of Code 2023 challenges!
