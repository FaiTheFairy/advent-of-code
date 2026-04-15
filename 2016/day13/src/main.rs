#![allow(dead_code)]
use std::collections::{HashSet, VecDeque};

use anyhow::{Context, Result};

const FAVORITE_NUMBER: usize = 1352;

fn main() -> Result<()> {
    let sol1 = bfs_steps_to(Coordinate { x: 31, y: 39 }).context("no solution found for part 1")?;
    println!("Part 1: {sol1}");

    let sol2 = reachable_within(50);
    println!("Part 2: {sol2}");

    Ok(())
}

fn reachable_within(max_steps: usize) -> usize {
    let start = Coordinate { x: 1, y: 1 };

    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut queue: VecDeque<(Coordinate, usize)> = VecDeque::new();

    visited.insert(start);
    queue.push_back((start, 0));

    while let Some((current, steps)) = queue.pop_front() {
        if steps == max_steps {
            continue;
        }

        for neighbor in current.neighbors().into_iter().flatten() {
            if !neighbor.is_open() {
                continue;
            }

            if visited.insert(neighbor) {
                queue.push_back((neighbor, steps + 1));
            }
        }
    }

    visited.len()
}

fn bfs_steps_to(target: Coordinate) -> Option<usize> {
    let start = Coordinate { x: 1, y: 1 };

    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut queue: VecDeque<(Coordinate, usize)> = VecDeque::new();

    visited.insert(start);
    queue.push_back((start, 0));

    while let Some((current, steps)) = queue.pop_front() {
        if current == target {
            return Some(steps);
        }

        for neighbor in current.neighbors().into_iter().flatten() {
            if !neighbor.is_open() {
                continue;
            }

            if visited.insert(neighbor) {
                queue.push_back((neighbor, steps + 1));
            }
        }
    }

    None
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn cell(self) -> Cell {
        let value = self.polynomial() + FAVORITE_NUMBER;

        if value.count_ones().is_multiple_of(2) {
            Cell::Open
        } else {
            Cell::Wall
        }
    }

    fn polynomial(self) -> usize {
        let Self { x, y } = self;
        (x * x) + (3 * x) + (2 * x * y) + (y) + (y * y)
    }

    fn is_open(self) -> bool {
        self.cell().is_open()
    }

    fn neighbors(self) -> [Option<Coordinate>; 4] {
        let left = self.x.checked_sub(1).map(|x| Coordinate { x, y: self.y });
        let up = self.y.checked_sub(1).map(|y| Coordinate { x: self.x, y });
        let right = Some(Coordinate {
            x: self.x + 1,
            y: self.y,
        });
        let down = Some(Coordinate {
            x: self.x,
            y: self.y + 1,
        });

        [left, up, right, down]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Cell {
    Open,
    Wall,
}

impl Cell {
    /// Returns `true` if the cell is [`Open`].
    ///
    /// [`Open`]: Cell::Open
    #[must_use]
    fn is_open(&self) -> bool {
        matches!(self, Self::Open)
    }
}
