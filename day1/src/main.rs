pub mod tests;

use std::fs;

const FILE_PATH_LOCATION: &str =
    "/Users/codyabbott/Documents/GitHub/AdventOfCode23/day1/src/input-file.txt";

const NUMBER_STRING_ARRAY: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn generate_line_number(line: &str) -> u32 {
    let mut left_number: u32 = 0;
    let mut left_number_populated: bool = false;
    let mut right_number: u32 = 0;

    for c in line.chars() {
        if c.is_numeric() {
            let digit = c.to_digit(10).unwrap();
            if left_number_populated == false {
                left_number = digit;
            }
            right_number = digit;
            left_number_populated = true;
        }
    }

    return (left_number * 10) + right_number;
}

fn main() {
    println!("Reading input from file location: {}", FILE_PATH_LOCATION);

    let file_contents: String =
        fs::read_to_string(FILE_PATH_LOCATION).expect("Failed to open file!");

    let split_lines = file_contents.lines();

    let mut running_total: u32 = 0;
    for line in split_lines {
        running_total += generate_line_number(line);
    }

    println!("Total: {}", running_total);
}
