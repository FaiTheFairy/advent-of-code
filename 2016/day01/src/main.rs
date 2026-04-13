use std::{collections::HashSet, fs, str::FromStr};

use anyhow::{Context, Result, bail};

fn main() -> Result<()> {
    let instructions: Instructions = fs::read_to_string("input.txt")?.parse()?;

    let mut me: Me = Me::default();
    me.apply_all_instructions(&instructions);
    let sol1: usize = me.distance();
    println!("Part 1: {sol1}");

    let mut me: Me = Me::default();
    let first: Coordinates = me
        .first_location_visited_twice(&instructions)
        .context("no location visited twice")?;
    let sol2: usize = first.distance();
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Me {
    facing: Direction,
    coordinates: Coordinates,
}

impl Me {
    fn distance(&self) -> usize {
        self.coordinates.distance()
    }

    fn first_location_visited_twice(&mut self, instructions: &Instructions) -> Option<Coordinates> {
        let mut visited: HashSet<Coordinates> = HashSet::new();
        visited.insert(self.coordinates);

        for instruction in instructions.iter() {
            let (turn, steps): (Turn, isize) = instruction.parts();
            self.facing = self.facing.turn(turn);

            for _ in 0..steps {
                self.coordinates.step(self.facing);
                if !visited.insert(self.coordinates) {
                    return Some(self.coordinates);
                }
            }
        }

        None
    }

    fn apply_all_instructions(&mut self, instructions: &Instructions) {
        for instruction in instructions.iter() {
            self.apply_instruction(instruction);
        }
    }

    fn apply_instruction(&mut self, instruction: Instruction) {
        let (turn, steps): (Turn, isize) = instruction.parts();
        self.facing = self.facing.turn(turn);
        self.coordinates.walk(steps, self.facing);
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Coordinates {
    north: isize,
    east: isize,
}

impl Coordinates {
    fn distance(&self) -> usize {
        self.north.unsigned_abs() + self.east.unsigned_abs()
    }

    fn walk(&mut self, n: isize, direction: Direction) {
        let (dnorth, deast): (isize, isize) = match direction {
            Direction::North => (n, 0),
            Direction::East => (0, n),
            Direction::South => (-n, 0),
            Direction::West => (0, -n),
        };
        self.north += dnorth;
        self.east += deast;
    }

    fn step(&mut self, direction: Direction) {
        self.walk(1, direction);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Instructions(Vec<Instruction>);

impl Instructions {
    fn iter(&self) -> impl Iterator<Item = Instruction> + '_ {
        self.0.iter().copied()
    }
}

impl FromStr for Instructions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions: Vec<Instruction> = s
            .split(',')
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self(instructions))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Instruction {
    Left(isize),
    Right(isize),
}

impl Instruction {
    fn parts(self) -> (Turn, isize) {
        match self {
            Self::Left(n) => (Turn::Left, n),
            Self::Right(n) => (Turn::Right, n),
        }
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, n): (&str, &str) = s.split_at(1);
        let n: isize = n.parse()?;
        match direction {
            "R" => Ok(Self::Right(n)),
            "L" => Ok(Self::Left(n)),
            _ => bail!("unknown direction: {s}"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Turn {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
enum Direction {
    #[default]
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(self, turn: Turn) -> Self {
        match (self, turn) {
            (Direction::North, Turn::Right) => Direction::East,
            (Direction::East, Turn::Right) => Direction::South,
            (Direction::South, Turn::Right) => Direction::West,
            (Direction::West, Turn::Right) => Direction::North,
            (Direction::North, Turn::Left) => Direction::West,
            (Direction::West, Turn::Left) => Direction::South,
            (Direction::South, Turn::Left) => Direction::East,
            (Direction::East, Turn::Left) => Direction::North,
        }
    }
}
