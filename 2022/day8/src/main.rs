use std::{fs, str::FromStr};

use anyhow::Context;
use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt")?;
    let trees = input.parse::<Trees>()?;
    let sol1 = trees.count_visible_trees();
    println!("Part 1. Number of trees visible from outside the grid = {sol1}");

    let sol2 = trees.highest_scenic_score()?;
    println!("Part 2. Highest scenic score possible = {sol2}");
    Ok(())
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq, Eq))]
struct Trees {
    // 2d grid of tree lengths from 0 (shortest) to 9
    grid: Vec<u8>,
    width: usize,
}

impl Trees {
    fn idx(&self, row: usize, col: usize) -> usize {
        col + row * self.width
    }

    fn coords(&self, idx: usize) -> (usize, usize) {
        let row = idx / self.width;
        let col = idx % self.width;
        (row, col)
    }

    fn get(&self, row: usize, col: usize) -> Option<u8> {
        self.grid.get(self.idx(row, col)).copied()
    }

    fn row(&self, row: usize) -> &[u8] {
        let w = self.width;
        &self.grid[row * w..(row + 1) * w]
    }

    fn col(&self, col: usize) -> impl Iterator<Item = u8> {
        self.grid.iter().skip(col).step_by(self.width).copied()
    }

    fn left(&self, row: usize, col: usize) -> impl Iterator<Item = u8> {
        self.row(row).iter().take(col).copied().rev()
    }
    fn right(&self, row: usize, col: usize) -> impl Iterator<Item = u8> {
        self.row(row).iter().skip(col + 1).copied()
    }

    fn up(&self, row: usize, col: usize) -> impl Iterator<Item = u8> {
        (0..row).rev().map(move |r| self.grid[self.idx(r, col)])
    }

    fn down(&self, row: usize, col: usize) -> impl Iterator<Item = u8> {
        self.col(col).skip(row + 1)
    }

    fn is_tree_visible(&self, row: usize, col: usize) -> bool {
        let height = match self.get(row, col) {
            Some(h) => h,
            None => return false,
        };

        let visible_left = self.left(row, col).all(|t| t < height);
        let visible_right = self.right(row, col).all(|t| t < height);
        let visible_up = self.up(row, col).all(|t| t < height);
        let visible_down = self.down(row, col).all(|t| t < height);

        visible_right || visible_left || visible_up || visible_down
    }

    fn count_visible_trees(&self) -> usize {
        let mut count = 0usize;
        for idx in 0..self.grid.len() {
            let (row, col) = self.coords(idx);
            if self.is_tree_visible(row, col) {
                count += 1;
            }
        }
        count
    }

    fn scenic_score(&self, row: usize, col: usize) -> Result<usize> {
        let height = self
            .get(row, col)
            .context("Can't find scenic score for non-existent tree")?;
        let distance_up = viewing_distance(self.up(row, col), height);
        let distance_down = viewing_distance(self.down(row, col), height);
        let distance_left = viewing_distance(self.left(row, col), height);
        let distance_right = viewing_distance(self.right(row, col), height);

        Ok(distance_up * distance_down * distance_left * distance_right)
    }

    fn highest_scenic_score(&self) -> Result<usize> {
        let mut max = 0;
        for idx in 0..self.grid.len() {
            let (row, col) = self.coords(idx);
            let score = self.scenic_score(row, col)?;
            max = max.max(score);
        }
        Ok(max)
    }
}

fn viewing_distance<I: Iterator<Item = u8>>(it: I, height: u8) -> usize {
    let mut dist = 0usize;
    for t in it {
        dist += 1;
        if t >= height {
            break;
        }
    }
    dist
}

impl FromStr for Trees {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .find("\n")
            .context("No newline found in first line to measure width")?;
        let grid: Vec<u8> = s
            .bytes()
            .filter(|b| b.is_ascii_digit())
            .map(|b| b - b'0')
            .collect();

        Ok(Self { grid, width })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    const EXAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_parse_input() -> Result<()> {
        let result = EXAMPLE.parse::<Trees>()?;
        let grid: Vec<u8> = EXAMPLE
            .bytes()
            .filter(|b| b.is_ascii_digit())
            .map(|b| b - b'0')
            .collect();
        let expected = Trees { grid, width: 5 };
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_solve_part_1() -> Result<()> {
        let result = EXAMPLE.parse::<Trees>()?.count_visible_trees();
        assert_eq!(result, 21usize);
        Ok(())
    }

    #[test]
    fn test_scenic_score() -> Result<()> {
        let trees = EXAMPLE.parse::<Trees>()?;
        let result = trees.scenic_score(1, 2)?;
        assert_eq!(result, 4);

        let result2 = trees.scenic_score(3, 2)?;
        assert_eq!(result2, 8);
        Ok(())
    }

    #[test]
    fn test_highest_scenic_score() -> Result<()> {
        let trees = EXAMPLE.parse::<Trees>()?;
        let result = trees.highest_scenic_score()?;
        assert_eq!(result, 8);
        Ok(())
    }
}
