#![allow(unused)]

use std::{collections::HashSet, fs, str::FromStr};

use anyhow::{Context, Result, bail};

fn main() -> Result<()> {
    let input: Input = fs::read_to_string("input.txt")?.parse()?;

    let sol1 = {
        let mut rope = Rope::new(2);
        rope.apply_directions(&input.0);
        rope.tail_positions.len()
    };
    println!("Part 1: {sol1}");

    let sol2 = {
        let mut rope = Rope::new(10);
        rope.apply_directions(&input.0);
        rope.tail_positions.len()
    };
    println!("Part 2: {sol2}");

    Ok(())
}

struct Input(Vec<Direction>);

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut inner = Vec::with_capacity(s.lines().count());
        for line in s.lines() {
            let (direction, frequency) =
                line.trim().split_once(' ').context("missing ' ' (space)")?;
            let direction = direction.parse()?;
            let frequency = frequency.parse()?;

            for _ in 0..frequency {
                inner.push(direction);
            }
        }

        Ok(Self(inner))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Rope {
    head: Position,
    body: Vec<Position>,
    tail: Position,
    tail_positions: HashSet<Position>,
}

impl Rope {
    fn new(total_len: usize) -> Self {
        assert!(total_len >= 2);

        Self {
            head: Position::default(),
            body: vec![Position::default(); total_len - 2],
            tail: Position::default(),
            tail_positions: HashSet::from([Position::default()]),
        }
    }

    fn len(&self) -> usize {
        self.body.len() + 2
    }

    fn apply_directions(&mut self, directions: &[Direction]) {
        for direction in directions {
            self.move_head(*direction);
        }
    }

    fn move_head(&mut self, direction: Direction) {
        self.head = self.head.moved(direction);

        for i in 1..self.len() {
            let leader = self.get(i - 1);
            let follower = &mut self.get_mut(i);

            follower.follow(leader);
        }

        self.tail_positions.insert(self.tail);
    }

    fn get(&self, idx: usize) -> Position {
        if idx == 0 {
            self.head
        } else if idx == self.len() - 1 {
            self.tail
        } else {
            self.body[idx - 1]
        }
    }

    fn get_mut(&mut self, idx: usize) -> &mut Position {
        if idx == 0 {
            &mut self.head
        } else if idx == self.len() - 1 {
            &mut self.tail
        } else {
            &mut self.body[idx - 1]
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn moved(self, direction: impl Into<Delta>) -> Self {
        let Delta { dx, dy } = direction.into();
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    fn follow(&mut self, leader: Self) {
        let dx = leader.x - self.x;
        let dy = leader.y - self.y;

        if dx.abs() <= 1 && dy.abs() <= 1 {
            return;
        }

        self.x += dx.signum();
        self.y += dy.signum();
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.trim() {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            other => bail!("unknown direction: {other}"),
        }
    }
}

impl From<Direction> for Delta {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Delta { dx: 0, dy: 1 },
            Direction::Down => Delta { dx: 0, dy: -1 },
            Direction::Left => Delta { dx: -1, dy: 0 },
            Direction::Right => Delta { dx: 1, dy: 0 },
        }
    }
}

struct Delta {
    dx: i32,
    dy: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const EXAMPLE_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part_2() {
        let input_1: Input = EXAMPLE_1.parse().unwrap();
        let rope = Rope::new(10);
        let result = {
            let mut rope = rope.clone();
            rope.apply_directions(&input_1.0);
            rope.tail_positions.len()
        };

        let input_2: Input = EXAMPLE_2.parse().unwrap();
        let result = {
            let mut rope = rope.clone();
            rope.apply_directions(&input_2.0);
            rope.tail_positions.len()
        };
        assert_eq!(result, 36);
    }

    #[test]
    fn part_1() {
        let input: Input = EXAMPLE_1.parse().unwrap();
        let mut rope = Rope::new(2);
        rope.apply_directions(&input.0);
        let result = rope.tail_positions.len();
        assert_eq!(result, 13);
    }
}
