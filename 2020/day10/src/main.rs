use std::fs;

use anyhow::Result;
use day10::{solve_part_1, solve_part_2};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let sol1 = solve_part_1(&input)?;
    println!("Part 1. product of 1-jolt diffs by 3-jolt diffs = {sol1}");

    let sol2 = solve_part_2(&input)?;
    println!("Part 2. adapters can be arranged in {sol2} ways");

    Ok(())
}
