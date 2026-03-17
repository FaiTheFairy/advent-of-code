#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{Result, anyhow, ensure};

pub fn solve_part_1(input: &str) -> Result<usize> {
    let map: Map = input.parse()?;
    let sol = map.antinodes_part_1().len();
    Ok(sol)
}
pub fn solve_part_2(input: &str) -> Result<usize> {
    let map: Map = input.parse()?;
    let sol = map.antinodes_part_2().len();
    Ok(sol)
}

type Frequency = char;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Map {
    width: usize,
    height: usize,
    antennas: HashMap<Frequency, Vec<Point>>,
}

impl Map {
    fn antinodes_part_2(&self) -> HashSet<Point> {
        let mut antinodes = HashSet::new();

        for points in self.antennas.values() {
            for i in 0..points.len() {
                for j in i + 1..points.len() {
                    let a = points[i];
                    let b = points[j];

                    let dr = b.row - a.row;
                    let dc = b.col - a.col;

                    let g = gcd(dr.abs(), dc.abs());
                    let step_row = dr / g;
                    let step_col = dc / g;

                    // walk backward
                    let mut p = a;
                    while self.contains(p) {
                        antinodes.insert(p);
                        p = Point {
                            row: p.row - step_row,
                            col: p.col - step_col,
                        };
                    }

                    // walk forward
                    let mut p = a;
                    while self.contains(p) {
                        antinodes.insert(p);
                        p = Point {
                            row: p.row + step_row,
                            col: p.col + step_col,
                        };
                    }
                }
            }
        }

        antinodes
    }

    fn contains(&self, point: Point) -> bool {
        (0..self.height as isize).contains(&point.row)
            && (0..self.width as isize).contains(&point.col)
    }

    fn antinodes_part_1(&self) -> HashSet<Point> {
        let mut antinodes = HashSet::new();

        for points in self.antennas.values() {
            for i in 0..points.len() {
                for j in i + 1..points.len() {
                    let a = points[i];
                    let b = points[j];

                    let dr = b.row - a.row;
                    let dc = b.col - a.col;

                    let p1 = Point {
                        row: a.row - dr,
                        col: a.col - dc,
                    };

                    let p2 = Point {
                        row: b.row + dr,
                        col: b.col + dc,
                    };

                    if self.contains(p1) {
                        antinodes.insert(p1);
                    }

                    if self.contains(p2) {
                        antinodes.insert(p2);
                    }
                }
            }
        }

        antinodes
    }
}

fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines: Vec<&str> = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect();

        ensure!(!lines.is_empty(), "input is empty");

        let height = lines.len();
        let width = lines[0].len();
        ensure!(width > 0, "input has empty first row");

        for line in &lines {
            ensure!(
                line.len() == width,
                "grid is not rectangular: expected width {width}, got {}",
                line.len()
            );
        }

        let mut antennas: HashMap<Frequency, Vec<Point>> = HashMap::new();

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch != '.' {
                    antennas.entry(ch).or_default().push(Point {
                        row: row as isize,
                        col: col as isize,
                    });
                }
            }
        }

        Ok(Self {
            width,
            height,
            antennas,
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    row: isize,
    col: isize,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_parse_map() -> Result<()> {
        let map: Map = EXAMPLE.parse()?;

        assert_eq!(map.width, 12);
        assert_eq!(map.height, 12);
        assert_eq!(map.antennas.get(&'0').unwrap().len(), 4);
        assert_eq!(map.antennas.get(&'A').unwrap().len(), 3);

        Ok(())
    }

    #[test]
    fn test_solve_part_1_example() -> Result<()> {
        assert_eq!(solve_part_1(EXAMPLE)?, 14);
        Ok(())
    }

    #[test]
    fn test_contains() -> Result<()> {
        let map: Map = EXAMPLE.parse()?;

        assert!(map.contains(Point { row: 0, col: 0 }));
        assert!(map.contains(Point { row: 11, col: 11 }));
        assert!(!map.contains(Point { row: 50, col: 0 }));
        assert!(!map.contains(Point { row: 0, col: 12 }));

        Ok(())
    }
}
