use std::{fs, str::FromStr};

use anyhow::Result;

fn main() -> Result<()> {
    let input: Input = fs::read_to_string("input.txt")?.parse()?;

    let sol1 = input.root.metadata_sum();
    println!("Part 1: {sol1}");

    let sol2 = input.root.value();
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Input {
    root: Node,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut numbers = s
            .split_whitespace()
            .map(str::parse::<usize>)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter();

        let root = Node::parse(&mut numbers);

        Ok(Self { root })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn parse(numbers: &mut impl Iterator<Item = usize>) -> Self {
        let child_count = numbers.next().unwrap();
        let metadata_count = numbers.next().unwrap();

        let children = (0..child_count).map(|_| Self::parse(numbers)).collect();

        let metadata = (0..metadata_count)
            .map(|_| numbers.next().unwrap())
            .collect();

        Self { children, metadata }
    }

    fn metadata_sum(&self) -> usize {
        self.metadata.iter().sum::<usize>()
            + self.children.iter().map(Node::metadata_sum).sum::<usize>()
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .filter_map(|&idx| idx.checked_sub(1))
                .filter_map(|idx| self.children.get(idx))
                .map(|node| node.value())
                .sum()
        }
    }
}
