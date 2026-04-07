use anyhow::{Context, Result, bail};
use std::{fs, str::FromStr};

fn main() -> Result<()> {
    let instructions: Instructions = fs::read_to_string("input.txt")?.parse()?;

    let sol1 = instructions.final_floor(Floor::GROUND);
    println!("Part 1: {}", sol1.value());

    let sol2 = instructions
        .first_position_reaching(Floor::BASEMENT, Floor::GROUND)
        .context("instructions never reach the basement")?;
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Floor(isize);

impl Floor {
    const GROUND: Self = Self(0);
    const BASEMENT: Self = Self(-1);

    fn value(self) -> isize {
        self.0
    }

    fn moved(self, direction: Direction) -> Self {
        Self(self.0 + direction.delta())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Instructions(Vec<Direction>);

impl Instructions {
    fn iter(&self) -> impl Iterator<Item = Direction> {
        self.0.iter().copied()
    }

    fn final_floor(&self, start: Floor) -> Floor {
        self.iter().fold(start, Floor::moved)
    }

    fn first_position_reaching(&self, target: Floor, start: Floor) -> Option<usize> {
        self.iter()
            .scan(start, |floor: &mut Floor, direction: Direction| {
                *floor = floor.moved(direction);
                Some(*floor)
            })
            .position(|floor| floor == target)
            .map(|idx| idx + 1)
    }
}

impl FromStr for Instructions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let instructions: Vec<Direction> = s
            .trim()
            .chars()
            .map(Direction::try_from)
            .collect::<Result<_, _>>()?;

        Ok(Self(instructions))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
}

impl Direction {
    fn delta(self) -> isize {
        match self {
            Direction::Up => 1,
            Direction::Down => -1,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::Up),
            ')' => Ok(Self::Down),
            _ => bail!("unknown direction: {value}"),
        }
    }
}
