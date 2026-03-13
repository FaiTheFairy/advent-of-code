#![allow(unused)]
use anyhow::Result;
use grid::*;

use std::{fs, str::FromStr};

fn main() -> Result<()> {
    let tree_map = fs::read_to_string("input.txt")?.parse::<TreeMap>()?;
    let sol1 = tree_map.solve_part_1();
    println!("Part 1. traversing the map, {sol1} trees were encountered");

    let sol2 = tree_map.solve_part_2();
    println!(
        "Part 2. traversing the map for the indicated slopes yields a product of tree counts = {sol2}"
    );

    Ok(())
}

type IsTree = bool;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TreeMap(Grid<IsTree>);

impl TreeMap {
    fn solve_part_2(&self) -> usize {
        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let mut product = 1usize;
        for (right, down) in slopes {
            product *= self.count_trees_for_slope(right, down);
        }

        product
    }

    fn solve_part_1(&self) -> usize {
        self.count_trees_for_slope(3, 1)
    }

    fn count_trees_for_slope(&self, right: usize, down: usize) -> usize {
        let (mut row, mut col) = (0, 0);
        let mut on_last_row = false;
        let mut count = 0usize;
        let mut tree_map = self.clone();
        while !on_last_row {
            if col + right >= tree_map.0.cols() {
                tree_map = tree_map.repeated_right_once();
            }
            (row, col) = (row + down, col + right);
            if let Some(&is_tree) = tree_map.0.get(row, col)
                && is_tree
            {
                count += 1;
            }
            if row >= tree_map.0.rows() {
                on_last_row = true;
            }
        }
        count
    }

    fn repeated_right(&mut self, n: usize) -> Self {
        let mut new_grid = self;
        for _ in 0..n {
            *new_grid = new_grid.repeated_right_once()
        }
        new_grid.clone()
    }

    fn repeated_right_once(&self) -> Self {
        let mut new_grid: Grid<IsTree> = Grid::with_capacity(self.0.rows(), self.0.cols() * 2);
        for row in self.0.iter_rows() {
            let mut row: Vec<IsTree> = row.copied().collect();
            row.extend_from_within(..);
            new_grid.push_row(row);
        }
        TreeMap(new_grid)
    }
}

impl FromStr for TreeMap {
    type Err = anyhow::Error;

    /// Parses grid of '.' (empty) and '#' (tree) to a `TreeMap(Grid<IsTree>)`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.lines().count();
        let cols = s.lines().next().unwrap().trim().chars().count();
        let mut grid = Grid::with_capacity(rows, cols);
        for line in s.lines() {
            let row: Vec<IsTree> = line.chars().map(|l| matches!(l, '#')).collect();
            grid.push_row(row);
        }

        Ok(Self(grid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    const EXAMPLE_TRUNC: &str = "..##.......
#...#...#..
.#....#..#.";

    const EXAMPLE_TRUNC_REPEATED_RIGHT_TWICE: &str = "..##.........##.........##.........##.......
#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#.
";

    #[test]
    fn test_solve_part_2() {
        let result = EXAMPLE.parse::<TreeMap>().unwrap().solve_part_2();
        assert_eq!(result, 336);
    }

    #[test]
    fn test_solve_part_1() {
        let result = EXAMPLE.parse::<TreeMap>().unwrap().solve_part_1();
        assert_eq!(result, 7);
    }

    #[test]
    fn test_repeat_right() {
        let mut result = EXAMPLE_TRUNC.parse::<TreeMap>().unwrap().repeated_right(2);
        let mut expected = EXAMPLE_TRUNC_REPEATED_RIGHT_TWICE
            .parse::<TreeMap>()
            .unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_tree_map() {
        let result = EXAMPLE_TRUNC.parse::<TreeMap>().unwrap();
        let expected = TreeMap(grid![
            [false, false, true, true, false, false, false, false, false, false, false]
            [true, false, false, false, true, false, false, false, true, false, false]
            [false, true, false, false, false, false, true, false, false, true, false]
        ]);

        assert_eq!(result, expected);
    }
}
