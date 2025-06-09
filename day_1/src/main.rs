use std::fs::File;
use std::io::{Error, ErrorKind, Read};

fn main() {
    let input = read_input_file("input.txt");
    // println!("{}", input);

    let lines = input.lines().collect::<Vec<&str>>();
    // println!("{:?}", lines);

    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    for line in lines {
        let parts = line.split("   ").collect::<Vec<&str>>();

        let left = parts[0];
        let right = parts[1];

        left_list.push(left.parse::<u32>().unwrap());
        right_list.push(right.parse::<u32>().unwrap());
    }

    // order the lists
    left_list.sort();
    right_list.sort();

    let result = compare_lists(left_list, right_list);
    println!("result: {:?}", result);
}

fn read_input_file(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn compare_lists(left_list: Vec<u32>, right_list: Vec<u32>) -> Result<u32, Error> {
    let mut sum = 0;

    // check if the lists are the same length
    if left_list.len() != right_list.len() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Lists are not the same length",
        ));
    }

    // get each item of the list and compare the difference between the two
    for i in 0..left_list.len() {
        let left = left_list[i];
        let right = right_list[i];
        sum += if left > right {
            left - right
        } else {
            right - left
        };
    }

    // check if the lists are the same
    Ok(sum)
}
