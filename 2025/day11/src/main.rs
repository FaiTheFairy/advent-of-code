use std::{fs, path::PathBuf};

use day11::{count_paths, count_paths_through_dac_and_fft, parse_input};

fn main() {
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = fs::read_to_string(filename).expect("Couldn't read input file");
    let graph = parse_input(&input);
    let count = count_paths(&graph, "you", "out");
    println!("Part 1. The number of paths that go from you to out = {count}");

    let count_visit_dat_fft = count_paths_through_dac_and_fft(&graph);
    println!(
        "Part 2. The number of paths that go through both `dat` and `fft` = {count_visit_dat_fft}"
    );
}
