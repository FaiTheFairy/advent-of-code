use std::{fs, path::PathBuf};

use day2::{count_safe, count_safe_with_damper, parse_input};

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = fs::read_to_string(path).expect("Couldn't read input file");
    let reports = parse_input(&input);
    let count_safe = count_safe(&reports);
    println!(
        "Part 1. Number of safe reports (monotone, and difference between adjacent levels between 1 and 3) = {count_safe}"
    );

    let count_safe_with_damper = count_safe_with_damper(&reports);
    println!(
        "Part 2. Number of safe reports, if we can tolerate a single bad level in what would otherwise be a safe report, = {count_safe_with_damper}"
    );
}
