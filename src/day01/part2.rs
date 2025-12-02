/// Solves Day 01 Part 2 puzzle.
///
/// Processes a list of dial rotation commands and counts how many times
/// the dial passes through position 0 during all rotations.
///
/// # Parameters
/// - `input`: A string slice containing commands, one per line.  
///   Each command starts with `"R"` or `"L"` followed by a number, e.g., `"R5"` or `"L12"`.
///
/// # Returns
/// A `String` representing the total number of times the dial passed through 0.
///
/// # Examples
/// ```
/// use aoc2025::day01::part2::solve;
///
/// let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
/// let result = solve(input);
/// assert_eq!(result, "6");
/// ```
pub fn solve(input: &str) -> String {
    let commands = input.split("\n");
    let mut dial = 50;
    let mut dial_zero_count = 0;
    for command in commands {
        let update: (i32, i32) = rotate_dial(dial, command);
        dial = update.0;
        dial_zero_count += update.1;
    }
    return dial_zero_count.to_string();
}

/// Rotates a dial from a starting position based on a command,
/// counting how often position 0 is passed.
///
/// # Parameters
/// - `start_position`: Current dial position (0..=99)
/// - `command`: Rotation command string starting with `"R"` or `"L"`
///   followed by a positive integer count.
///
/// # Returns
/// A tuple `(new_position, zero_passes)`
/// - `new_position`: the dial position after applying the command
/// - `zero_passes`: number of times position 0 was passed during this rotation
///
/// # Examples
/// ```
/// use aoc2025::day01::part2::rotate_dial;
///
/// let result = rotate_dial(99, "R5");
/// assert_eq!(result, (4, 1)); // Wraps around once
/// ```
fn rotate_dial(start_position: i32, command: &str) -> (i32, i32) {
    let right: bool = command.starts_with("R");
    let mut count: i32 = command[1..].parse().unwrap();
    let mut updated: i32 = start_position;
    let mut zero_passes: i32 = 0;
    while count > 0 {
        if right {
            updated += 1;
        } else {
            updated -= 1;
        }

        if updated > 99 {
            updated = 0;
        }
        if updated < 0 {
            updated = 99;
        }

        if updated == 0 {
            zero_passes += 1;
        }

        count -= 1;
    }
    return (updated, zero_passes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_dial_right_without_overflow() {
        let start_position = 50;
        let command = "R5";
        let result = rotate_dial(start_position, command);
        assert_eq!(result, (55, 0));
    }

    #[test]
    fn test_rotate_dial_right_with_small_overflow() {
        let start_position = 99;
        let command = "R1";
        let result = rotate_dial(start_position, command);
        assert_eq!(result, (0, 1));
    }

    #[test]
    fn test_rotate_dial_right_with_large_overflow() {
        let start_position = 99;
        let command = "R5";
        let result = rotate_dial(start_position, command);
        assert_eq!(result, (4, 1));
    }

    #[test]
    fn test_rotate_dial_right_with_extra_large_overflow() {
        let start_position = 50;
        let command = "R1000";
        let result = rotate_dial(start_position, command);
        assert_eq!(result, (50, 10));
    }

    #[test]
    fn test_rotate_dial_left_without_overflow() {
        let start_position = 50;
        let command = "L5";
        let result = rotate_dial(start_position, command);
        assert_eq!(result, (45, 0));
    }

    #[test]
    fn test_rotate_dial_left_without_overflow_to_zero() {
        let start_position = 5;
        let command = "L5";
        let result = rotate_dial(start_position, command);
        assert_eq!(result, (0, 1));
    }

    #[test]
    fn test_rotate_dial_left_with_small_overflow() {
        let start_position = 0;
        let command = "L1";
        let result = rotate_dial(start_position, command);
        assert_eq!(result, (99, 0));
    }

    #[test]
    fn test_rotate_dial_left_with_large_overflow_and_start_at_zero() {
        let start_position = 0;
        let command = "L5";
        let result = rotate_dial(start_position, command);
        assert_eq!(result, (95, 0));
    }

    #[test]
    fn test_rotate_dial_left_with_large_overflow_and_start_above_zero() {
        let start_position = 1;
        let command = "L5";
        let result = rotate_dial(start_position, command);
        assert_eq!(result, (96, 1));
    }

    #[test]
    fn test_rotate_dial_left_with_extra_large_overflow() {
        let start_position = 50;
        let command = "L1000";
        let result = rotate_dial(start_position, command);
        assert_eq!(result, (50, 10));
    }

    #[test]
    fn test_solve() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        let result = solve(input);
        assert_eq!(result, "6");
    }
}