use anyhow::Result;
use std::{
    collections::HashSet,
    fs,
    num::ParseIntError,
    ops::{Add, AddAssign},
    str::FromStr,
};

fn main() -> Result<()> {
    let input: Input = fs::read_to_string("input.txt")?.parse()?;

    let sol1 = Frequency(0).apply_changes(&input.0);
    println!("Part 1: {}", sol1.0);

    let sol2 = Frequency(0).first_frequency_reached_twice(&input.0);
    println!("Part 2: {}", sol2.0);

    Ok(())
}

struct Input(Vec<FrequencyChange>);

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self(inner))
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Frequency(i32);

impl Frequency {
    fn apply_changes(self, changes: &[FrequencyChange]) -> Self {
        changes.iter().fold(self, |acc, df| acc + *df)
    }

    fn first_frequency_reached_twice(self, changes: &[FrequencyChange]) -> Self {
        let mut current = self;
        let mut visited = HashSet::new();

        for df in changes.iter().cycle() {
            current += *df;
            if !visited.insert(current) {
                return current;
            }
        }
        unreachable!()
    }
}

impl Add<FrequencyChange> for Frequency {
    type Output = Self;

    fn add(self, rhs: FrequencyChange) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign<FrequencyChange> for Frequency {
    fn add_assign(&mut self, rhs: FrequencyChange) {
        self.0 += rhs.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct FrequencyChange(i32);

impl FromStr for FrequencyChange {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
