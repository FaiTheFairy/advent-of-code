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
    fn cost_of_moving_to(&self, position: usize) -> usize {
        self.0.iter().map(|c| c.cost_of_moving_to(position)).sum()
    }

    fn cost_of_moving_to_v2(&self, position: usize) -> usize {
        self.0
            .iter()
            .map(|c| c.cost_of_moving_to_v2(position))
            .sum()
    }

    fn cost_of_cheapest_move(&self) -> Result<usize> {
        let max = self
            .0
            .iter()
            .map(|c| c.horizontal)
            .max()
            .context("empty positions")?;

        let mut ideal = self.cost_of_moving_to(max);
        for i in 0..max {
            ideal = ideal.min(self.cost_of_moving_to(i));
        }
        Ok(ideal)
    }
    fn cost_of_cheapest_move_v2(&self) -> Result<usize> {
        let max = self
            .0
            .iter()
            .map(|c| c.horizontal)
            .max()
            .context("empty positions")?;

        let mut ideal = self.cost_of_moving_to_v2(max);
        for i in 0..max {
            ideal = ideal.min(self.cost_of_moving_to_v2(i));
        }
        Ok(ideal)
    }
}

impl FromStr for CrabSwarm {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim()
                .split(',')
                .flat_map(|d| d.parse::<usize>())
                .map(|d| Crab { horizontal: d })
                .collect::<Vec<Crab>>(),
        ))
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
        (1..=self.horizontal.abs_diff(position)).sum()
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
    fn test_swarm_cost_of_moving() {
        let result = EXAMPLE.parse::<CrabSwarm>().unwrap().cost_of_moving_to(2);
        assert_eq!(result, 37);
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
    fn test_swarm_cost_of_moving_v2() {
        let result = EXAMPLE
            .parse::<CrabSwarm>()
            .unwrap()
            .cost_of_moving_to_v2(2);
        assert_eq!(result, 206);
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
