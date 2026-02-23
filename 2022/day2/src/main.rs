#![allow(dead_code, unused)]
use std::{cmp::Ordering, str::FromStr};

use anyhow::{Context, Result, bail};

fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Guide(Vec<GuideEntry>);

impl FromStr for Guide {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let entries = s
            .lines()
            .map(|l| l.parse::<GuideEntry>())
            .collect::<Result<Vec<GuideEntry>>>()?;
        Ok(Self(entries))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GuideEntry {
    opponent: Hand,
    me: Hand,
}

impl FromStr for GuideEntry {
    type Err = anyhow::Error;

    /// Takes "A Y"
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (opponent, me) = s
            .trim()
            .split_once(' ')
            .with_context(|| format!(r#"entry not seperated by space "{s}""#))?;
        // now we have opponent = "A", me = "Y"
        let opponent = opponent.parse::<Hand>()?;
        let me = me.parse::<Hand>()?;
        Ok(Self { opponent, me })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => bail!("Letter does not match any of the hands"),
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Hand {
    fn play(&self, other: &Hand) -> Outcome {
        use Hand::*;
        match (self, other) {
            (a, b) if a == b => Outcome::Draw,
            (Scissors, Paper) | (Paper, Rock) | (Rock, Scissors) => Outcome::Win,
            _ => Outcome::Lose,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HAND: &str = "A";
    const ENTRY: &str = "B Z";
    const GUIDE: &str = "A Y
                         B X
                         C Z";

    #[test]
    fn test_parse_hand() {
        let result = HAND.parse::<Hand>().unwrap();
        let expected = Hand::Rock;
        assert_eq!(result, expected);
    }
    #[test]
    fn test_parse_entry() {
        let result = ENTRY.parse::<GuideEntry>().unwrap();
        let expected = GuideEntry {
            opponent: Hand::Paper,
            me: Hand::Scissors,
        };
        assert_eq!(result, expected);
    }
    #[test]
    fn test_parse_guide() {
        let result = GUIDE.parse::<Guide>().unwrap();
        let expected = Guide(vec![
            GuideEntry {
                opponent: Hand::Rock,
                me: Hand::Paper,
            },
            GuideEntry {
                opponent: Hand::Paper,
                me: Hand::Rock,
            },
            GuideEntry {
                opponent: Hand::Scissors,
                me: Hand::Scissors,
            },
        ]);
        assert_eq!(result, expected);
    }
}
