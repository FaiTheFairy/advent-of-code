#![allow(dead_code)]
use std::str::FromStr;

use anyhow::{Context, Result, ensure};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let part_1: Input = input.parse()?;

    let sol1 = part_1.count_possible();
    println!("Part 1: {sol1}");

    let part_2: Input2 = input.parse()?;
    let sol2 = part_2.count_possible();
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Input2(Vec<PossibleTriangle>);

impl Input2 {
    fn count_possible(&self) -> usize {
        self.iter().filter(|t| t.is_triangle()).count()
    }

    fn iter(&self) -> impl Iterator<Item = PossibleTriangle> {
        self.0.iter().copied()
    }
}

impl FromStr for Input2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let len = s.lines().count();
        let mut cols: [_; 3] = std::array::from_fn(|_| Vec::with_capacity(len));

        for line in s.lines().map(str::trim).filter(|line| !line.is_empty()) {
            let row: Vec<u32> = line
                .split_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()?;

            cols[0].push(row[0]);
            cols[1].push(row[1]);
            cols[2].push(row[2]);
        }

        let mut triangles = Vec::with_capacity(len);
        for col in cols {
            let triangles_chunk: Vec<PossibleTriangle> = col
                .chunks(3)
                .map(|s| PossibleTriangle {
                    s1: s[0],
                    s2: s[1],
                    s3: s[2],
                })
                .collect();
            triangles.extend_from_slice(&triangles_chunk);
        }

        Ok(Self(triangles))
    }
}

struct Input(Vec<PossibleTriangle>);

impl Input {
    fn count_possible(&self) -> usize {
        self.iter().filter(|t| t.is_triangle()).count()
    }

    fn iter(&self) -> impl Iterator<Item = PossibleTriangle> {
        self.0.iter().copied()
    }
}

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct PossibleTriangle {
    s1: u32,
    s2: u32,
    s3: u32,
}

impl PossibleTriangle {
    fn is_triangle(self) -> bool {
        self.s1 + self.s2 > self.s3 && self.s2 + self.s3 > self.s1 && self.s1 + self.s3 > self.s2
    }
}

impl FromStr for PossibleTriangle {
    type Err = anyhow::Error;

    /// Parses "3 4 5" to `PossibleTriangle { s1: 3, s2: 4, s3: 5 }`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sides = s.split_whitespace().map(str::trim);

        let s1 = sides.next().context("triangle cannot be empty")?.parse()?;
        let s2 = sides
            .next()
            .context("triangle cannot consist of one side")?
            .parse()?;
        let s3 = sides
            .next()
            .context("triangle cannot consist of two sides")?
            .parse()?;

        ensure!(
            sides.next().is_none(),
            "triangle cannot have more than three sides"
        );

        Ok(Self { s1, s2, s3 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_2() {
        let result: Input2 = "3 4 5\n1 2 9\n7 6 8".parse().unwrap();
        let expected = Input2(vec![
            PossibleTriangle {
                s1: 3,
                s2: 1,
                s3: 7,
            },
            PossibleTriangle {
                s1: 4,
                s2: 2,
                s3: 6,
            },
            PossibleTriangle {
                s1: 5,
                s2: 9,
                s3: 8,
            },
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse() {
        let result: PossibleTriangle = "3 4 5".parse().unwrap();
        let expected = PossibleTriangle {
            s1: 3,
            s2: 4,
            s3: 5,
        };
        assert_eq!(result, expected);
        assert!(result.is_triangle());
    }
}
