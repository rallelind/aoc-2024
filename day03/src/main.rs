use helpers::read_input;

fn extract_and_multiply(input: String) -> i32 {
    // regex to extract the following match mul(5,5)
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut total = 0;

    for cap in re.captures_iter(&input) {
        let a: i32 = cap[1].parse().unwrap();
        let b: i32 = cap[2].parse().unwrap();
        total += a * b;
    }

    total
}

fn extract_and_multiply_conditionally(input: String) -> i32 {
    // regex to extract if a do or don't and then the mul or just the mul
    let mut total = 0;

    total
}

fn main() {
    let input = read_input(3); // Day 3
    let result = extract_and_multiply(input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_and_multiply() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected_result = 2 * 4 + 5 * 5 + 11 * 8 + 8 * 5;
        let result = extract_and_multiply(input.to_string());
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_extract_and_multiply_conditionally() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected_result = 2 * 4 + 8 * 5;
        let result = extract_and_multiply_conditionally(input.to_string());
        assert_eq!(result, expected_result);
    }
}