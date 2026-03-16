use std::fs;

use anyhow::Result;
use day9::{solve_part_1, solve_part_2};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let sol1 = solve_part_1(&input)?;
    println!("Part 1. first number that doesn't have the property = {sol1}");

    let sol2 = solve_part_2(&input)?;
    println!("Part 2. encryption weakness = {sol2}");

    Ok(())
}
