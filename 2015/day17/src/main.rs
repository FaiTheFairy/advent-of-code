use anyhow::Result;
use itertools::Itertools;
use std::str::FromStr;

fn main() -> Result<()> {
    let containers: Containers = std::fs::read_to_string("input.txt")?.parse()?;

    let sol1 = containers.count_combinations_that_store(150);
    println!("Part 1: {sol1}");

    let sol2 = containers.count_min_combinations_that_store(150);
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Containers {
    sizes: Vec<usize>,
}

impl Containers {
    fn count_combinations_that_store(&self, target: usize) -> usize {
        (1..=self.sizes.len())
            .flat_map(|k| self.sizes.iter().copied().combinations(k))
            .filter(|combo| combo.iter().sum::<usize>() == target)
            .count()
    }

    fn count_min_combinations_that_store(&self, target: usize) -> usize {
        let valid: Vec<Vec<usize>> = (1..=self.sizes.len())
            .flat_map(|k| self.sizes.iter().copied().combinations(k))
            .filter(|combo| combo.iter().sum::<usize>() == target)
            .collect();

        let min_len = valid.iter().map(|combo| combo.len()).min().unwrap();

        valid
            .into_iter()
            .filter(|combo| combo.len() == min_len)
            .count()
    }
}

impl FromStr for Containers {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let sizes = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self { sizes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "20\n15\n10\n5\n5";

    #[test]
    fn test_count_min_combinations_that_store() {
        let result = EXAMPLE
            .parse::<Containers>()
            .unwrap()
            .count_min_combinations_that_store(25);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_count_combinations_that_store() {
        let result = EXAMPLE
            .parse::<Containers>()
            .unwrap()
            .count_combinations_that_store(25);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_parse() {
        let result: Containers = EXAMPLE.parse().unwrap();
        let expected = Containers {
            sizes: vec![20, 15, 10, 5, 5],
        };

        assert_eq!(result, expected);
    }
}
