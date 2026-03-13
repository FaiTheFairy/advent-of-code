use std::{collections::HashSet, fs, str::FromStr};

use anyhow::{Result, anyhow};
use grid::*;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let height_map = input.parse::<HeightMap>()?;
    let sol1 = height_map.low_points_score()?;
    println!("Part 1. sum of the risk of all low points is {sol1}");

    let mut basins = height_map.basin_sizes();
    basins.sort_unstable_by(|a, b| b.cmp(a));
    let sol2: usize = basins.iter().take(3).product();
    println!("Part 2. product of sizes of the three largest basins = {sol2}");

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HeightMap(Grid<u8>);

impl HeightMap {
    fn get(&self, row: usize, col: usize) -> Option<&u8> {
        self.0.get(row, col)
    }

    fn is_low_point(&self, row: usize, col: usize) -> Result<bool> {
        let point = *self
            .get(row, col)
            .ok_or_else(|| anyhow!("Point ({row}, {col}) doesn't exist"))?;

        let neighbors = [
            row.checked_sub(1).and_then(|r| self.get(r, col)),
            self.get(row + 1, col),
            col.checked_sub(1).and_then(|c| self.get(row, c)),
            self.get(row, col + 1),
        ];

        Ok(neighbors
            .into_iter()
            .flatten()
            .all(|&neighbor| point < neighbor))
    }

    fn low_points(&self) -> Result<Vec<(usize, usize)>> {
        let mut out = Vec::new();
        let (row, col) = self.0.size();
        for r in 0..row {
            for c in 0..col {
                if self.is_low_point(r, c)? {
                    out.push((r, c));
                }
            }
        }
        Ok(out)
    }

    fn low_points_score(&self) -> Result<usize> {
        let mut score = 0;

        for (row, col) in self.low_points()? {
            score += *self
                .get(row, col)
                .expect("low point not found. map has been changed?") as usize
                + 1;
        }
        Ok(score)
    }

    fn neighbors(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
        [
            row.checked_sub(1).map(|r| (r, col)),
            Some((row + 1, col)),
            col.checked_sub(1).map(|c| (row, c)),
            Some((row, col + 1)),
        ]
        .into_iter()
        .flatten()
        .filter(|&(r, c)| self.get(r, c).is_some())
    }

    fn basin_size(&self, row: usize, col: usize) -> Result<usize> {
        let start = *self
            .get(row, col)
            .ok_or_else(|| anyhow!("Point ({row}, {col}) doesn't exist"))?;

        if start == 9 {
            return Ok(0);
        }

        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut stack = vec![(row, col)];

        while let Some((r, c)) = stack.pop() {
            if visited.contains(&(r, c)) {
                continue;
            }

            let height = *self
                .get(r, c)
                .ok_or_else(|| anyhow!("Point ({r}, {c}) doesn't exist"))?;

            if height == 9 {
                continue;
            }

            visited.insert((r, c));

            for (nr, nc) in self.neighbors(r, c) {
                if !visited.contains(&(nr, nc)) {
                    stack.push((nr, nc));
                }
            }
        }

        Ok(visited.len())
    }

    fn basin_sizes(&self) -> Vec<usize> {
        let mut out = Vec::new();

        for row in 0..self.0.rows() {
            for col in 0..self.0.cols() {
                if self.is_low_point(row, col).unwrap_or(false) {
                    out.push(self.basin_size(row, col).unwrap());
                }
            }
        }

        out
    }
}

impl FromStr for HeightMap {
    type Err = anyhow::Error;

    /// Parses 2d height_map
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = grid![];

        for line in s.trim().lines() {
            let row: Vec<u8> = line
                .bytes()
                .map(|b| {
                    if b.is_ascii_digit() {
                        Ok(b - b'0')
                    } else {
                        Err(anyhow!("invalid digit: {}", b as char))
                    }
                })
                .collect::<Result<_, _>>()?;

            grid.push_row(row);
        }

        Ok(Self(grid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    fn height_map() -> HeightMap {
        EXAMPLE.parse().unwrap()
    }

    #[test]
    fn test_parse_grid() {
        let result: HeightMap = height_map();
        let expected: HeightMap = HeightMap(grid![
            [2, 1, 9, 9, 9, 4, 3, 2, 1, 0]
            [3, 9, 8, 7, 8, 9, 4, 9, 2, 1]
            [9, 8, 5, 6, 7, 8, 9, 8, 9, 2]
            [8, 7, 6, 7, 8, 9, 6, 7, 8, 9]
            [9, 8, 9, 9, 9, 6, 5, 6, 7, 8]
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_low_point() {
        let low_point_is_low = height_map().is_low_point(0, 1).unwrap();
        assert!(low_point_is_low);

        let high_point_is_low = height_map().is_low_point(1, 4).unwrap();
        assert!(!high_point_is_low);
    }

    #[test]
    fn test_low_points() {
        let low_points = height_map().low_points().unwrap();
        let expected = vec![(0, 1), (0, 9), (2, 2), (4, 6)];
        assert_eq!(low_points, expected)
    }

    #[test]
    fn test_low_points_score() {
        let result = height_map().low_points_score().unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_basin_sizes() {
        let mut basins = height_map().basin_sizes();
        basins.sort_unstable_by(|a, b| b.cmp(a));
        let answer: usize = basins.iter().take(3).product();
        assert_eq!(answer, 1134);
    }
}
