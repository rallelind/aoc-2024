use helpers::read_input;
use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
};

type Result<T> = std::result::Result<T, String>;

fn parse_input(input: &str) -> Result<(HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>)> {
    let mut parts = input.split("\n\n");

    // Get the first and second parts
    let first_part = parts.next().ok_or("Missing first part")?;
    let second_part = parts.next().ok_or("Missing second part")?;

    // Parse the first part into a 2D array
    let rules = parse_rules(first_part)?;
    // Parse the second part into a 2D array
    let groups = parse_2d_array(second_part, ',')
        .map_err(|e| format!("Error parsing second part: {}", e))?;

    Ok((rules, groups))
}

fn parse_rules(input: &str) -> Result<HashMap<i32, HashSet<i32>>> {
    let mut rules = HashMap::new();

    for line in input.lines() {
        let parts: Vec<i32> = line
            .split('|')
            .map(|num| num.trim().parse::<i32>())
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| format!("Error parsing rule: {}", e))?;

        if parts.len() != 2 {
            return Err(format!("Invalid rule: {}", line));
        }

        let (a, b) = (parts[0], parts[1]);
        rules.entry(a).or_insert_with(HashSet::new).insert(b);
    }

    Ok(rules)
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
    let mut sum_of_mid_values = 0;

    match parse_input(str) {
        Ok((rules, groups)) => {
            for group in groups {
                let positions: HashMap<i32, usize> = group
                    .iter()
                    .enumerate()
                    .map(|(idx, &val)| (val, idx))
                    .collect();

                let mut is_valid = true;

                // print positions
                for (a, b_set) in &rules {
                    for b in b_set {
                        let a_pos = positions.get(&a);
                        let b_pos = positions.get(&b);

                        // Rule violated if both `a` and `b` are present and `b` comes before `a`
                        if let (Some(&a_idx), Some(&b_idx)) = (a_pos, b_pos) {
                            if b_idx < a_idx {
                                is_valid = false;
                                break;
                            }
                        }
                    }
                    if !is_valid {
                        break;
                    }
                }

                if is_valid {
                    let mid_value = group[group.len() / 2];
                    sum_of_mid_values += mid_value;
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    sum_of_mid_values
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
