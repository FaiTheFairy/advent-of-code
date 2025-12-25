use std::{fs, path::PathBuf};

use day7::{count_quantum_paths, count_splits, parse_input};

fn main() {
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = fs::read_to_string(filename).expect("Unable to open file");
    let grid = parse_input(&input);
    let count_splits = count_splits(&grid);
    println!("Part 1. The number of splits the beam goes through = {count_splits}");

    let count_quantum_paths = count_quantum_paths(&grid);
    println!(
        "Part 2. The number of timelines a single tachyon particle would end up on = {count_quantum_paths}"
    );
}
