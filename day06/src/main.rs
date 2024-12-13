#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn movement(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn amount_of_steps(input: &str) -> i32 {
    let parsed_input = parse_input(input);

    // go through the parsed input lines and find the position which will be ^ > v < and then go through the input and find the path
    // if the path is # then turn 90 degrees right, start by finding the starting position which can be one of the ones mentioned

    let mut direction = Direction::Up;
    let mut position = (0, 0);

    'outer: for (row_index, line) in parsed_input.iter().enumerate() {
        for (col_idx, &char) in line.iter().enumerate() {
            match char {
                '^' => {
                    direction = Direction::Up;
                    position = (row_index, col_idx);
                    break 'outer;
                }
                '>' => {
                    direction = Direction::Right;
                    position = (row_index, col_idx);
                    break 'outer;
                }
                'v' => {
                    direction = Direction::Down;
                    position = (row_index, col_idx);
                    break 'outer;
                }
                '<' => {
                    direction = Direction::Left;
                    position = (row_index, col_idx);
                    break 'outer;
                }
                _ => continue,
            }
        }
    }

    let mut amount_of_steps = 0;

    // Traverse the grid
    while let Some(row) = parsed_input.get(position.0) {
        if let Some(&cell) = row.get(position.1) {
            if cell == '#' {
                // Turn 90 degrees to the right
                direction = direction.turn_right();
            }

            // Move in the current direction
            let (dx, dy) = direction.movement();
            position = (
                (position.0 + dx) as usize,
                (position.1 + dy) as usize,
            );

            amount_of_steps += 1;
        } else {
            break; // Out of bounds
        }
    }

    amount_of_steps
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amount_of_steps() {
        let input = "....#.....
....^....#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...
";
        let result = amount_of_steps(input);
        assert_eq!(result, 46);
    }
}
