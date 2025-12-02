/// Solves Day 01 Part 1 puzzle.
///
/// Takes a multiline string of dial rotation commands and returns the number of times
/// the dial ends up at position 0 as a `String`.
///
/// # Parameters
/// - `input`: A string slice containing commands, one per line.  
///   Each command starts with "R" or "L" followed by a number, e.g., "R5" or "L12".
///
/// # Returns
/// A `String` representing how many times the dial reached 0 after executing all commands.
pub fn solve(input: &str) -> String {
    let commands = input.split("\n");
    let mut dial = 50;
    let mut dial_zero_count = 0;
    for command in commands {
        dial = rotate_dial(dial, command);
        if dial == 0 {
            dial_zero_count += 1;
        }
    }
    return dial_zero_count.to_string();
}

/// Rotates a dial from a starting position based on a command.
///
/// The dial has positions from 0 to 99 and wraps around.  
/// Commands are strings starting with "R" (rotate right / increment) or "L" (rotate left / decrement)
/// followed by a positive integer count.
///
/// # Parameters
/// - `start_position`: Current dial position (0..=99).
/// - `command`: Rotation command string, e.g., "R5" or "L12".
///
/// # Returns
/// The new dial position after applying the rotation command.
fn rotate_dial(start_position: i32, command: &str) -> i32 {
    let right: bool = command.starts_with("R");
    let mut count: i32 = command[1..].parse().unwrap();
    let mut updated: i32 = start_position;
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

        count -= 1;
    }
    return updated;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_dial_right_without_overflow() {
        let start_position = 50;
        let command = "R5";
        let result = rotate_dial(start_position, command);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_rotate_dial_right_with_overflow() {
        let start_position = 99;
        let command = "R1";
        let result = rotate_dial(start_position, command);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_rotate_dial_left_without_overflow() {
        let start_position = 50;
        let command = "L5";
        let result = rotate_dial(start_position, command);
        assert_eq!(result, 45);
    }

    #[test]
    fn test_rotate_dial_left_with_overflow() {
        let start_position = 0;
        let command = "L1";
        let result = rotate_dial(start_position, command);
        assert_eq!(result, 99);
    }

    #[test]
    fn test_solve() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        let result = solve(input);
        assert_eq!(result, "3");
    }
}
