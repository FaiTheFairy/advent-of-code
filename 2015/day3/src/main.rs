use std::{collections::HashSet, fs, str::FromStr};

use anyhow::{Result, bail};

fn main() -> Result<()> {
    let route: Route = fs::read_to_string("input.txt")?.parse()?;

    let sol1 = route.count_unique_houses();
    println!("Part 1: {sol1}");

    let (santa, robo_santa) = route.split_work();

    let visited_by_santa = santa.visited_houses();
    let visited_by_robo = robo_santa.visited_houses();
    let all_visited = visited_by_santa.union(&visited_by_robo);

    let sol2 = all_visited.count();
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Route(Vec<Direction>);

impl Route {
    fn visited_houses(&self) -> HashSet<House> {
        let mut visited = HashSet::new();
        let mut current = House::default();

        visited.insert(current);

        for direction in self.iter() {
            current = current.moved(direction);
            visited.insert(current);
        }

        visited
    }

    fn count_unique_houses(&self) -> usize {
        self.visited_houses().len()
    }

    fn split_work(&self) -> (Self, Self) {
        let even_instructions = self.iter().step_by(2).collect();
        let odd_instructions = self.iter().skip(1).step_by(2).collect();

        (Self(even_instructions), Self(odd_instructions))
    }

    fn iter(&self) -> impl Iterator<Item = Direction> {
        self.0.iter().copied()
    }
}

impl FromStr for Route {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let instructions = s
            .trim()
            .chars()
            .map(Direction::try_from)
            .collect::<Result<_, _>>()?;

        Ok(Self(instructions))
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct House {
    x: isize,
    y: isize,
}

impl House {
    fn moved(self, direction: Direction) -> Self {
        let (dx, dy) = direction.delta();

        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn delta(self) -> (isize, isize) {
        match self {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::North),
            '>' => Ok(Self::East),
            'v' => Ok(Self::South),
            '<' => Ok(Self::West),
            _ => bail!("unknown direction: {value}"),
        }
    }
}
