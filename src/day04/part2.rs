/// Computes the total number of removed rolls based on the puzzle rules.
///
/// The function performs the following steps:
/// 1. Parses the input into a two-dimensional boolean grid, where `true`
///    represents an `'@'`.
/// 2. Adds a padding border around the grid to avoid bounds checking.
/// 3. Repeatedly scans the inner area of the grid and removes any `true` cell
///    whose number of adjacent `true` neighbors (8-directional) is below 4.
///    Each removal increases the result counter.
/// 4. The process continues until a full pass removes nothing.
///
/// # Arguments
/// * `input` – A multiline string representing the raw puzzle input.
///
/// # Returns
/// A string containing the total number of removed rolls.
pub fn solve(input: &str) -> String {
    let mut result: i32 = 0;

    let mut grid: Vec<Vec<bool>> = parse_input_to_bool_grid(input);
    pad_grid(&mut grid);

    let height: usize = grid.len();
    let width: usize = grid[0].len();
    loop {
        let mut removed = false;
        for h in 1..(height - 1) {
            for w in 1..(width - 1) {
                if !grid[h][w] {
                    continue;
                }

                if count_rolls_around_position(&grid, h, w) < 4 {
                    result += 1;
                    grid[h][w] = false;
                    removed = true;
                }
            }
        }
        if !removed {
            break;
        }
    }

    result.to_string()
}

/// Parses the given input string into a two-dimensional boolean grid.
///
/// Each line in the input becomes one row in the grid.  
/// Each character in a line is converted to `true` if it is `'@'`
/// and `false` otherwise.
///
/// The function returns a `Vec<Vec<bool>>` where all rows have the same
/// length as their corresponding input lines.
///
/// # Arguments
/// * `input` – The raw multiline string to parse.
fn parse_input_to_bool_grid(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line: &str| line.chars().map(|c: char| c == '@').collect::<Vec<bool>>())
        .collect()
}

/// Adds a padding border around a two-dimensional boolean grid.
///
/// A new row is inserted at the top and bottom,
/// and each existing row receives a new first and last element.
/// All padding cells are filled with false.
///
/// This operation modifies the grid in place.
///
/// # Arguments
/// * `grid` – The grid to be extended.
fn pad_grid(grid: &mut Vec<Vec<bool>>) {
    let width = grid[0].len();

    grid.insert(0, vec![false; width]);

    grid.push(vec![false; width]);

    for row in grid.iter_mut() {
        row.insert(0, false);
        row.push(false);
    }
}

/// Counts how many of the eight surrounding cells contain a roll (`true`).
///
/// The function looks at all 8 neighbors of the given position:
/// top-left, top, top-right, left, right, bottom-left, bottom,
/// and bottom-right.
///
/// The grid is expected to already contain padding, ensuring that all
/// neighboring indices are valid.
///
/// # Arguments
/// * `grid` – A two-dimensional boolean grid.
/// * `h` – The row index of the cell.
/// * `w` – The column index of the cell.
///
/// # Returns
/// The number of surrounding cells that contain `true`.
fn count_rolls_around_position(grid: &Vec<Vec<bool>>, h: usize, w: usize) -> i32 {
    let mut count: i32 = 0;
    if grid[h - 1][w - 1] {
        count += 1;
    }
    if grid[h - 1][w] {
        count += 1;
    }
    if grid[h - 1][w + 1] {
        count += 1;
    }
    if grid[h][w - 1] {
        count += 1;
    }
    if grid[h][w + 1] {
        count += 1;
    }
    if grid[h + 1][w - 1] {
        count += 1;
    }
    if grid[h + 1][w] {
        count += 1;
    }
    if grid[h + 1][w + 1] {
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        let result = solve(input);
        assert_eq!(result, "43");
    }
}
