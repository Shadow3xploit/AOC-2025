# Advent of Code 2025

This repository contains my solutions for **Advent of Code 2025**.  
My main goal is to use the daily challenges to **learn Rust**, deepen my understanding of Rust’s ecosystem, and practice writing clean, modular code.

## Project Structure

The project is organized to keep each day isolated while still sharing common utilities:

```
.
├── Cargo.toml
├── inputs/
│   ├── day01.txt
│   ├── day01_part1.txt
│   └── ...
└── src/
    ├── bin/
    ├── days/
    │   ├── day01.rs
    │   ├── day02.rs
    │   └── ...
    └── utils.rs
```

### Explanation

- **`src/bin/`**  
  Contains one bootstrap file for each challenge part of the Advent of Code days.

- **`src/days/`**  
  Each file contains the solution for one part of the Advent of Code day.  
  The module exposes functions the functions to solve the problem.

- **`inputs/`**  
  Contains all puzzle inputs.  
  The runner tries to load inputs following this priority:  
  ```
  inputs/day{XX}_part{N}.txt
  inputs/day{XX}.txt
  ```

- **`src/utils.rs`**  
  Shared helper logic such as input loading.

## Running Solutions

You can run a specific day with:

```
cargo run --bin day{XX}_part{N}
```

Examples:

```
cargo run --bin day01_part1
```

## Running All Tests

Tests are integrated in every source file (where necessary). To run them execute:

```
cargo test --lib
```

To test only one day/part:

```
argo test --lib day{XX}[::part{N}]
```