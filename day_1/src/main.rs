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

    let total_distance = calculate_total_distance(&left_list, &right_list);
    let total_distance_2 = calculate_total_distance_2(&left_list, &right_list);

    println!("Total distance: {}", total_distance);
    println!("Total distance 2: {}", total_distance_2);

    Ok(())
}

fn calculate_total_distance(left_list: &[u32], right_list: &[u32]) -> u32 {
    let mut total_distance = 0u32;
    for i in 0..left_list.len() {
        let distance = if left_list[i] > right_list[i] {
            left_list[i] - right_list[i]
        } else {
            right_list[i] - left_list[i]
        };
        total_distance += distance;
    }
    total_distance
}

fn calculate_total_distance_2(left_list: &[u32], right_list: &[u32]) -> u32 {
    let mut total_distance_sum = 0;

    // get for loops left side
    for item in left_list {
        let mut founded_items = 0;
        for item2 in right_list {
            if item.eq(item2) {
                founded_items += 1;
            }
        }

        // multiple founded items by item
        let total_distance = item * founded_items;

        total_distance_sum += total_distance;
    }

    total_distance_sum
}
