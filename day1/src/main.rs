pub mod tests;

use std::fs;

const FILE_PATH_LOCATION: &str =
    "/Users/codyabbott/Documents/GitHub/AdventOfCode23/day1/src/input-file.txt";

pub fn generate_line_number(line: &str) -> u32 {
    let mut left_number: u32 = 0;
    let mut right_number: u32 = 0;

    for c in line.chars() {
        if c.is_numeric() {
            let digit = c.to_digit(10).unwrap();
            left_number = digit;
            break;
        }
    }

    return (left_number * 10) + right_number;
}

fn main() {
    println!("Reading input from file location: {}", FILE_PATH_LOCATION);

    let file_contents = fs::read_to_string(FILE_PATH_LOCATION).expect("Failed to open file!");
    println!("{}", file_contents);

    let split_lines = file_contents.lines();

    let mut running_total: u32 = 0;
    for line in split_lines {
        running_total += generate_line_number(line);
    }

    println!("Total: {}", running_total);
}
