use std::{fs, path::PathBuf};

use day8::{solve_part1, solve_part2};

fn main() {
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = fs::read_to_string(filename).expect("Couldn't open file");

    let ans_part1 = solve_part1(&input, 1000);
    println!(
        "Part 1. Multiplying together the sizes of the three largest circuits yields {ans_part1}"
    );

    let ans_part2 = solve_part2(&input);
    println!(
        "Part 2. Multiplying together the X coordinates of the last two junction boxes needed to connect to form one large circut yields {ans_part2}"
    );
}
