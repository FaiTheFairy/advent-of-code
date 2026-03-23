use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let modules: Vec<usize> = input
        .lines()
        .map(str::trim)
        .map(str::parse::<usize>)
        .collect::<Result<_, _>>()?;
    let sol1 = modules.iter().map(|x| x / 3 - 2).sum::<usize>();
    println!("Part 1. sum of the fuel requirements = {sol1}");

    // part 2
    let sol2 = modules.iter().copied().map(total_fuel).sum::<usize>();
    println!("Part 2. sum = {sol2}");

    Ok(())
}

fn total_fuel(mass: usize) -> usize {
    let mut total = 0;
    let mut current = mass;

    while current > 0 {
        let fuel = (current / 3).saturating_sub(2);
        total += fuel;
        current = fuel;
    }

    total
}
