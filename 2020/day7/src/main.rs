use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let sol1 = day7::solve_part_1(&input)?;
    println!("Part 1. number of bag colors that can contain at least one shiny gold bag = {sol1}");

    let sol2 = day7::solve_part_2(&input)?;
    println!("Part 2. number of bag colors required inside a shiny gold bag = {sol2}");

    Ok(())
}
