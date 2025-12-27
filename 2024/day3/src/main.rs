use std::{fs, path::PathBuf};

use day3::{solve_part1, solve_part2};

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input = fs::read_to_string(path).expect("Couldn't read input file");
    let part1_ans = solve_part1(&input);
    println!("Part 1. The sum of the valid instructions = {part1_ans}");

    let part2_ans = solve_part2(&input);
    println!("Part 2. The sum of the valid instructions that are enabled = {part2_ans}");
}
