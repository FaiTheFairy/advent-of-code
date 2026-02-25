use std::{fs, str::FromStr};

use anyhow::{Context, Result, anyhow};

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt")?;
    let list = input.parse::<List>()?;
    let max = list.sum_of_calories_of_max_elf()?;
    println!("Part 1. The elf with the most calories is carrying {max} calories");

    let max3 = list.sum_of_calories_of_max_3_elves();
    println!("Part 2. The three elves with the most calories are carrying {max3} calories");
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct List {
    elves: Vec<Elf>,
}

impl List {
    fn sum_of_calories_of_max_elf(&self) -> Result<usize> {
        self.elves
            .iter()
            .map(Elf::total_calories)
            .max()
            .ok_or_else(|| anyhow!("no elves in input"))
    }

    fn sum_of_calories_of_max_3_elves(&self) -> usize {
        let mut calories = self
            .elves
            .iter()
            .map(Elf::total_calories)
            .collect::<Vec<_>>();
        calories.sort_unstable();
        calories.iter().rev().take(3).sum()
    }
}

impl FromStr for List {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let elves = s
            .split("\n\n")
            .map(str::trim)
            .filter(|chunk| !chunk.is_empty())
            .map(|chunk| chunk.parse::<Elf>().context("failed to parse elf"))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { elves })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Elf {
    items: Vec<Item>,
}

impl Elf {
    fn total_calories(&self) -> usize {
        self.items.iter().map(|i| i.calories).sum()
    }
}

impl FromStr for Elf {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let items = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<Item>().context("failed to parse item"))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { items })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Item {
    calories: usize,
}

impl FromStr for Item {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let calories = s
            .trim()
            .parse::<usize>()
            .with_context(|| format!("invalid calories: {s:?}"))?;
        Ok(Self { calories })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_parse_input() {
        let result = EXAMPLE.parse::<List>().unwrap();
        let expected = List {
            elves: vec![
                Elf {
                    items: vec![
                        Item { calories: 1000 },
                        Item { calories: 2000 },
                        Item { calories: 3000 },
                    ],
                },
                Elf {
                    items: vec![Item { calories: 4000 }],
                },
                Elf {
                    items: vec![Item { calories: 5000 }, Item { calories: 6000 }],
                },
                Elf {
                    items: vec![
                        Item { calories: 7000 },
                        Item { calories: 8000 },
                        Item { calories: 9000 },
                    ],
                },
                Elf {
                    items: vec![Item { calories: 10000 }],
                },
            ],
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn test_max_calories() {
        let list = EXAMPLE.parse::<List>().unwrap();
        let max = list.sum_of_calories_of_max_elf().unwrap();
        assert_eq!(max, 24_000);
    }

    #[test]
    fn test_max_3_calories() {
        let list = EXAMPLE.parse::<List>().unwrap();
        let max = list.sum_of_calories_of_max_3_elves();
        assert_eq!(max, 45_000);
    }
}
