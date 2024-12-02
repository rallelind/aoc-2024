use helpers::read_input;

fn calculate_distance(input: &str) -> i32 {
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

    let mut total = 0;

    left.sort();
    right.sort();

    for i in 0..left.len() {
        total += (left[i] - right[i]).abs();
    }

    total
}

fn main() {
    let input = read_input(1); // Day 1
    let total_distance = calculate_distance(&input);
    println!("Distance: {}", total_distance);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distance() {
        let input = "1 4\n2 3\n5 6";
        let expected_distance = 5; // Calculated manually: |1-3| + |2-4| = 5
        let result = calculate_distance(input);
        assert_eq!(result, expected_distance);
    }
}
