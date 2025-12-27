use std::{fs, path::PathBuf};

use day4::{count_x_mas, count_xmas, parse_input};

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = fs::read_to_string(path).expect("Couldn't read input file");
    let grid = parse_input(&input);
    let count_xmas = count_xmas(&grid);
    println!("Part 1. Number of times \"XMAS\" appears = {count_xmas}");

    let count_x_mas = count_x_mas(&grid);
    println!("Part 2. Number of times \"X-MAS\" appears (MAS in the shape of X) = {count_x_mas}");
}
