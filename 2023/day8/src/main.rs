#![allow(dead_code, unused)]

use std::{collections::HashMap, fs};

use anyhow::{Context, Result, bail, ensure};

const START: Node = Node([b'A'; 3]);
// const START: Node = Node([b'Q', b'N', b'L']);
const END: Node = Node([b'Z'; 3]);
// const END: Node = Node([b'V', b'L', b'M']);

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt")?;
    let map = parse_input(&input)?;
    let soln1 = map.solve()?;
    let soln2 = map.solve_part_2()?;

    println!("Part 1. Required steps = {soln1}");
    println!("Part 2. Required steps = {soln2}");

    Ok(())
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
#[cfg_attr(test, derive(PartialOrd, Ord))]
struct Node([u8; 3]);

impl Node {
    fn ends_with(&self, byte: u8) -> bool {
        self.0[2] == byte
    }
}

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

    fn nodes_ending_with(&self, byte: u8) -> Vec<&Node> {
        self.instructions
            .keys()
            .filter(|&n| n.ends_with(byte))
            .collect()
    }

    fn solve_part_2(&self) -> Result<usize> {
        let mut current_nodes = self.nodes_ending_with(b'A');
        ensure!(!current_nodes.is_empty());
        let mut steps = self.steps.iter().cycle();
        let mut count = 0;
        while !current_nodes.iter().all(|&n| n.ends_with(b'Z')) {
            let step = steps.next().unwrap();
            for node in current_nodes.iter_mut() {
                let lr: &LeftRight = self.leftright(node)?;
                *node = match step {
                    Direction::Left => &lr.0,
                    Direction::Right => &lr.1,
                };
            }
            count += 1;
        }
        Ok(count)
    }

    fn leftright(&self, node: &Node) -> Result<&LeftRight> {
        self.instructions
            .get(node)
            .with_context(|| format!("missing instruction for {:?}", node))
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

    const EXAMPLE_PART2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

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

    #[test]
    fn test_nodes_ending_with() {
        let map = parse_input(EXAMPLE1).unwrap();
        let result = map.nodes_ending_with(b'A');
        let expected = vec![&Node([b'A', b'A', b'A'])];
        assert_eq!(result, expected);

        let map = parse_input(EXAMPLE_PART2).unwrap();
        let mut result = map.nodes_ending_with(b'A');
        dbg!(&result);
        let mut expected = vec![&Node([b'1', b'1', b'A']), &Node([b'2', b'2', b'A'])];
        result.sort_unstable();
        expected.sort_unstable();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_part_2() {
        let map = parse_input(EXAMPLE_PART2).unwrap();
        let result = map.solve_part_2().unwrap();
        assert_eq!(result, 6);
    }
}
