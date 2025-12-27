use std::{fs, path::PathBuf};

use day1::{compute_diffs, compute_similarity, parse_input};

fn main() {
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = fs::read_to_string(filename).expect("Couldn't read input file");
    let lists = parse_input(&input);
    let diffs = compute_diffs(&lists.0, &lists.1);
    let diffs_sum: u64 = diffs.iter().sum();
    println!(
        "Part 1. The sum of the differences computed between the two ordered lists = {diffs_sum}"
    );

    let similarity_score: u64 = compute_similarity(&lists.0, &lists.1);
    println!(
        "Part 2. The similarity score, computed as the running sum of the values in list 1 times the number of times they're repeated in list 2, = {}",
        similarity_score
    );
}
