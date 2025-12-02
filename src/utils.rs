use std::fs;
use std::io;
use std::path::Path;
use std::time::Instant;

/// Determines whether the current stdout supports colored output.
///
/// # Returns
/// `true` if stdout is a terminal and supports ANSI colors, `false` otherwise.
fn supports_color() -> bool {
    atty::is(atty::Stream::Stdout)
}

/// Reads an input file, executes a solver function, logs metadata, timing, and the result,
/// and returns the solver result.
///
/// This function will automatically select an input file if `input_path` is `None`:
/// - First it tries `"inputs/day{day:02}_part{part}.txt"`.
/// - If that does not exist, it falls back to `"inputs/day{day:02}.txt"`.
///
/// # Parameters
/// - `day`: The day number of the puzzle (used for input path selection and logging).
/// - `part`: The part number of the puzzle (used for input path selection and logging).
/// - `input_path`: Optional path to a specific input file. If `None`, automatic selection is used.
/// - `solve`: A function or closure that takes the file contents as `&str` and returns a `String` result.
///
/// # Returns
/// A `Result<String, io::Error>` containing the result of the `solve` function, or an I/O error
/// if the input file could not be found or read.
///
/// # Examples
/// ```no_run
/// use aoc2025::utils::run_puzzle;
/// use aoc2025::day01::part1::solve;
///
/// // Automatically select input file for day 1, part 1
/// let result = run_puzzle(1, 1, None, solve).unwrap();
///
/// // Use a specific input file
/// let result = run_puzzle(1, 1, Some("inputs/day01_example.txt"), solve).unwrap();
/// ```
pub fn run_puzzle<F>(day: i32, part: i32, input_path: Option<&str>, solve: F) -> io::Result<String>
where
    F: Fn(&str) -> String,
{
    let use_color = supports_color();

    // Determine input file
    let path = if let Some(p) = input_path {
        p.to_string()
    } else {
        let primary_path = format!("inputs/day{:02}_part{}.txt", day, part);
        let secondary_path = format!("inputs/day{:02}.txt", day);

        if Path::new(&primary_path).exists() {
            primary_path
        } else if Path::new(&secondary_path).exists() {
            secondary_path
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Input file not found: tried '{}' and '{}'",
                    primary_path, secondary_path
                ),
            ));
        }
    };

    // Start timing (input read + solve)
    let overall_start = Instant::now();

    // Read input
    let input_start = Instant::now();
    let input = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(err) => {
            if use_color {
                eprintln!(
                    "\x1b[31m[ERROR]\x1b[0m Could not read input file '{}': {}",
                    path, err
                );
            } else {
                eprintln!("[ERROR] Could not read input file '{}': {}", path, err);
            }
            return Err(err);
        }
    };
    let input_duration = input_start.elapsed();

    // Execute solver
    let solve_start = Instant::now();
    let result = solve(&input);
    let solve_duration = solve_start.elapsed();
    let overall_duration = overall_start.elapsed();

    // --- Output ---
    if use_color {
        println!("\x1b[36m--- Advent of Code ---\x1b[0m");
        println!("\x1b[34mDay:\x1b[0m  {}", day);
        println!("\x1b[34mPart:\x1b[0m {}", part);
        println!("\x1b[34mInput:\x1b[0m {}", path);
        println!();
        println!("\x1b[33mTimings:\x1b[0m");
        println!("  Input read:  {:.3} ms", duration_ms(input_duration));
        println!("  Solve:       {:.3} ms", duration_ms(solve_duration));
        println!("  Total:       {:.3} ms", duration_ms(overall_duration));
        println!();
        println!("\x1b[32mResult:\x1b[0m {}", result);
    } else {
        println!("--- Advent of Code ---");
        println!("Day:  {}", day);
        println!("Part: {}", part);
        println!("Input: {}", path);
        println!();
        println!("Timings:");
        println!("  Input read:  {:.3} ms", duration_ms(input_duration));
        println!("  Solve:       {:.3} ms", duration_ms(solve_duration));
        println!("  Total:       {:.3} ms", duration_ms(overall_duration));
        println!();
        println!("Result: {}", result);
    }

    Ok(result)
}

/// Converts a `Duration` to milliseconds as a floating point number.
///
/// # Parameters
/// - `duration`: A `std::time::Duration` to convert.
///
/// # Returns
/// A `f64` representing the duration in milliseconds.
///
/// # Examples
/// ```
/// use std::time::Duration;
/// let d = Duration::from_millis(1234);
/// assert_eq!(super::duration_ms(d), 1234.0);
/// ```
fn duration_ms(duration: std::time::Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}
