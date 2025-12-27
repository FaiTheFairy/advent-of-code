use std::{fs, path::PathBuf};

use day6::{count_distinct, parse_input};

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = fs::read_to_string(path).expect("Couldn't read input file");
    let (grid, start) = parse_input(&input);
    let count_distinct = count_distinct(grid, start);
    println!("Part 1. The number of distinct tiles the guard will walk over = {count_distinct}");
}
