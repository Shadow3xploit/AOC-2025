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

/// Extracts columns from a whitespace-separated, row-oriented input string.
///
/// Each line in the input represents one row of values. The function splits all
/// lines by spaces, normalizes multiple spaces, and collects each field into a
/// temporary row matrix.  
/// Afterwards the matrix is transposed so that the returned vector contains
/// columns instead of rows.
///
/// # Arguments
/// * `input` – The raw puzzle input containing multiple rows of values.
///
/// # Returns
/// A vector where each element is a column represented as a `Vec<String>`.
fn extract_columns(input: &str) -> Vec<Vec<String>> {
    let mut columns: Vec<Vec<String>> = Vec::new();

    let mut tmp_read: Vec<Vec<String>> = Vec::new();
    for line in input.lines() {
        tmp_read.push(
            line.split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.to_string())
                .collect(),
        );
    }

    for _ in 0..tmp_read.first().unwrap().len() {
        columns.push(Vec::new());
    }

    for x in tmp_read {
        for (i, y) in x.iter().enumerate() {
            columns[i].push(y.to_string());
        }
    }

    columns
}

/// Performs a calculation over a column of values.
///
/// The column contains numbers followed by a trailing operator (`*` or `+`).  
/// All values except the last element are parsed as `i64`.  
/// If the final element is `"*"`, all numbers are multiplied.  
/// If the final element is `"+"`, all numbers are added.
///
/// # Arguments
/// * `column` – A vector of strings where the last element is the operator.
///
/// # Returns
/// The computed result as `i64`.
///
/// # Panics
/// * If any number cannot be parsed.
/// * If the column is empty.
fn perform_calculation(column: Vec<String>) -> i64 {
    let multiply: bool = column.last().unwrap() == "*";
    let mut result: i64 = column[0].parse().unwrap();
    for number in column[1..(column.len() - 1)].iter() {
        let parsed: i64 = number.parse().unwrap();
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
                    "45".to_string(),
                    "6".to_string(),
                    "*".to_string()
                ],
                vec![
                    "328".to_string(),
                    "64".to_string(),
                    "98".to_string(),
                    "+".to_string()
                ],
                vec![
                    "51".to_string(),
                    "387".to_string(),
                    "215".to_string(),
                    "*".to_string()
                ],
                vec![
                    "64".to_string(),
                    "23".to_string(),
                    "314".to_string(),
                    "+".to_string()
                ]
            ]
        );
    }

    #[test]
    fn test_perform_calculation_case_1() {
        let input = vec![
            "123".to_string(),
            "45".to_string(),
            "6".to_string(),
            "*".to_string(),
        ];
        let result = perform_calculation(input);
        assert_eq!(result, 33210);
    }

    #[test]
    fn test_perform_calculation_case_2() {
        let input = vec![
            "328".to_string(),
            "64".to_string(),
            "98".to_string(),
            "+".to_string(),
        ];
        let result = perform_calculation(input);
        assert_eq!(result, 490);
    }

    #[test]
    fn test_perform_calculation_case_3() {
        let input = vec![
            "51".to_string(),
            "387".to_string(),
            "215".to_string(),
            "*".to_string(),
        ];
        let result = perform_calculation(input);
        assert_eq!(result, 4243455);
    }

    #[test]
    fn test_perform_calculation_case_4() {
        let input = vec![
            "64".to_string(),
            "23".to_string(),
            "314".to_string(),
            "+".to_string(),
        ];
        let result = perform_calculation(input);
        assert_eq!(result, 401);
    }

    #[test]
    fn test_solve() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let result = solve(input);
        assert_eq!(result, "4277556");
    }
}
