/// Calculates the sum of all "invalid IDs" within the ranges specified in the input string.
///
/// # Arguments
///
/// * `input` - A string containing ranges separated by commas, e.g. `"11-22,95-115"`.
///   Each range is specified with a dash, e.g. `"11-22"`.
///
/// # Returns
///
/// A `String` containing the sum of all found "invalid IDs".
pub fn solve(input: &str) -> String {
    let mut result: i64 = 0;

    let ranges = input.split(",");
    for range in ranges {
        let ids: Vec<&str> = range.split('-').collect();
        for id in collect_invalid_ids_in_range(ids[0].parse().unwrap(), ids[1].parse().unwrap()) {
            result += id;
        }
    }

    result.to_string()
}

/// Returns a vector of all "invalid IDs" within a given range.
///
/// # Arguments
///
/// * `start` - The start of the range (inclusive)
/// * `end` - The end of the range (inclusive)
///
/// # Returns
///
/// A `Vec<i64>` containing all IDs in the range that are considered "invalid".
fn collect_invalid_ids_in_range(start: i64, end: i64) -> Vec<i64> {
    let mut numbers: Vec<i64> = Vec::new();

    for id in start..=end {
        if is_invalid_id(&id.to_string()) {
            numbers.push(id);
        }
    }

    numbers
}

/// Checks whether a given ID is considered "invalid".
///
/// An ID is considered invalid if it is made only of some sequence of digits
/// repeated at least twice. So, 12341234 (1234 two times), 123123123 (123 three times),
/// 1212121212 (12 five times), and 1111111 (1 seven times) are all invalid IDs.
///
/// # Arguments
///
/// * `id` - The ID as a string slice
///
/// # Returns
///
/// `true` if the ID is invalid, `false` otherwise
fn is_invalid_id(id: &str) -> bool {
    let length: usize = id.len();

    'elements_loop: for elements in 2..=length {
        if length % elements != 0 {
            continue;
        }

        let element_length = length / elements;

        for test_element in 1..elements {
            if id[0..element_length]
                != id[(element_length * test_element)
                    ..((element_length * test_element) + element_length)]
            {
                continue 'elements_loop;
            }
        }

        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_id_11() {
        assert!(is_invalid_id(&11.to_string()));
    }

    #[test]
    fn test_invalid_id_22() {
        assert!(is_invalid_id(&22.to_string()));
    }

    #[test]
    fn test_invalid_id_99() {
        assert!(is_invalid_id(&99.to_string()));
    }

    #[test]
    fn test_invalid_id_111() {
        assert!(is_invalid_id(&111.to_string()));
    }

    #[test]
    fn test_invalid_id_999() {
        assert!(is_invalid_id(&999.to_string()));
    }

    #[test]
    fn test_invalid_id_1010() {
        assert!(is_invalid_id(&1010.to_string()));
    }

    #[test]
    fn test_invalid_id_1188511885() {
        assert!(is_invalid_id(&1188511885.to_string()));
    }

    #[test]
    fn test_invalid_id_222222() {
        assert!(is_invalid_id(&222222.to_string()));
    }

    #[test]
    fn test_invalid_id_446446() {
        assert!(is_invalid_id(&446446.to_string()));
    }

    #[test]
    fn test_invalid_id_38593859() {
        assert!(is_invalid_id(&38593859.to_string()));
    }

    #[test]
    fn test_invalid_id_565656() {
        assert!(is_invalid_id(&565656.to_string()));
    }

    #[test]
    fn test_invalid_id_824824824() {
        assert!(is_invalid_id(&824824824.to_string()));
    }

    #[test]
    fn test_invalid_id_2121212121() {
        assert!(is_invalid_id(&2121212121.to_string()));
    }

    #[test]
    fn test_valid_id_12() {
        assert!(!is_invalid_id(&12.to_string()));
    }

    #[test]
    fn test_valid_id_123() {
        assert!(!is_invalid_id(&123.to_string()));
    }

    #[test]
    fn test_range_11_22() {
        assert_eq!(collect_invalid_ids_in_range(11, 22), vec![11, 22]);
    }

    #[test]
    fn test_range_95_115() {
        assert_eq!(collect_invalid_ids_in_range(95, 115), vec![99, 111]);
    }

    #[test]
    fn test_range_998_1012() {
        assert_eq!(collect_invalid_ids_in_range(998, 1012), vec![999, 1010]);
    }

    #[test]
    fn test_range_1188511880_1188511890() {
        assert_eq!(
            collect_invalid_ids_in_range(1188511880, 1188511890),
            vec![1188511885]
        );
    }

    #[test]
    fn test_range_222220_222224() {
        assert_eq!(collect_invalid_ids_in_range(222220, 222224), vec![222222]);
    }

    #[test]
    fn test_range_1698522_1698528() {
        assert_eq!(
            collect_invalid_ids_in_range(1698522, 1698528),
            Vec::<i64>::new()
        );
    }

    #[test]
    fn test_range_446443_446449() {
        assert_eq!(collect_invalid_ids_in_range(446443, 446449), vec![446446]);
    }

    #[test]
    fn test_range_38593856_38593862() {
        assert_eq!(
            collect_invalid_ids_in_range(38593856, 38593862),
            vec![38593859]
        );
    }

    #[test]
    fn test_range_565653_565659() {
        assert_eq!(collect_invalid_ids_in_range(565653, 565659), vec![565656]);
    }

    #[test]
    fn test_range_824824821_824824827() {
        assert_eq!(
            collect_invalid_ids_in_range(824824821, 824824827),
            vec![824824824]
        );
    }

    #[test]
    fn test_range_2121212118_2121212124() {
        assert_eq!(
            collect_invalid_ids_in_range(2121212118, 2121212124),
            vec![2121212121]
        );
    }

    #[test]
    fn test_solve() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = solve(input);
        assert_eq!(result, "4174379265");
    }
}
