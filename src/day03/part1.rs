/// Computes the total joltage value for all battery banks in the input.
///
/// Each line in the input represents a single battery bank.
/// For each bank, the maximum possible two-digit joltage value
/// is derived from that bank. The resulting joltages are summed
/// and returned as a string.
///
/// # Parameters
/// - `input`: A string containing one bank per line.
///
/// # Returns
/// A string containing the total sum of all computed joltages.
///
/// # Panics
/// This function will panic if any line contains non-numeric characters
/// or if joltage construction/parsing fails internally.
pub fn solve(input: &str) -> String {
    let mut result: i32 = 0;

    let banks = input.split("\n");
    for bank in banks {
        let joltage: i32 = find_best_joltage(bank);
        result += joltage;
    }

    result.to_string()
}

/// Finds the best possible two-digit joltage in a digit string.
///
/// The algorithm selects the highest digit among all but the final
/// position, and then selects the highest digit found to the *right*
/// of that position. The original order of digits is always preserved.
/// The two selected digits are combined into a two-digit number.
///
/// # Parameters
/// - `bank`: A string slice representing a sequence of digit characters (`'0'`–`'9'`).
///
/// # Returns
/// The two-digit joltage as an `i32`.
///
/// # Panics
/// - If `bank` contains any non-digit characters.
/// - If the string has length < 2.
/// - If parsing the constructed two-digit number fails.
fn find_best_joltage(bank: &str) -> i32 {
    let first_slice: &str = &bank[0..(bank.len() - 1)];
    let first_index: usize = find_highest_number(first_slice);

    let second_slice: &str = &bank[(first_index + 1)..(bank.len())];
    let second_index: usize = find_highest_number(second_slice);

    (first_slice[first_index..=first_index].to_owned() + &second_slice[second_index..=second_index])
        .parse()
        .unwrap()
}

/// Returns the index of the highest digit within a digit substring.
///
/// The function iterates through all characters in the given `range`
/// and identifies the index of the numerically largest digit.
///
/// # Parameters
/// - `range`: A string slice consisting only of digit characters.
///
/// # Returns
/// The zero-based index of the highest digit in the slice.  
/// If multiple positions share the highest digit, the earliest index is returned.
///
/// # Panics
/// - If any character in the range is not a digit.
/// - If indexing into the string fails (e.g., non-ASCII digits).
fn find_highest_number(range: &str) -> usize {
    let mut index = 0;
    let mut value = 0;
    for i in 0..range.len() {
        let digit_value: i32 = range[i..(i + 1)].parse().unwrap();
        if value < digit_value {
            value = digit_value;
            index = i;
        }
    }
    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_highest_number_simple() {
        assert_eq!(find_highest_number("12345"), 4); // '5'
    }

    #[test]
    fn test_find_highest_number_first_is_highest() {
        assert_eq!(find_highest_number("987654"), 0); // '9'
    }

    #[test]
    fn test_find_highest_number_last_is_highest() {
        assert_eq!(find_highest_number("111119"), 5); // '9'
    }

    #[test]
    fn test_find_highest_number_middle_is_highest() {
        assert_eq!(find_highest_number("1213121"), 3); // '3'
    }

    #[test]
    fn test_find_highest_number_repetitive_pattern() {
        assert_eq!(find_highest_number("818181"), 0); // first '8'
    }

    #[test]
    fn test_find_best_joltage_case_1() {
        assert_eq!(find_best_joltage("987654321111111"), 98);
    }

    #[test]
    fn test_find_best_joltage_case_2() {
        assert_eq!(find_best_joltage("811111111111119"), 89);
    }

    #[test]
    fn test_find_best_joltage_case_3() {
        assert_eq!(find_best_joltage("234234234234278"), 78);
    }

    #[test]
    fn test_find_best_joltage_case_4() {
        assert_eq!(find_best_joltage("818181911112111"), 92);
    }

    #[test]
    fn test_solve() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        let result = solve(input);
        assert_eq!(result, "357");
    }
}
