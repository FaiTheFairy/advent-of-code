use std::{fs, path::PathBuf};

use day11::{Graph, count_paths, parse_input};

fn main() {
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = fs::read_to_string(filename).expect("Couldn't read input file");
    let graph = parse_input(&input);
    let count = count_paths(&graph, "you", "out");
    println!("Part 1. The number of paths that go from you to out = {count}");
}
