#![allow(unused)]

use std::{fs, str::FromStr};

use anyhow::Result;

fn main() -> Result<()> {
    let polymer: Polymer = fs::read_to_string("input.txt")?.trim().into();

    let sol1 = polymer.reacted_fully().len();
    println!("Part 1: {sol1}");

    let sol2 = polymer.shortest_reacted_without_one_unit();
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Polymer(Vec<u8>);

impl Polymer {
    // Example: bAcCaCBAcCc
    //
    //  unit | last | reacts? | action         | stack
    // ------+------+---------+----------------+---------------------
    //   b   |  -   |   -     | push           | [b]
    //   A   |  b   |   no    | push           | [b, A]
    //   c   |  A   |   no    | push           | [b, A, c]
    //   C   |  c   |  yes    | pop            | [b, A]
    //   a   |  A   |  yes    | pop            | [b]
    //   C   |  b   |   no    | push           | [b, C]
    //   B   |  C   |   no    | push           | [b, C, B]
    //   A   |  B   |   no    | push           | [b, C, B, A]
    //   c   |  A   |   no    | push           | [b, C, B, A, c]
    //   C   |  c   |  yes    | pop            | [b, C, B, A]
    //   c   |  A   |   no    | push           | [b, C, B, A, c]
    fn reacted_fully(&self) -> Self {
        let mut stack = Vec::with_capacity(self.0.len());

        for &unit in &self.0 {
            match stack.last().copied() {
                Some(last) if reacts(last, unit) => {
                    stack.pop();
                }
                _ => stack.push(unit),
            }
        }

        Self(stack)
    }

    fn shortest_reacted_without_one_unit(&self) -> usize {
        (b'a'..=b'z')
            .map(|unit| self.without_unit(unit).reacted_fully().len())
            .min()
            .unwrap_or(0)
    }

    fn without_unit(&self, to_remove: u8) -> Self {
        Self(
            self.0
                .iter()
                .copied()
                .filter(|unit| !unit.eq_ignore_ascii_case(&to_remove))
                .collect(),
        )
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<&str> for Polymer {
    fn from(value: &str) -> Self {
        Self(value.bytes().collect())
    }
}

fn reacts(a: u8, b: u8) -> bool {
    a != b && a.eq_ignore_ascii_case(&b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let result = Polymer::from("dabAcCaCBAcCcaDA").reacted_fully();
        assert_eq!(result, Polymer("dabCBAcaDA".bytes().collect()));
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn part_2() {
        let result = Polymer::from("dabAcCaCBAcCcaDA").shortest_reacted_without_one_unit();
        assert_eq!(result, 4);
    }
}
