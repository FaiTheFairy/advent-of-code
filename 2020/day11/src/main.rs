use std::fs;

use anyhow::Result;
use day11::{RULES_1, RULES_2};
use day11::{solve_part_1, solve_part_2};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let sol1 = solve_part_1(&input)?;
    println!("Part 1. {sol1} remain occupied with rule {RULES_1:?}.");

    let sol2 = solve_part_2(&input)?;
    println!("Part 2. {sol2} remain occupied with rule {RULES_2:?}.");

    Ok(())
}
