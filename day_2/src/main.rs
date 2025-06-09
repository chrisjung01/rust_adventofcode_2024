fn main() {
    let input = include_str!("./input.txt");

    let mut safe_lines = 0;
    let total_lines = input.lines().count();

    println!("Analyzing {} sequences for safety...\n", total_lines);

    let safe_lines = input
        .lines()
        .filter(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            is_safe(&levels)
        })
        .count();

    println!("safe lines: {}", safe_lines);
}

fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    // Determine if the sequence should be increasing or decreasing based on first two numbers
    let is_increasing = levels[1] > levels[0];

    for i in 0..levels.len() - 1 {
        let current_diff = levels[i + 1] - levels[i];

        if levels[i] == levels[i + 1] {
            return false;
        }

        // Check if the direction matches the initial direction
        if (is_increasing && current_diff <= 0) || (!is_increasing && current_diff >= 0) {
            return false;
        }

        // Check if the difference is too large
        if current_diff.abs() > 3 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe() {
        // Test case 1: Safe because the levels are all decreasing by 1 or 2
        assert!(is_safe(&[7, 6, 4, 2, 1]));

        // Test case 2: Unsafe because 2 7 is an increase of 5
        assert!(!is_safe(&[1, 2, 7, 8, 9]));

        // Test case 3: Unsafe because 6 2 is a decrease of 4
        assert!(!is_safe(&[9, 7, 6, 2, 1]));

        // Test case 4: Unsafe because 1 3 is increasing but 3 2 is decreasing
        assert!(!is_safe(&[1, 3, 2, 4, 5]));

        // Test case 5: Safe because the levels are all increasing by 1, 2, or 3
        assert!(is_safe(&[1, 3, 6, 7, 9]));

        // Test case 6: Unsafe because 3 3 is not a valid sequence
        assert!(!is_safe(&[1, 3, 3, 6, 7]));
    }
}
