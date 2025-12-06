/// Solves the puzzle by processing all columns and summing their results.
///
/// The function splits the input into columns using `extract_columns`, then
/// runs `perform_calculation` on each column. All results are summed and
/// returned as a string.
///
/// # Arguments
/// * `input` – The raw puzzle input.
///
/// # Returns
/// A string containing the final sum of all column computations.
pub fn solve(input: &str) -> String {
    let mut result: i64 = 0;

    for column in extract_columns(input) {
        result += perform_calculation(column);
    }

    result.to_string()
}

/// Extracts vertical columns from the given multiline input.
/// 
/// Columns are detected by scanning the last line for non-space
/// characters, which mark the starting indices. Each column
/// consists of substrings taken from every line between two
/// detected column boundaries.
///
/// # Arguments
/// * `input` - The raw multiline input string.
///
/// # Returns
/// A vector of columns, where each column is a vector of strings.
/// Each inner string represents the slice of one line belonging
/// to that column.
fn extract_columns(input: &str) -> Vec<Vec<String>> {
    let mut columns: Vec<Vec<String>> = Vec::new();

    let lines: Vec<&str> = input.lines().collect();

    let mut collum_start_indicies: Vec<usize> = Vec::new();
    for i in 0..lines.last().unwrap().len() {
        if lines.last().unwrap()[i..=i] != " ".to_string() {
            collum_start_indicies.push(i);
        }
    }

    for i in 0..collum_start_indicies.len() {
        let mut column: Vec<String> = Vec::new();

        let start: usize = collum_start_indicies[i];
        let end: usize;
        if i == collum_start_indicies.len() - 1 {
            end = lines.last().unwrap().len();
        } else {
            end = collum_start_indicies[i + 1] - 1;
        }

        for line in &lines {
            column.push(line[start..end].to_string());
        }

        columns.push(column);
    }

    columns
}

/// Performs the calculation for a single column. 
/// 
/// The last row determines whether the operation is addition (`+`) or
/// multiplication (`*`). All other rows contain digits aligned
/// vertically, which are combined into full numbers per column
/// before applying the operation.
///
/// # Arguments
/// * `column` - A vector of strings representing one extracted column.
///
/// # Returns
/// The evaluated result as `i64`.
fn perform_calculation(column: Vec<String>) -> i64 {
    let multiply: bool = column.last().unwrap().trim() == "*";

    let mut numbers: Vec<String> = Vec::new();
    for _ in 0..column.first().unwrap().len() {
        numbers.push("".to_string());
    }
    for line in column[0..(column.len() - 1)].iter() {
        for i in 0..line.len() {
            numbers[i] = numbers[i].to_owned() + &line[i..=i];
        }
    }

    let mut result: i64 = numbers[0].trim().parse().unwrap();
    for number in numbers[1..].iter() {
        let parsed: i64 = number.trim().parse().unwrap();
        if multiply {
            result *= parsed;
        } else {
            result += parsed;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_columns() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let result = extract_columns(input);
        assert_eq!(
            result,
            vec![
                vec![
                    "123".to_string(),
                    " 45".to_string(),
                    "  6".to_string(),
                    "*  ".to_string()
                ],
                vec![
                    "328".to_string(),
                    "64 ".to_string(),
                    "98 ".to_string(),
                    "+  ".to_string()
                ],
                vec![
                    " 51".to_string(),
                    "387".to_string(),
                    "215".to_string(),
                    "*  ".to_string()
                ],
                vec![
                    "64 ".to_string(),
                    "23 ".to_string(),
                    "314".to_string(),
                    "+  ".to_string()
                ]
            ]
        );
    }

    #[test]
    fn test_perform_calculation_case_1() {
        let input = vec![
            "123".to_string(),
            " 45".to_string(),
            "  6".to_string(),
            "*  ".to_string(),
        ];
        let result = perform_calculation(input);
        assert_eq!(result, 8544);
    }

    #[test]
    fn test_perform_calculation_case_2() {
        let input = vec![
            "328".to_string(),
            "64 ".to_string(),
            "98 ".to_string(),
            "+  ".to_string(),
        ];
        let result = perform_calculation(input);
        assert_eq!(result, 625);
    }

    #[test]
    fn test_perform_calculation_case_3() {
        let input = vec![
            " 51".to_string(),
            "387".to_string(),
            "215".to_string(),
            "*  ".to_string(),
        ];
        let result = perform_calculation(input);
        assert_eq!(result, 3253600);
    }

    #[test]
    fn test_perform_calculation_case_4() {
        let input = vec![
            "64 ".to_string(),
            "23 ".to_string(),
            "314".to_string(),
            "+  ".to_string(),
        ];
        let result = perform_calculation(input);
        assert_eq!(result, 1058);
    }

    #[test]
    fn test_solve() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let result = solve(input);
        assert_eq!(result, "3263827");
    }
}
