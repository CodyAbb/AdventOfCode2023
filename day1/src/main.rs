use std::fs;

const FILE_PATH_LOCATION: &str =
    "/Users/codyabbott/Documents/GitHub/AdventOfCode23/day1/src/input-file.txt";

fn generate_line_number(line: &str) -> u32 {
    let mut left_number = 0;
    let mut right_number = 0;

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

    let running_total: u32 = 0;
    for line in split_lines {
        let coordinate = generate_line_number(line);
    }
}
