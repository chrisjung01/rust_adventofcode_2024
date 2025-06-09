use std::io::{Error, ErrorKind};

fn main() -> Result<(), Error> {
    let input = include_str!("./input.txt");

    // Parse input into two separate vectors
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line
            .trim()
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .collect();

        if parts.len() != 2 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Each line must contain exactly two numbers, got: '{}'",
                    line
                ),
            ));
        }

        let left: u32 = parts[0].parse().map_err(|_| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Failed to parse left number: '{}'", parts[0]),
            )
        })?;

        let right: u32 = parts[1].parse().map_err(|_| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Failed to parse right number: '{}'", parts[1]),
            )
        })?;

        left_list.push(left);
        right_list.push(right);
    }

    left_list.sort();
    right_list.sort();

    let mut total_distance = 0u32;
    for i in 0..left_list.len() {
        let distance = if left_list[i] > right_list[i] {
            left_list[i] - right_list[i]
        } else {
            right_list[i] - left_list[i]
        };
        total_distance += distance;
    }

    println!("\nTotal number of pairs: {}", left_list.len());
    println!("Total distance: {}", total_distance);

    Ok(())
}
