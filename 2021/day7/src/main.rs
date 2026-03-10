use anyhow::{Context, Result};
use std::{fs, str::FromStr};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let swarm = input.parse::<CrabSwarm>()?;
    let sol1 = swarm.cost_of_cheapest_move()?;
    println!("Part 1. cost of aligning to most economical position = {sol1}");

    let sol2 = swarm.cost_of_cheapest_move_v2()?;
    println!("Part 2. cost of aligning to most economical position (v2) = {sol2}");

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CrabSwarm(Vec<Crab>);

impl CrabSwarm {
    fn cost_of_cheapest_move(&self) -> Result<usize> {
        self.cheapest_cost_by(Crab::cost_of_moving_to)
    }
    fn cost_of_cheapest_move_v2(&self) -> Result<usize> {
        self.cheapest_cost_by(Crab::cost_of_moving_to_v2)
    }

    fn cheapest_cost_by(&self, f: impl Fn(&Crab, usize) -> usize) -> Result<usize> {
        let max = self
            .0
            .iter()
            .map(|c| c.horizontal)
            .max()
            .context("empty positions")?;

        (0..=max)
            .map(|target| self.0.iter().map(|c| f(c, target)).sum())
            .min()
            .context("empty positions")
    }
}

impl FromStr for CrabSwarm {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let crabs = s
            .trim()
            .split(',')
            .map(str::parse::<usize>)
            .map(|r| r.map(|h| Crab { horizontal: h }))
            .collect::<Result<Vec<Crab>, _>>()?;

        Ok(Self(crabs))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Crab {
    horizontal: usize,
}

impl Crab {
    fn cost_of_moving_to(&self, position: usize) -> usize {
        self.horizontal.abs_diff(position)
    }

    fn cost_of_moving_to_v2(&self, position: usize) -> usize {
        // naive solution:
        // (1..=self.horizontal.abs_diff(position)).sum()
        // solution using triangular number formula
        let n = self.horizontal.abs_diff(position);
        n * (n + 1) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_parse_crab_swarm() {
        let result = EXAMPLE.parse::<CrabSwarm>().unwrap();
        let expected = CrabSwarm(vec![
            Crab { horizontal: 16 },
            Crab { horizontal: 1 },
            Crab { horizontal: 2 },
            Crab { horizontal: 0 },
            Crab { horizontal: 4 },
            Crab { horizontal: 2 },
            Crab { horizontal: 7 },
            Crab { horizontal: 1 },
            Crab { horizontal: 2 },
            Crab { horizontal: 14 },
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_crab_cost_of_moving() {
        let result = Crab { horizontal: 16 }.cost_of_moving_to(2);
        assert_eq!(result, 14)
    }

    #[test]
    fn test_swarm_least_fuel() {
        let result = EXAMPLE
            .parse::<CrabSwarm>()
            .unwrap()
            .cost_of_cheapest_move()
            .unwrap();
        assert_eq!(result, 37);
    }

    #[test]
    fn test_crab_cost_of_moving_v2() {
        let result = Crab { horizontal: 16 }.cost_of_moving_to_v2(5);
        assert_eq!(result, 66)
    }

    #[test]
    fn test_swarm_least_fuel_v2() {
        let result = EXAMPLE
            .parse::<CrabSwarm>()
            .unwrap()
            .cost_of_cheapest_move_v2()
            .unwrap();
        assert_eq!(result, 168);
    }
}
