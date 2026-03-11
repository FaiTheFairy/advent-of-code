use std::{fs, str::FromStr};

use anyhow::{Context, Result, anyhow, ensure};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt").context("failed to read input.txt")?;
    let grid = input.parse::<Grid>()?;

    let part_1 = solve_part_1(&grid);
    let part_2 = solve_part_2(&grid);

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");

    Ok(())
}

fn solve_part_1(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    (0..100).map(|_| grid.step()).sum()
}

fn solve_part_2(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    let target = grid.len();

    for step in 1.. {
        if grid.step() == target {
            return step;
        }
    }

    unreachable!("a synchronized flash should eventually occur")
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Energy>,
}

impl Grid {
    fn len(&self) -> usize {
        self.cells.len()
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    #[allow(unused)]
    fn get(&self, row: usize, col: usize) -> Option<Energy> {
        let index = self.index(row, col);
        self.cells.get(index).copied()
    }

    #[allow(unused)]
    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Energy> {
        let index = self.index(row, col);
        self.cells.get_mut(index)
    }

    fn neighbors(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
        const DIRS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        DIRS.into_iter().filter_map(move |(dr, dc)| {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if new_row < 0 || new_col < 0 {
                return None;
            }

            let new_row = new_row as usize;
            let new_col = new_col as usize;

            (new_row < self.height && new_col < self.width).then_some((new_row, new_col))
        })
    }

    /// Steps all energy levels and returns the number of flashes caused.
    fn step(&mut self) -> usize {
        let mut flashed = vec![false; self.len()];
        let mut pending = Vec::with_capacity(self.len());

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.index(row, col);
                self.cells[index].increase();
                if self.cells[index].is_ready_to_flash() {
                    pending.push((row, col));
                }
            }
        }

        while let Some((row, col)) = pending.pop() {
            let index = self.index(row, col);

            if flashed[index] {
                continue;
            }

            if !self.cells[index].is_ready_to_flash() {
                continue;
            }

            flashed[index] = true;

            let neighbors: Vec<(usize, usize)> = self.neighbors(row, col).collect();
            for (n_row, n_col) in neighbors {
                let neighbor_index = self.index(n_row, n_col);

                if flashed[neighbor_index] {
                    continue;
                }

                self.cells[neighbor_index].increase();

                if self.cells[neighbor_index].is_ready_to_flash() {
                    pending.push((n_row, n_col));
                }
            }
        }

        let mut flashes = 0;

        for (cell, did_flash) in self.cells.iter_mut().zip(flashed.into_iter()) {
            if did_flash {
                cell.reset();
                flashes += 1;
            }
        }

        flashes
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let lines: Vec<&str> = s.lines().filter(|line| !line.trim().is_empty()).collect();

        ensure!(!lines.is_empty(), "input is empty");

        let width = lines[0].len();
        ensure!(width > 0, "input contains an empty row");

        let height = lines.len();
        let mut cells = Vec::with_capacity(width * height);

        for (row_index, line) in lines.iter().enumerate() {
            ensure!(
                line.len() == width,
                "row {row_index} has length {}, expected {width}",
                line.len()
            );

            for c in line.chars() {
                let digit = c
                    .to_digit(10)
                    .ok_or_else(|| anyhow!("invalid digit {c:?} in input"))?;
                cells.push(Energy::try_new(digit as u8)?);
            }
        }

        Ok(Self {
            width,
            height,
            cells,
        })
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let energy = self.cells[self.index(row, col)].value();
                write!(f, "{energy}")?;
            }

            if row + 1 != self.height {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Energy(u8);

impl Energy {
    fn try_new(value: u8) -> Result<Self> {
        ensure!(value <= 9, "energy must be in 0..=9, got {value}");
        Ok(Self(value))
    }

    fn increase(&mut self) {
        self.0 += 1;
    }

    fn is_ready_to_flash(self) -> bool {
        self.0 > 9
    }

    fn reset(&mut self) {
        self.0 = 0;
    }

    fn value(self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL: &str = "\
11111
19991
19191
19991
11111";

    const LARGE: &str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn parses_input() {
        let grid = "123\n456".parse::<Grid>().unwrap();

        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 2);
        assert_eq!(grid.get(0, 0), Some(Energy(1)));
        assert_eq!(grid.get(1, 2), Some(Energy(6)));
        assert_eq!(grid.get(2, 0), None);
    }

    #[test]
    fn rejects_ragged_input() {
        let error = "12\n345".parse::<Grid>().unwrap_err().to_string();
        assert!(error.contains("expected 2"));
    }

    #[test]
    fn neighbors_of_corner_are_correct() {
        let grid = "12\n34".parse::<Grid>().unwrap();
        let neighbors: Vec<(usize, usize)> = grid.neighbors(0, 0).collect();

        assert_eq!(neighbors, vec![(0, 1), (1, 0), (1, 1)]);
    }

    #[test]
    fn small_example_step_1() {
        let mut grid = SMALL.parse::<Grid>().unwrap();

        let flashes = grid.step();

        assert_eq!(flashes, 9);
        assert_eq!(
            grid.to_string(),
            "\
34543
40004
50005
40004
34543"
        );
    }

    #[test]
    fn small_example_step_2() {
        let mut grid = SMALL.parse::<Grid>().unwrap();

        grid.step();
        let flashes = grid.step();

        assert_eq!(flashes, 0);
        assert_eq!(
            grid.to_string(),
            "\
45654
51115
61116
51115
45654"
        );
    }

    #[test]
    fn large_example_step_1() {
        let mut grid = LARGE.parse::<Grid>().unwrap();

        let flashes = grid.step();

        assert_eq!(flashes, 0);
        assert_eq!(
            grid.to_string(),
            "\
6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637"
        );
    }

    #[test]
    fn large_example_step_2() {
        let mut grid = LARGE.parse::<Grid>().unwrap();

        grid.step();
        let flashes = grid.step();

        assert_eq!(flashes, 35);
        assert_eq!(
            grid.to_string(),
            "\
8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848"
        );
    }

    #[test]
    fn part_1_example_after_10_steps() {
        let mut grid = LARGE.parse::<Grid>().unwrap();
        let flashes: usize = (0..10).map(|_| grid.step()).sum();

        assert_eq!(flashes, 204);
        assert_eq!(
            grid.to_string(),
            "\
0481112976
0031112009
0041112504
0081111406
0099111306
0093511233
0442361130
5532252350
0532250600
0032240000"
        );
    }

    #[test]
    fn part_1_example_after_100_steps() {
        let grid = LARGE.parse::<Grid>().unwrap();
        assert_eq!(solve_part_1(&grid), 1656);
    }

    #[test]
    fn part_2_example() {
        let grid = LARGE.parse::<Grid>().unwrap();
        assert_eq!(solve_part_2(&grid), 195);
    }
}
