use std::{fs, str::FromStr};

use anyhow::{Result, bail};

fn main() -> Result<()> {
    let input: Input = fs::read_to_string("input.txt")?.parse()?;

    let sol1 = input.solve_part_1();
    println!("Part 1: {sol1}");

    let sol2 = input.solve_part_2();
    println!("Part 2: {sol2}");

    Ok(())
}

struct Input(Vec<Direction>);

impl Input {
    fn solve_part_1(&self) -> usize {
        Hexagon::default()
            .step_multiple(&self.0)
            .distance_from_origin()
    }

    fn solve_part_2(&self) -> usize {
        let mut current_hex = Hexagon::default();
        let mut max_dist = 0;

        for dir in &self.0 {
            current_hex = current_hex.step(*dir);
            max_dist = max_dist.max(current_hex.distance_from_origin());
        }

        max_dist
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let inner = s
            .trim()
            .split(',')
            .map(str::parse::<Direction>)
            .collect::<Result<_, _>>()?;
        Ok(Self(inner))
    }
}

/// a hexagon in a hex grid, with (N, Ne, Se) corresponding to (x, y, z)
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Hexagon {
    q: isize,
    r: isize,
    s: isize,
}

impl Hexagon {
    fn distance_from_origin(self) -> usize {
        self.distance(Hexagon::default())
    }

    fn distance(self, other: Hexagon) -> usize {
        (self.q.abs_diff(other.q) + self.r.abs_diff(other.r) + self.s.abs_diff(other.s)) / 2
    }

    fn step_multiple(self, dirs: &[Direction]) -> Self {
        dirs.iter().fold(self, |acc, dir| acc.step(*dir))
        // same as
        // ```
        // let mut hex = self;
        // for dir in dirs {
        //     hex = hex.step(dir);
        // }
        // hex
        // ```
    }

    fn step(self, dir: Direction) -> Self {
        let (dq, dr, ds) = dir.delta();
        let hexagon = Self {
            q: self.q + dq,
            r: self.r + dr,
            s: self.s + ds,
        };
        debug_assert!(hexagon.q + hexagon.r + hexagon.s == 0);
        hexagon
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "n" => Ok(Direction::North),
            "ne" => Ok(Direction::NorthEast),
            "se" => Ok(Direction::SouthEast),
            "s" => Ok(Direction::South),
            "sw" => Ok(Direction::SouthWest),
            "nw" => Ok(Direction::NorthWest),
            _ => bail!("unknown direction: {s}"),
        }
    }
}

impl Direction {
    #[rustfmt::skip]
    /// Deltas (dq, dr, ds) for moving in the 6 directions.
    fn delta(self) -> (isize, isize, isize) {
        match self {
            Direction::North     => ( 0, -1,  1),
            Direction::NorthEast => ( 1, -1,  0),
            Direction::SouthEast => ( 1,  0, -1),
            Direction::South     => ( 0,  1, -1),
            Direction::SouthWest => (-1,  1,  0),
            Direction::NorthWest => (-1,  0,  1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let result = "ne,ne,ne".parse::<Input>().unwrap().solve_part_1();
        assert_eq!(result, 3);

        let result = "ne,ne,sw,sw ".parse::<Input>().unwrap().solve_part_1();
        assert_eq!(result, 0);

        let result = "ne,ne,s,s".parse::<Input>().unwrap().solve_part_1();
        assert_eq!(result, 2);

        let result = "se,sw,se,sw,sw".parse::<Input>().unwrap().solve_part_1();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_distance() {
        let hex = Hexagon::default();
        assert_eq!(
            hex.step_multiple(&[Direction::NorthEast; 3])
                .distance_from_origin(),
            3
        );
        assert_eq!(
            hex.step_multiple(&[Direction::NorthEast; 2])
                .step_multiple(&[Direction::SouthWest; 2])
                .distance_from_origin(),
            0
        );
        assert_eq!(
            hex.step_multiple(&[Direction::NorthEast; 2])
                .step_multiple(&[Direction::South; 2])
                .distance_from_origin(),
            2
        );
        assert_eq!(
            hex.step_multiple(&[Direction::SouthEast])
                .step_multiple(&[Direction::SouthWest])
                .step_multiple(&[Direction::SouthEast])
                .step_multiple(&[Direction::SouthWest; 2])
                .distance_from_origin(),
            3
        );
    }

    #[test]
    fn test_step() {
        let hex = Hexagon::default().step_multiple(&[Direction::NorthEast; 3]);
        assert_eq!(hex, Hexagon { q: 3, r: -3, s: 0 });

        let hex = Hexagon::default()
            .step_multiple(&[Direction::NorthEast; 2])
            .step_multiple(&[Direction::SouthWest; 2]);
        assert_eq!(hex, Hexagon::default());
    }
}
