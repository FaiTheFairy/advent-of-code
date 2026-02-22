#![allow(dead_code, unused)]

use std::{collections::HashMap, fs};

use anyhow::{Context, Result, bail, ensure};

const START: Node = Node([b'A'; 3]);
// const START: Node = Node([b'Q', b'N', b'L']);
const END: Node = Node([b'Z'; 3]);
// const END: Node = Node([b'V', b'L', b'M']);

fn main() -> Result<()> {
    let soln1 = solve_part_1()?;
    println!("Part 1. Required steps = {soln1}");

    Ok(())
}

fn solve_part_1() -> Result<usize> {
    let input = fs::read_to_string("./input.txt")?;
    let map = parse_input(&input)?;
    map.solve()
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<u8> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            b'L' => Ok(Self::Left),
            b'R' => Ok(Self::Right),
            _ => bail!("Directions must be either 'L' or 'R'"),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Node([u8; 3]);

impl TryFrom<&str> for Node {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let value = value.trim().as_bytes();
        if value.len() == 3 {
            Ok(Self([value[0], value[1], value[2]]))
        } else {
            bail!("Node was not 3 characters.");
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct LeftRight(Node, Node);

#[derive(Debug, PartialEq, Eq)]
struct Map {
    steps: Vec<Direction>,
    instructions: HashMap<Node, LeftRight>,
}

impl Map {
    /// Attempts to solve the puzzle by starting with node AAA by
    /// and following the next element based on the sequence of steps.
    /// This returns the number of steps it took to go from `AAA` to `ZZZ`.
    /// Returns `None` if no solution is found
    fn solve(&self) -> Result<usize> {
        ensure!(
            self.instructions.contains_key(&START),
            "missing starting node"
        );
        ensure!(!self.steps.is_empty(), "no steps");

        let mut current_node = START;
        let mut count = 0;
        let mut steps = self.steps.iter().cycle();

        while current_node != END {
            let step = steps.next().unwrap();

            let lr = self
                .instructions
                .get(&current_node)
                .with_context(|| format!("missing instruction for {:?}", current_node))?;

            current_node = match step {
                Direction::Left => lr.0,
                Direction::Right => lr.1,
            };
            count += 1;
        }

        Ok(count)
    }
}

fn parse_input(input: &str) -> Result<Map> {
    // we start by parsing the first line which indicates the steps we have to take.
    let mut lines = input.lines();
    let steps = lines.next().context("Input is empty")?;
    let steps = steps
        .bytes()
        .map(Direction::try_from)
        .collect::<Result<Vec<Direction>>>()?;

    // then we skip a line and parse all entries below to generate HashMap<Node, LeftRight>
    let mut instructions = HashMap::new();
    for line in lines.skip(1) {
        let (node, leftright) = line
            .split_once("=")
            .map(|(n, lr)| (n.trim(), lr.trim().trim_matches(['(', ')'])))
            .with_context(|| format!(r#"Malformed entry "{line}""#))?;

        let node = Node::try_from(node)?;

        let (left, right) = leftright
            .split_once(", ")
            .with_context(|| format!(r#"Malformed instructions "{leftright}""#))?;
        let (left, right) = (Node::try_from(left)?, Node::try_from(right)?);

        instructions.insert(node, LeftRight(left, right));
    }

    Ok(Map {
        steps,
        instructions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    use Direction::*;

    #[test]
    fn test_parse_input() {
        let result = parse_input(EXAMPLE2).unwrap();
        let instructions: HashMap<Node, LeftRight> = HashMap::from([
            (Node(*b"AAA"), LeftRight(Node(*b"BBB"), Node(*b"BBB"))),
            (Node(*b"BBB"), LeftRight(Node(*b"AAA"), Node(*b"ZZZ"))),
            (Node(*b"ZZZ"), LeftRight(Node(*b"ZZZ"), Node(*b"ZZZ"))),
        ]);
        let expected = Map {
            steps: vec![Left, Left, Right],
            instructions,
        };

        assert_eq!(result, expected)
    }

    #[test]
    fn test_solve_ex1() {
        let map = parse_input(EXAMPLE1).unwrap();
        let result = map.solve().unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_solve_ex2() {
        let map = parse_input(EXAMPLE2).unwrap();
        let result = map.solve().unwrap();
        assert_eq!(result, 6);
    }
}
