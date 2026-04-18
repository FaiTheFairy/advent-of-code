#![allow(dead_code)]

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};

use anyhow::Result;

fn main() -> Result<()> {
    let banks: Banks = fs::read_to_string("input.txt")?.parse()?;

    let sol1 = banks.clone().redistribute_until_repeat();
    println!("Part 1: {sol1}");

    let sol2 = banks.clone().redistribute_repeat_loop_size();
    println!("Part 2: {sol2}");

    Ok(())
}

type Blocks = usize;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Banks(Vec<Blocks>);

impl Banks {
    fn redistribute_repeat_loop_size(&mut self) -> usize {
        let mut seen = HashMap::new();
        let mut idx = 0;
        while !seen.contains_key(self) {
            seen.insert(self.clone(), idx);
            self.redistribute();
            idx += 1;
        }
        // idx indicates the number of items in `seen`.
        // `self` is the last banks instance, that we know is
        // a repeat and exists in `seen`.
        (idx, seen.get(self));
        idx - seen
            .get(self)
            .expect("should have value that broke above loop")
    }

    fn redistribute_until_repeat(&mut self) -> usize {
        let mut seen = HashSet::new();
        while seen.insert(self.clone()) {
            self.redistribute();
        }
        seen.len()
    }

    fn redistribute(&mut self) {
        let max_idx = self
            .0
            .iter()
            .enumerate()
            .max_by(|(idx_a, a), (idx_b, b)| match a.cmp(b) {
                Ordering::Equal => Ordering::reverse(idx_a.cmp(idx_b)),
                ord => ord,
            })
            .map(|(idx, _a)| idx)
            .unwrap_or(0);

        let mut blocks = self.0[max_idx];
        self.0[max_idx] = 0;

        let mut idx = max_idx + 1;
        let len = self.0.len();
        while blocks > 0 {
            self.0[idx % len] += 1;
            idx += 1;
            blocks -= 1;
        }
    }
}

impl FromStr for Banks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let inner = s
            .split_whitespace()
            .map(str::parse::<usize>)
            .collect::<Result<_, _>>()?;

        Ok(Self(inner))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0 2 7 0";

    #[test]
    fn redistribute_repeat_loop_size() {
        let mut banks = Banks(vec![0, 2, 7, 0]);
        assert_eq!(banks.redistribute_repeat_loop_size(), 4);
    }

    #[test]
    fn test_redistribute_until_repeat() {
        let mut banks = Banks(vec![0, 2, 7, 0]);
        assert_eq!(banks.redistribute_until_repeat(), 5);
    }

    #[test]
    fn test_redistribute() {
        let mut banks = Banks(vec![0, 2, 7, 0]);

        banks.redistribute();
        assert_eq!(banks, Banks(vec![2, 4, 1, 2]));

        banks.redistribute();
        assert_eq!(banks, Banks(vec![3, 1, 2, 3]));

        banks.redistribute();
        assert_eq!(banks, Banks(vec![0, 2, 3, 4]));

        banks.redistribute();
        assert_eq!(banks, Banks(vec![1, 3, 4, 1]));

        banks.redistribute();
        assert_eq!(banks, Banks(vec![2, 4, 1, 2]));
    }

    #[test]
    fn test_parse() {
        let result: Banks = EXAMPLE.parse().unwrap();
        let expected = Banks(vec![0, 2, 7, 0]);
        assert_eq!(result, expected);
    }
}
