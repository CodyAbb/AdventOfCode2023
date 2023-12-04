pub mod tests;

use std::{collections::HashMap, fs};

const FILE_PATH_LOCATION: &str =
    "/Users/codyabbott/Documents/GitHub/AdventOfCode23/day1/src/input-file.txt";

pub fn generate_line_number(line: &str) -> u32 {
    let number_string_lookup: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut position_and_value: HashMap<u32, u32> = HashMap::new();

    for character in line.char_indices() {
        if character.1.is_numeric() {
            position_and_value.insert(
                character.0.try_into().unwrap(),
                character.1.to_digit(10).unwrap(),
            );
        }
    }

    for num in number_string_lookup.iter() {
        let mut first_position_index: u32 = 0;
        let mut last_position_index: u32 = 0;

        match line.find(num.0) {
            Some(index) => first_position_index = index.try_into().unwrap(),
            None => (),
        }
        match line.rfind(num.0) {
            Some(index) => last_position_index = index.try_into().unwrap(),
            None => (),
        }

        if first_position_index + last_position_index == 0 {
            continue;
        } else if first_position_index == last_position_index {
            position_and_value.insert(first_position_index, num.1.clone());
        } else {
            position_and_value.insert(first_position_index, num.1.clone());
            position_and_value.insert(last_position_index, num.1.clone());
        }
    }

    let mut ordered_position_and_value: Vec<(u32, u32)> = position_and_value.into_iter().collect();
    ordered_position_and_value.sort_by_key(|v| v.0);

    return (ordered_position_and_value[0].1 * 10)
        + ordered_position_and_value[ordered_position_and_value.len() - 1].1;
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
