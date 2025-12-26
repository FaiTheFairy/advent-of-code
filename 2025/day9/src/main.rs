use std::{fs, path::PathBuf};

use day9::{max_area, parse_input};

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = fs::read_to_string(path).expect("Couldn't read file");
    let points = parse_input(&input);
    let max_area = max_area(&points);
    println!("Part 1. The max area formed is {max_area}");
}
