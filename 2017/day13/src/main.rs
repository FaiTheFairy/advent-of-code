#![allow(unused)]

use std::{fs, iter::Sum, str::FromStr};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let firewall: Firewall = fs::read_to_string("input.txt")?.parse()?;

    let sol1 = firewall.trip_severity(Delay(0));
    println!("Part 1: {}", sol1.0);

    let sol2 = firewall
        .smallest_delay_not_caught(1_000_000_000)
        .context("no delay found in given range")?;
    println!("Part 2: {}", sol2.0);

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Firewall(Vec<Layer>);

impl Firewall {
    fn trip_severity(&self, delay: Delay) -> Severity {
        self.0
            .iter()
            .filter(|layer| layer.catches(delay))
            .map(|layer| layer.severity())
            .sum()
    }

    fn smallest_delay_not_caught(&self, max_tries: usize) -> Option<Delay> {
        (0..max_tries)
            .map(Delay)
            .find(|delay| !self.is_caught(*delay))
    }

    fn is_caught(&self, delay: Delay) -> bool {
        self.0.iter().any(|layer| layer.catches(delay))
    }
}

impl FromStr for Firewall {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let inner = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self(inner))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Layer {
    depth: Depth,
    range: Range,
}

impl Layer {
    fn new(depth: usize, range: usize) -> Self {
        Self {
            depth: Depth(depth),
            range: Range(range),
        }
    }

    fn severity(self) -> Severity {
        Severity(self.depth.0 * self.range.0)
    }

    /// Returns whether the scanner is at the top position when
    /// you reach it, given a delay before entering.
    fn catches(self, delay: Delay) -> bool {
        match self.range.0 {
            0 => false,
            1 => true,
            _ => {
                let arrival_time = delay.0 + self.depth.0;
                arrival_time.is_multiple_of(self.range.period().0)
            }
        }
    }
}

impl FromStr for Layer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (depth, range) = s
            .trim()
            .split_once(": ")
            .context("missing ': ' separator")?;
        let depth = Depth(depth.parse()?);
        let range = Range(range.parse()?);
        Ok(Self { depth, range })
    }
}

/// Layer index
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Depth(usize);

/// Scanner height
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Range(usize);

impl Range {
    fn period(self) -> Period {
        match self.0 {
            0 => Period(0),
            1 => Period(1),
            n => Period(2 * (n - 1)),
        }
    }
}

/// Waiting time before entering in picoseconds
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Delay(usize);

/// Total penalty
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Severity(usize);

impl Sum<Self> for Severity {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self(0), |a, b| Self(a.0 + b.0))
    }
}

/// Scanner cycle duration in picoseconds
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Period(usize);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0: 3
1: 2
4: 4
6: 4";

    fn firewall() -> Firewall {
        EXAMPLE.parse().unwrap()
    }

    #[test]
    fn test_trip_severity() {
        let result = firewall().trip_severity(Delay(0));
        assert_eq!(result, Severity(24));
    }

    #[test]
    fn test_parse_firewall() {
        let result = firewall();
        let expected = Firewall(vec![
            Layer::new(0, 3),
            Layer::new(1, 2),
            Layer::new(4, 4),
            Layer::new(6, 4),
        ]);
        assert_eq!(result, expected);
    }
}
