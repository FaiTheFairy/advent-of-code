use std::{fs, path::PathBuf};

use day8::{component_sizes, connect_n_closest, parse_input};

fn main() {
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = fs::read_to_string(filename).expect("Couldn't open file");

    let points = parse_input(&input);

    let mut dsu = connect_n_closest(&points, 1000);

    let mut sizes = component_sizes(&mut dsu);
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    let ans_part1 = sizes[0] * sizes[1] * sizes[2];
    println!(
        "Part 1. Multiplying together the sizes of the three largest circuits yields {ans_part1}"
    )
}
