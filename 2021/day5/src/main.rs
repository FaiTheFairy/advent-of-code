use std::{collections::HashMap, fs, str::FromStr};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let vent_field = input.parse::<VentField>()?;
    let sol1 = vent_field.overlap_count_straight();
    println!(
        "Part 1. number of points where at least two horizontal or vertical lines overlap = {sol1}"
    );

    let sol2 = vent_field.overlap_count();
    println!("Part 2. number of points where at least two lines overlap = {sol2}");

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct VentField {
    lines: Vec<VentLine>,
}

impl VentField {
    fn straight_lines(&self) -> impl Iterator<Item = &VentLine> {
        self.lines
            .iter()
            .filter(|&vl| vl.is_horizontal_or_vertical())
    }

    fn overlap_count_straight(&self) -> usize {
        let mut map = HashMap::new();
        for line in self.straight_lines() {
            for point in line.points() {
                map.entry(point).and_modify(|e| *e += 1).or_insert(1);
            }
        }

        map.iter().filter(|&e| *e.1 >= 2).count()
    }

    fn overlap_count(&self) -> usize {
        let mut map = HashMap::new();
        for line in &self.lines {
            for point in line.points() {
                map.entry(point).and_modify(|e| *e += 1).or_insert(1);
            }
        }

        map.iter().filter(|&e| *e.1 >= 2).count()
    }
}

impl FromStr for VentField {
    type Err = anyhow::Error;

    /// Parses
    /// ```text
    /// 0,9 -> 5,9
    /// 8,0 -> 0,8
    /// ```
    /// to `VentsMap`
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(|l| l.parse::<VentLine>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { lines })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VentLine {
    start: Coord,
    end: Coord,
}

impl VentLine {
    fn is_horizontal(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn is_vertical(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_horizontal_or_vertical(&self) -> bool {
        self.is_vertical() || self.is_horizontal()
    }

    fn points(&self) -> impl Iterator<Item = Coord> {
        let Coord(x1, y1) = self.start;
        let Coord(x2, y2) = self.end;

        let dx = (x2 as isize - x1 as isize).signum();
        let dy = (y2 as isize - y1 as isize).signum();

        let steps = usize::max(
            (x2 as isize - x1 as isize).unsigned_abs(),
            (y2 as isize - y1 as isize).unsigned_abs(),
        );

        (0..=steps).map(move |i| {
            Coord(
                (x1 as isize + dx * i as isize) as usize,
                (y1 as isize + dy * i as isize) as usize,
            )
        })
    }
}

impl FromStr for VentLine {
    type Err = anyhow::Error;

    /// Parses "0,9 -> 5,9" to `VentLine`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .trim()
            .split_once("->")
            .with_context(|| format!("invalid line: {s}"))?;
        let (start, end) = (start.parse::<Coord>()?, end.parse::<Coord>()?);
        Ok(VentLine { start, end })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord(usize, usize);

impl FromStr for Coord {
    type Err = anyhow::Error;

    /// Parses "0,9" to `Coord(0, 9)`
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (s1, s2) = s
            .trim()
            .split_once(',')
            .with_context(|| format!("invalid coordinates {s}"))?;
        Ok(Coord(s1.parse::<usize>()?, s2.parse::<usize>()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_overlap_count() {
        let result = EXAMPLE
            .parse::<VentField>()
            .unwrap()
            .overlap_count_straight();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_parse_coord() {
        let result = "0,9".parse::<Coord>().unwrap();
        let expected = Coord(0, 9);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_vent_line() {
        let result = "8,0 -> 0,8".parse::<VentLine>().unwrap();
        let expected = VentLine {
            start: Coord(8, 0),
            end: Coord(0, 8),
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_vent_map() {
        let result = "0,9 -> 5,9\n8,0 -> 0,8".parse::<VentField>().unwrap();
        let expected = VentField {
            lines: vec![
                VentLine {
                    start: Coord(0, 9),
                    end: Coord(5, 9),
                },
                VentLine {
                    start: Coord(8, 0),
                    end: Coord(0, 8),
                },
            ],
        };
        assert_eq!(result, expected);
    }
}
