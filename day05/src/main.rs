use helpers::read_input;
use std::num::ParseIntError;

type Result<T> = std::result::Result<T, String>;

fn parse_input(input: &str) -> Result<(Vec<Vec<i32>>, Vec<Vec<i32>>)> {
    let mut parts = input.split("\n\n");

    // Get the first and second parts
    let first_part = parts.next().ok_or("Missing first part")?;
    let second_part = parts.next().ok_or("Missing second part")?;

    // Parse the first part into a 2D array
    let first_part_parsed =
        parse_2d_array(first_part, '|').map_err(|e| format!("Error parsing first part: {}", e))?;

    // Parse the second part into a 2D array
    let second_part_parsed = parse_2d_array(second_part, ',')
        .map_err(|e| format!("Error parsing second part: {}", e))?;

    Ok((first_part_parsed, second_part_parsed))
}

fn parse_2d_array(input: &str, delimiter: char) -> Result<Vec<Vec<i32>>> {
    input
        .lines()
        .map(|line| {
            line.split(delimiter)
                .map(|num| num.trim().parse::<i32>())
                .collect::<std::result::Result<Vec<_>, ParseIntError>>()
                .map_err(|e| format!("Failed to parse number: {}", e))
        })
        .collect()
}

fn print_queue(str: &str) -> i32 {
    // Split input on double newline
    match parse_input(str) {
        Ok((first_part, second_part)) => {
            println!("First part: {:?}", first_part);
            println!("Second part: {:?}", second_part);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    0
}

fn main() {
    let input = read_input(5); // Day 5
    let result = print_queue(&input);

    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_queue() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let expected_result = 143;
        let result = print_queue(input);
        assert_eq!(result, expected_result);
    }
}
