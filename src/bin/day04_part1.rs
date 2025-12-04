use aoc2025::day04::part1::solve;
use aoc2025::utils::run_puzzle;

/// Entry point for Advent of Code Day 4, Part 1.
///
/// This binary reads the puzzle input and executes the solver function for Day 4, Part 1.
/// It automatically selects the input file using the `run_puzzle` utility:.
///
/// The result, along with metadata and timings, is printed to stdout.
fn main() {
    run_puzzle(4, 1, None, solve).expect("failed to run puzzle");
}
