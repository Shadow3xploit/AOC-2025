use aoc2025::day01::part2::solve;
use aoc2025::utils::run_puzzle;

/// Entry point for Advent of Code Day 1, Part 2.
///
/// This binary reads the puzzle input and executes the solver function for Day 1, Part 2.
/// It automatically selects the input file using the `run_puzzle` utility:.
///
/// The result, along with metadata and timings, is printed to stdout.
fn main() {
    run_puzzle(1, 2, None, solve).expect("failed to run puzzle");
}
