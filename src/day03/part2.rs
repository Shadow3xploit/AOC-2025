/// Computes the total joltage value for all battery banks in the input.
///
/// Each line in the input represents a single battery bank.
/// For each bank, the maximum possible twelve-digit joltage value
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
    let mut result: i64 = 0;

    let banks = input.split("\n");
    for bank in banks {
        let joltage: i64 = find_best_joltage(bank);
        result += joltage;
    }

    result.to_string()
}

/// Computes the maximum twelve-digit joltage that can be obtained from a battery bank.
///
/// The function iteratively selects the highest digit in a moving window across the
/// bank string to construct a twelve-digit number. At each step:
/// 1. A slice of the bank is taken from the current `start_index` up to the end of
///    the remaining window needed to complete 12 digits.
/// 2. The highest digit in that slice is found using [`find_highest_number`].
/// 3. That digit is appended to the result string.
/// 4. `start_index` is advanced to the next position after the chosen digit.
///
/// The order of digits in the original bank is always preserved.
///
/// # Parameters
/// - `bank`: A string slice representing a sequence of digit characters (`'0'`–`'9'`).
///
/// # Returns
/// A twelve-digit joltage as `i64`.
///
/// # Panics
/// - If `bank` contains non-digit characters.
/// - If the bank is too short to construct a 12-digit joltage.
/// - If parsing the constructed string as `i64` fails.
fn find_best_joltage(bank: &str) -> i64 {
    let mut result: String = "".to_string();

    let mut start_index: usize = 0;
    for i in 1..=12 {
        let end_index: usize = bank.len() - 12 + i;
        let slice: &str = &bank[start_index..end_index];
        let found_index: usize = find_highest_number(slice);
        result = result.to_owned() + &slice[found_index..=found_index];
        start_index = start_index + found_index + 1;
    }

    result.parse().unwrap()
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
        assert_eq!(find_best_joltage("987654321111111"), 987654321111);
    }

    #[test]
    fn test_find_best_joltage_case_2() {
        assert_eq!(find_best_joltage("811111111111119"), 811111111119);
    }

    #[test]
    fn test_find_best_joltage_case_3() {
        assert_eq!(find_best_joltage("234234234234278"), 434234234278);
    }

    #[test]
    fn test_find_best_joltage_case_4() {
        assert_eq!(find_best_joltage("818181911112111"), 888911112111);
    }

    #[test]
    fn test_solve() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        let result = solve(input);
        assert_eq!(result, "3121910778619");
    }
}
