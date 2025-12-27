use std::{fs, path::PathBuf};

use day7::{parse_input, total_calibration};

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = fs::read_to_string(path).expect("Couldn't open input file");
    let eqs = parse_input(&input);
    let total_calibration = total_calibration(&eqs);
    println!("Part 2. The total calibration = {total_calibration}");
}
