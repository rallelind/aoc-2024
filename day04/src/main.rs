use helpers::read_input;

fn count_xmas(input: &str) -> i32 {
    // each line should be an array of characters
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut count = 0;

    let directions = [
        (0, 1),   // right
        (1, 0),   // down
        (1, 1),   // diagonal down right
        (-1, 1),  // diagonal down left
        (0, -1),  // left
        (-1, 0),  // up
        (-1, -1), // diagonal up left
        (1, -1),  // diagonal up right
    ];

    // go through rows and columns and check if the word XMAS is present
    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            for &(row_delta, col_delta) in &directions {
                let target = ['X', 'M', 'A', 'S'];
                let mut found = true;

                for (i, &char) in target.iter().enumerate() {
                    let new_row = row as i32 + row_delta * i as i32;
                    let new_col = col as i32 + col_delta * i as i32;

                    if new_row < 0
                        || new_row >= lines.len() as i32
                        || new_col < 0
                        || new_col >= lines[row].len() as i32
                        || lines[new_row as usize][new_col as usize] != char
                    {
                        found = false;
                        break;
                    }
                }

                if found {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    let input = read_input(4);
    let result = count_xmas(&input);

    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xmas_count() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let expected_count = 18;
        let result = count_xmas(input);
        assert_eq!(result, expected_count);
    }
}
