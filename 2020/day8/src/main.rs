use std::fs;

use anyhow::Result;
use day8::{solve_part_1, solve_part_2};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let sol1 = solve_part_1(&input)?;
    println!("Part 1. before looping, the value of the accumulator = {sol1}");

    let sol2 = solve_part_2(&input)?;
    println!("Part 2. value of accumulator after the fixed program terminates = {sol2}");

    Ok(())
}
