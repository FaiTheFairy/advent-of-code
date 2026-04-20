use anyhow::{Context, Result};
use std::{fmt::Write as _, fs, str::FromStr};

const KNOT_SIZE: usize = 256;
const SUFFIX: [u8; 5] = [17, 31, 73, 47, 23];

fn main() -> Result<()> {
    let input: Input = fs::read_to_string("input.txt")
        .context("failed to read input.txt")?
        .parse()?;

    let part_1 = input.solve_part_1();
    println!("Part 1: {part_1}");

    let part_2 = input.solve_part_2();
    println!("Part 2: {part_2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Input(String);

impl Input {
    fn solve_part_1(&self) -> usize {
        let lengths = self
            .0
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::parse::<usize>)
            .collect::<std::result::Result<Vec<_>, _>>()
            .expect("part 1 input must be a comma-separated list of usize lengths");

        let mut knot = Knot::new(KNOT_SIZE);
        knot.run_round_usize(&lengths);

        usize::from(knot.elements[0]) * usize::from(knot.elements[1])
    }

    fn solve_part_2(&self) -> String {
        knot_hash(&self.0)
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(s.trim().to_owned()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Knot {
    elements: Vec<u8>,
    pos: usize,
    skip_size: usize,
}

impl Knot {
    fn new(size: usize) -> Self {
        assert!(u8::try_from(size - 1).is_ok());

        let elements = (0..size)
            .map(|n| u8::try_from(n).expect("knot element must fit into u8"))
            .collect();

        Self {
            elements,
            pos: 0,
            skip_size: 0,
        }
    }

    fn run_round_usize(&mut self, lengths: &[usize]) {
        for &length in lengths {
            self.step(length);
        }
    }

    fn run_round_bytes(&mut self, lengths: &[u8]) {
        for &length in lengths {
            self.step(usize::from(length));
        }
    }

    fn step(&mut self, length: usize) {
        self.reverse_span(length);
        let n = self.elements.len();
        self.pos = (self.pos + length + self.skip_size) % n;
        self.skip_size += 1;
    }

    fn reverse_span(&mut self, len: usize) {
        let n = self.elements.len();
        assert!(len <= n, "span length cannot exceed knot size");

        for i in 0..(len / 2) {
            let a = (self.pos + i) % n;
            let b = (self.pos + len - 1 - i) % n;
            self.elements.swap(a, b);
        }
    }

    fn dense_hash(&self) -> [u8; 16] {
        let mut dense = [0u8; 16];

        for (i, block) in self.elements.chunks_exact(16).enumerate() {
            dense[i] = block.iter().copied().reduce(|acc, x| acc ^ x).unwrap();
        }

        dense
    }
}

fn knot_hash(input: &str) -> String {
    let mut lengths = input.as_bytes().to_vec();
    lengths.extend(SUFFIX);

    let mut knot = Knot::new(KNOT_SIZE);

    for _ in 0..64 {
        knot.run_round_bytes(&lengths);
    }

    let dense = knot.dense_hash();
    dense_to_hex(&dense)
}

fn dense_to_hex(dense: &[u8]) -> String {
    let mut out = String::with_capacity(dense.len() * 2);

    for byte in dense {
        write!(&mut out, "{byte:02x}").expect("writing to String cannot fail");
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let mut knot = Knot::new(5);
        knot.run_round_usize(&[3, 4, 1, 5]);

        assert_eq!(knot.elements, vec![3, 4, 2, 1, 0]);
        assert_eq!(
            usize::from(knot.elements[0]) * usize::from(knot.elements[1]),
            12
        );
    }

    #[test]
    fn test_knot_hash_examples() {
        assert_eq!(knot_hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(knot_hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(knot_hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(knot_hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
