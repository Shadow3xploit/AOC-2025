/// Counts how many IDs fall within at least one of the specified ranges.
///
/// The input consists of two sections separated by an empty line:
/// 1. A list of ranges in the format `"start-end"`
/// 2. A list of numeric IDs
///
/// Each ID is checked against all ranges, and is counted once if it fits in **any**
/// of the ranges.
///
/// # Arguments
/// * `input` – Full problem input containing ranges and IDs.
///
/// # Returns
/// The total count of IDs that are contained in any range, encoded as `String`.
///
/// # Panics
/// Panics if the input format does not contain an empty line divider
/// or if ranges/IDs fail to parse.
pub fn solve(input: &str) -> String {
    let mut result: i32 = 0;

    let lines: Vec<&str> = input.lines().collect();
    let divider_index: usize = lines.iter().position(|&x| x == "").unwrap();

    'id: for id in lines[(divider_index + 1)..].iter() {
        let value: i64 = id.parse().unwrap();
        for range in lines[..divider_index].iter() {
            if is_id_in_range(value, range) {
                result += 1;
                continue 'id;
            }
        }
    }

    result.to_string()
}

/// Determines whether a given `id` falls within a numeric range defined as `"start-end"`.
///
/// # Arguments
/// * `id` – The value to check.
/// * `range` – A string slice containing the range in the format `"start-end"`.
///
/// # Returns
/// `true` if `id` is within the inclusive range, otherwise `false`.
///
/// # Panics
/// Panics if the range string cannot be split or parsed into valid integers.
fn is_id_in_range(id: i64, range: &str) -> bool {
    let values: Vec<&str> = range.split("-").collect();
    let start: i64 = values[0].parse().unwrap();
    let end: i64 = values[1].parse().unwrap();
    id >= start && id <= end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_id_in_range_case_1() {
        let id = 1;
        let range = "3-5";
        let result = is_id_in_range(id, range);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_id_in_range_case_2() {
        let id = 3;
        let range = "3-5";
        let result = is_id_in_range(id, range);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_id_in_range_case_3() {
        let id = 4;
        let range = "3-5";
        let result = is_id_in_range(id, range);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_id_in_range_case_4() {
        let id = 4;
        let range = "3-5";
        let result = is_id_in_range(id, range);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_id_in_range_case_5() {
        let id = 5;
        let range = "3-5";
        let result = is_id_in_range(id, range);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_id_in_range_case_6() {
        let id = 6;
        let range = "3-5";
        let result = is_id_in_range(id, range);
        assert_eq!(result, false);
    }

    #[test]
    fn test_solve() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let result = solve(input);
        assert_eq!(result, "3");
    }
}
