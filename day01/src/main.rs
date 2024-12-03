use helpers::read_input;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for row in input.trim().split('\n') {
        let nums: Vec<i32> = row
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        left.push(nums[0]);
        right.push(nums[1]);
    }

    left.sort();
    right.sort();

    (left, right)
}

fn calculate_distance(input: &str) -> i32 {
    let (left, right) = parse_input(input);

    let mut total = 0;

    for i in 0..left.len() {
        total += (left[i] - right[i]).abs();
    }

    total
}

fn calculate_distance_with_similarity(input: &str) -> i32 {
    let (left, right) = parse_input(input);

    let mut total = 0;

    for i in 0..left.len() {
        let occurences = right.iter().filter(|&n| n == &left[i]).count() as i32;
        total += (left[i] * occurences).abs()
    }

    total
}

fn main() {
    let input = read_input(1); // Day 1
    let total_distance_p1 = calculate_distance(&input);
    let total_distance_p2 = calculate_distance_with_similarity(&input);
    println!("Distance part 1: {}", total_distance_p1);
    println!("Distance part 2: {}", total_distance_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distance() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let expected_distance = 11;
        let result = calculate_distance(input);
        assert_eq!(result, expected_distance);
    }

    #[test]
    fn test_calculate_distance_with_similarity() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let expected_distance = 31;
        let result = calculate_distance_with_similarity(input);
        assert_eq!(result, expected_distance);
    }
}
