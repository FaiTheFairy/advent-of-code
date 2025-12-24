use std::{fs, path::PathBuf};

use day6::{calculate_columns, calculate_sum_of_results, parse_input};

fn main() {
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = fs::read_to_string(filename).expect("Couldn't read file");
    let columns = parse_input(&input);
    let results = calculate_columns(columns);
    let sum = calculate_sum_of_results(results);
    println!(
        "Part 1. The total found by adding all of the answers to the individual problems = {sum}"
    );
}
