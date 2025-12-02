use aoc2025::day02::part2::solve;
use aoc2025::utils::run_puzzle;

/// Entry point for Advent of Code Day 2, Part 2.
///
/// This binary reads the puzzle input and executes the solver function for Day 2, Part 2.
/// It automatically selects the input file using the `run_puzzle` utility:.
///
/// The result, along with metadata and timings, is printed to stdout.
fn main() {
    run_puzzle(2, 2, None, solve).expect("failed to run puzzle");
}
