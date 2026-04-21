use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input: Input = fs::read_to_string("input.txt")?.as_str().into();

    let sol1 = input.checksum();
    println!("Part 1: {sol1}");

    let sol2 = input.common_leters_of_correct_boxes();
    println!("Part 2: {sol2}");

    Ok(())
}

struct Input(Vec<BoxId>);

impl Input {
    fn common_leters_of_correct_boxes(&self) -> String {
        for (i, a) in self.0.iter().enumerate() {
            for b in &self.0[i + 1..] {
                if let Some(common) = a.common_if_one_off(b) {
                    return common;
                }
            }
        }
        unreachable!("AoC input should have solution");
    }

    fn checksum(&self) -> usize {
        let twos = self.0.iter().filter(|id| id.has_exactly(2)).count();
        let threes = self.0.iter().filter(|id| id.has_exactly(3)).count();
        twos * threes
    }
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let inner = value
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(BoxId::from)
            .collect();
        Self(inner)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BoxId(String);

impl BoxId {
    fn common_if_one_off(&self, other: &Self) -> Option<String> {
        let mut diffs = 0;

        let common: String = self
            .0
            .chars()
            .zip(other.0.chars())
            .filter_map(|(a, b)| {
                if a == b {
                    Some(a)
                } else {
                    diffs += 1;
                    None
                }
            })
            .collect();

        (diffs == 1).then_some(common)
    }

    fn has_exactly(&self, target: u8) -> bool {
        let mut counts = [0u8; 26];

        for byte in self.0.bytes() {
            counts[usize::from(byte - b'a')] += 1;
        }

        counts.into_iter().any(|count| count == target)
    }
}

impl From<&str> for BoxId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
";

    const EXAMPLE_2: &str = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";

    #[test]
    fn test_part_2() {
        let input: Input = EXAMPLE_2.into();
        let result = input.common_leters_of_correct_boxes();
        assert_eq!(result, "fgij");
    }

    #[test]
    fn test_part_1() {
        let input: Input = EXAMPLE_1.into();
        let checksum = input.checksum();
        assert_eq!(checksum, 12);
    }
}
