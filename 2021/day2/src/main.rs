use std::{fs, str::FromStr};

use anyhow::{Context, bail};

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let sol1 = solve_part_1(&input);
    println!("Part 1. Product of final horizontal position and depth is {sol1}");

    let sol2 = solve_part_2(&input);
    println!("Part 2. Product of final horizontal position and depth is {sol2}");

    Ok(())
}

fn solve_part_1(commands: &str) -> isize {
    let mut position = Position::default();
    let commands = parse_commands_to_vec(commands);
    for command in commands {
        position.apply_move(command);
    }
    position.x * position.z
}

fn solve_part_2(commands: &str) -> isize {
    let mut position = Position::default();
    let commands = parse_commands_to_vec(commands);
    for command in commands {
        position.apply_move_2(command);
    }
    position.x * position.z
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: isize,
    z: isize,
    aim: isize,
}

impl Position {
    fn apply_move(&mut self, command: Command) {
        match command {
            Command::Forward(unit) => {
                self.x += unit as isize;
                // self.z += self.aim * unit as isize;
            }
            Command::Down(unit) => {
                self.z += unit as isize;
            }
            Command::Up(unit) => {
                self.z -= unit as isize;
            }
        }
    }

    /// Applies move per part 2 rules (aim-based)
    fn apply_move_2(&mut self, command: Command) {
        match command {
            Command::Forward(unit) => {
                self.x += unit as isize;
                self.z += self.aim * unit as isize;
            }
            Command::Down(unit) => {
                self.aim += unit as isize;
            }
            Command::Up(unit) => {
                self.aim -= unit as isize;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    /// Parses commands like "forward 5" to `Command::Forward(5usize)`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let (command, units) = s
            .split_once(" ")
            .with_context(|| format!("malformed entry: {s}"))?;

        let units = units.parse()?;

        match command {
            "forward" => Ok(Self::Forward(units)),
            "down" => Ok(Self::Down(units)),
            "up" => Ok(Self::Up(units)),
            _ => bail!("Unknown command: {command}"),
        }
    }
}

fn parse_commands_to_vec(s: &str) -> Vec<Command> {
    let results: Vec<_> = s
        .lines()
        .map(str::parse::<Command>)
        .collect::<Result<_, _>>()
        .unwrap();
    results
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const EXAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_command_parsing() {
        let result = "forward 5".parse::<Command>().unwrap();
        let expected = Command::Forward(5);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_example() {
        let results = parse_commands_to_vec(EXAMPLE);

        use Command::*;
        let expected = vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
        assert_eq!(results, expected);
    }

    #[test]
    fn test_solve_part_1() {
        let results = solve_part_1(EXAMPLE);
        assert_eq!(results, 900isize);
    }
}
