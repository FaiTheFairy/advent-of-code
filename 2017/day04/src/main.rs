use anyhow::Result;
use std::{collections::HashSet, fs, str::FromStr};

fn main() -> Result<()> {
    let input: Input = fs::read_to_string("input.txt")?.parse()?;

    let sol1 = input.count_valid_v1();
    println!("Part 1: {sol1}");

    let sol2 = input.count_valid_v2();
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Input(Vec<Passphrase>);

impl Input {
    fn count_valid_v1(&self) -> usize {
        self.0.iter().filter(|pass| pass.is_valid_v1()).count()
    }

    fn count_valid_v2(&self) -> usize {
        self.0.iter().filter(|pass| pass.is_valid_v2()).count()
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner: Vec<_> = s.lines().map(str::to_owned).map(Passphrase).collect();

        Ok(Self(inner))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Passphrase(String);

impl Passphrase {
    fn is_valid_v1(&self) -> bool {
        let word_count = self.0.split_whitespace().count();
        let set: HashSet<&str> = self.0.split_whitespace().collect();
        word_count == set.len()
    }

    fn is_valid_v2(&self) -> bool {
        let mut words: Vec<Vec<char>> = self
            .0
            .split_whitespace()
            .map(|s| {
                let mut vec = s.chars().collect::<Vec<char>>();
                vec.sort_unstable();
                vec
            })
            .collect();

        let prededup_len = words.len();

        words.sort_unstable();
        words.dedup();

        words.len() == prededup_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa";

    #[test]
    fn test_is_valid_v2() {
        assert!(Passphrase("abcde fghij".into()).is_valid_v2());
        assert!(!Passphrase("abcde xyz ecdab".into()).is_valid_v2());
        assert!(Passphrase("a ab abc abd abf abj".into()).is_valid_v2());
        assert!(Passphrase("iiii oiii ooii oooi oooo".into()).is_valid_v2());
        assert!(!Passphrase("oiii ioii iioi iiio".into()).is_valid_v2());
    }

    #[test]
    fn test_part_1() {
        let result = EXAMPLE.parse::<Input>().unwrap().count_valid_v1();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_is_valid_v1() {
        assert!(Passphrase("aa bb cc dd ee".to_string()).is_valid_v1());
        assert!(!Passphrase("aa bb cc dd aa".to_string()).is_valid_v1());
        assert!(Passphrase("aa bb cc dd aaa".to_string()).is_valid_v1());
    }
}
