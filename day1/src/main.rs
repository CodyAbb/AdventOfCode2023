use std::fs;

const FILE_PATH_LOCATION: &str =
    "/Users/codyabbott/Documents/GitHub/AdventOfCode23/day1/src/input-file.txt";

fn main() {
    println!("Reading input from file location: {}", FILE_PATH_LOCATION);

    let file_contents = fs::read_to_string(FILE_PATH_LOCATION).expect("Failed to open file!");
    println!("{}", file_contents);
}
