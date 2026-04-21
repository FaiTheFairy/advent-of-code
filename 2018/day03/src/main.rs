#![allow(unused)]
use std::{collections::HashMap, fs, num::ParseIntError, str::FromStr};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input: Input = fs::read_to_string("input.txt")?.parse()?;

    let mut fabric = FabricGrid::default();
    fabric.add_claims(input.as_ref());

    let sol1 = fabric.count_overlapped_claims();
    println!("Part 1: {sol1}");

    let sol2 = fabric
        .find_intact(input.as_ref())
        .context("no intact claim found")?;
    println!("Part 2: {}", sol2.0);

    Ok(())
}

struct Input(Vec<Claim>);

impl AsRef<[Claim]> for Input {
    fn as_ref(&self) -> &[Claim] {
        &self.0
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

/// Holds information of all the coords and the number of times they overlap
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct FabricGrid(HashMap<Coord, usize>);

impl FabricGrid {
    fn find_intact(&self, claims: &[Claim]) -> Option<ClaimId> {
        claims
            .iter()
            .find(|claim| self.is_intact(claim))
            .map(|claim| claim.id)
    }

    fn is_intact(&self, claim: &Claim) -> bool {
        let coords = claim.coords();
        self.0
            .iter()
            .filter(|(coord, _count)| coords.contains(coord))
            .all(|(_coord, count)| *count == 1)
    }

    fn count_overlapped_claims(&self) -> usize {
        self.0.values().copied().filter(|count| *count >= 2).count()
    }

    fn add_claims(&mut self, claims: &[Claim]) {
        for claim in claims {
            self.add_one_claim(claim);
        }
    }

    fn add_one_claim(&mut self, claim: &Claim) {
        for coord in claim.coords() {
            self.0.entry(coord).and_modify(|e| *e += 1).or_insert(1);
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Claim {
    id: ClaimId,
    left_edge: usize,
    top_edge: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn coords(&self) -> Vec<Coord> {
        let mut out = Vec::with_capacity(self.width * self.height);

        for row in self.top_edge..self.top_edge + self.height {
            for col in self.left_edge..self.left_edge + self.width {
                out.push(Coord { row, col });
            }
        }

        out
    }
}

impl FromStr for Claim {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rest) = s
            .trim()
            .strip_prefix('#')
            .context("missing '#'")?
            .split_once(' ')
            .context("missing space after claim id")?;

        let id: ClaimId = id.parse()?;

        let (left_top, width_height) = rest
            .strip_prefix("@ ")
            .context("missing '@ '")?
            .split_once(": ")
            .context("missing ': ' between left-right and dimensions")?;

        let (left_edge, top_edge) = left_top.split_once(',').context("missing ','")?;
        let left_edge = left_edge.parse()?;
        let top_edge = top_edge.parse()?;

        let (width, height) = width_height.split_once('x').context("missing 'x'")?;
        let width = width.parse()?;
        let height = height.parse()?;

        Ok(Self {
            id,
            left_edge,
            top_edge,
            width,
            height,
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ClaimId(usize);

impl FromStr for ClaimId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Input {
        "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"
            .parse()
            .unwrap()
    }

    #[test]
    fn test_part_2() {
        let claims = example();
        let mut fabric = FabricGrid::default();
        fabric.add_claims(claims.as_ref());
        let result = fabric.find_intact(claims.as_ref()).unwrap();
        assert_eq!(result, ClaimId(3));
    }

    #[test]
    fn test_part_1() {
        let claims = example();
        let mut fabric = FabricGrid::default();
        fabric.add_claims(claims.as_ref());
        let result = fabric.count_overlapped_claims();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_parse_claim() {
        let result: Claim = "#123 @ 3,2: 5x4".parse().unwrap();
        let expected = Claim {
            id: ClaimId(123),
            left_edge: 3,
            top_edge: 2,
            width: 5,
            height: 4,
        };
        assert_eq!(result, expected);
    }
}
