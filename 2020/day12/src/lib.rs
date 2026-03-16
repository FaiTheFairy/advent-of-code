use std::str::FromStr;

use anyhow::{Result, anyhow, bail};

pub fn solve_part_1(input: &str) -> Result<usize> {
    let instructions: Instructions = input.parse()?;
    let ship = Ship::default().apply_all(&instructions);
    Ok(ship.position.manhattan_distance())
}

pub fn solve_part_2(input: &str) -> Result<usize> {
    let instructions: Instructions = input.parse()?;
    let ship = WaypointShip::default().apply_all(&instructions);
    Ok(ship.position.manhattan_distance())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Instructions(Vec<Instruction>);

impl FromStr for Instructions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(instructions))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Instruction {
    action: Action,
    value: usize,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.trim();
        let (action, value) = s.split_at(1);

        let action = action
            .chars()
            .next()
            .ok_or_else(|| anyhow!("empty instruction"))?;
        let action: Action = action.try_into()?;
        let value = value.parse::<usize>()?;

        Ok(Self { action, value })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Action {
    Move(Direction),
    Turn(Turn),
    Forward,
}

impl TryFrom<char> for Action {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'N' => Ok(Self::Move(Direction::North)),
            'S' => Ok(Self::Move(Direction::South)),
            'E' => Ok(Self::Move(Direction::East)),
            'W' => Ok(Self::Move(Direction::West)),
            'L' => Ok(Self::Turn(Turn::Left)),
            'R' => Ok(Self::Turn(Turn::Right)),
            'F' => Ok(Self::Forward),
            _ => bail!("unknown action: {value}"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Turn {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turned(self, turn: Turn, degrees: usize) -> Self {
        let quarter_turns = (degrees / 90) % 4;
        let index = self.index();

        match turn {
            Turn::Right => Self::from_index(index + quarter_turns),
            Turn::Left => Self::from_index(index + 4 - quarter_turns),
        }
    }

    fn index(self) -> usize {
        match self {
            Self::North => 0,
            Self::East => 1,
            Self::South => 2,
            Self::West => 3,
        }
    }

    fn from_index(index: usize) -> Self {
        match index % 4 {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Position {
    east: isize,
    north: isize,
}

impl Position {
    fn manhattan_distance(self) -> usize {
        self.east.unsigned_abs() + self.north.unsigned_abs()
    }

    fn moved(self, direction: Direction, value: usize) -> Self {
        let value = value as isize;

        match direction {
            Direction::North => Self {
                north: self.north + value,
                ..self
            },
            Direction::South => Self {
                north: self.north - value,
                ..self
            },
            Direction::East => Self {
                east: self.east + value,
                ..self
            },
            Direction::West => Self {
                east: self.east - value,
                ..self
            },
        }
    }

    fn rotated(self, turn: Turn, degrees: usize) -> Self {
        match (turn, (degrees / 90) % 4) {
            (_, 0) => self,
            (Turn::Right, 1) | (Turn::Left, 3) => Self {
                east: self.north,
                north: -self.east,
            },
            (_, 2) => Self {
                east: -self.east,
                north: -self.north,
            },
            (Turn::Right, 3) | (Turn::Left, 1) => Self {
                east: -self.north,
                north: self.east,
            },
            _ => unreachable!(),
        }
    }

    fn scaled(self, factor: usize) -> Self {
        let factor = factor as isize;

        Self {
            east: self.east * factor,
            north: self.north * factor,
        }
    }
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            east: self.east + rhs.east,
            north: self.north + rhs.north,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Ship {
    position: Position,
    facing: Direction,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            position: Position::default(),
            facing: Direction::East,
        }
    }
}

impl Ship {
    fn apply_all(mut self, instructions: &Instructions) -> Self {
        for instruction in &instructions.0 {
            self = self.apply(*instruction);
        }
        self
    }

    fn apply(mut self, instruction: Instruction) -> Self {
        match instruction.action {
            Action::Move(direction) => {
                self.position = self.position.moved(direction, instruction.value);
            }
            Action::Turn(turn) => {
                self.facing = self.facing.turned(turn, instruction.value);
            }
            Action::Forward => {
                self.position = self.position.moved(self.facing, instruction.value);
            }
        }

        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct WaypointShip {
    position: Position,
    waypoint: Position,
}

impl Default for WaypointShip {
    fn default() -> Self {
        Self {
            position: Position::default(),
            waypoint: Position { east: 10, north: 1 },
        }
    }
}

impl WaypointShip {
    fn apply_all(mut self, instructions: &Instructions) -> Self {
        for instruction in &instructions.0 {
            self = self.apply(*instruction);
        }
        self
    }

    fn apply(mut self, instruction: Instruction) -> Self {
        match instruction.action {
            Action::Move(direction) => {
                self.waypoint = self.waypoint.moved(direction, instruction.value);
            }
            Action::Turn(turn) => {
                self.waypoint = self.waypoint.rotated(turn, instruction.value);
            }
            Action::Forward => {
                self.position = self.position + self.waypoint.scaled(instruction.value);
            }
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_parse_instruction() {
        let result = "F10".parse::<Instruction>().unwrap();
        let expected = Instruction {
            action: Action::Forward,
            value: 10,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_instructions() {
        let result = EXAMPLE.parse::<Instructions>().unwrap();
        let expected = Instructions(vec![
            Instruction {
                action: Action::Forward,
                value: 10,
            },
            Instruction {
                action: Action::Move(Direction::North),
                value: 3,
            },
            Instruction {
                action: Action::Forward,
                value: 7,
            },
            Instruction {
                action: Action::Turn(Turn::Right),
                value: 90,
            },
            Instruction {
                action: Action::Forward,
                value: 11,
            },
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(EXAMPLE).unwrap();
        assert_eq!(result, 25);
    }

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(EXAMPLE).unwrap();
        assert_eq!(result, 286);
    }

    #[test]
    fn test_rotate_waypoint_right() {
        let waypoint = Position { east: 10, north: 4 };
        let result = waypoint.rotated(Turn::Right, 90);
        assert_eq!(
            result,
            Position {
                east: 4,
                north: -10
            }
        );
    }

    #[test]
    fn test_rotate_waypoint_left() {
        let waypoint = Position { east: 10, north: 4 };
        let result = waypoint.rotated(Turn::Left, 90);
        assert_eq!(
            result,
            Position {
                east: -4,
                north: 10
            }
        );
    }

    #[test]
    fn test_turn_direction() {
        let result = Direction::East.turned(Turn::Right, 90);
        assert_eq!(result, Direction::South);

        let result = Direction::East.turned(Turn::Left, 180);
        assert_eq!(result, Direction::West);
    }
}
