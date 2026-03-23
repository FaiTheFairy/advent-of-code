#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{Result, anyhow, bail};

pub fn solve_part_1(input: &str) -> Result<usize> {
    let wires: Wires = input.parse()?;
    let wire1: HashSet<Coord> = wires.0[0].trace_path().keys().copied().collect();
    let wire2: HashSet<Coord> = wires.0[1].trace_path().keys().copied().collect();
    let sol = wire1
        .intersection(&wire2)
        .copied()
        .map(|coord| manhattan_distance((0, 0), coord))
        .min()
        .ok_or(anyhow!("no intersection found"))?;

    Ok(sol)
}

fn manhattan_distance(origin: (isize, isize), intersections: (isize, isize)) -> usize {
    origin.0.abs_diff(intersections.0) + origin.1.abs_diff(intersections.1)
}

pub fn solve_part_2(input: &str) -> Result<usize> {
    let wires: Wires = input.parse()?;
    let wire1 = wires.0[0].trace_path();
    let wire2 = wires.0[1].trace_path();

    let sol = wire1
        .iter()
        .filter_map(|(coord, steps1)| wire2.get(coord).map(|steps2| steps1 + steps2))
        .min()
        .ok_or(anyhow!("no intersection found"))?;

    Ok(sol)
}

type NumberOfWires = usize;
type Coord = (isize, isize);
type Steps = usize;

impl WirePath {
    fn trace_path(&self) -> HashMap<Coord, Steps> {
        let mut visited = HashMap::with_capacity(self.0.len());
        let mut x = 0;
        let mut y = 0;
        let mut steps = 0;

        for dir in self.0.iter() {
            let (dx, dy, len) = match dir {
                Direction::Up(n) => (0, 1, *n),
                Direction::Down(n) => (0, -1, *n),
                Direction::Right(n) => (1, 0, *n),
                Direction::Left(n) => (-1, 0, *n),
            };

            for _ in 0..len {
                x += dx;
                y += dy;
                steps += 1;
                visited.entry((x, y)).or_insert(steps);
            }
        }

        visited
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Wires(Vec<WirePath>);

impl FromStr for Wires {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let wires = s
            .lines()
            .map(str::trim)
            .map(str::parse::<WirePath>)
            .collect::<Result<_, _>>()?;

        Ok(Self(wires))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct WirePath(Vec<Direction>);

impl FromStr for WirePath {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let path: Vec<Direction> = s
            .trim()
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self(path))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up(usize),
    Down(usize),
    Right(usize),
    Left(usize),
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (direction, value) = s.split_at(1);
        let value = value.parse()?;
        match direction {
            "U" => Ok(Self::Up(value)),
            "D" => Ok(Self::Down(value)),
            "R" => Ok(Self::Right(value)),
            "L" => Ok(Self::Left(value)),
            _ => bail!("unknown direction: {direction}"),
        }
    }
}
