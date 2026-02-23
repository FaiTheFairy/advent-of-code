#![allow(dead_code, unused)]
use std::{cmp::Ordering, str::FromStr};

use anyhow::{Context, Result, bail};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./input.txt")?;
    let guide = input.parse::<Guide>()?;
    let score = guide.score();
    println!("Part 1. Total score upon executing the strategy guide is {score}");

    let guide2 = Guide::from_str_part2(&input)?;
    let score2 = guide2.score();
    println!("Part 2. Total score upon executing the new strategy is {score2}");

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

impl Guide {
    fn from_str_part2(s: &str) -> Result<Self> {
        let entries = s
            .lines()
            .map(GuideEntry::form_str_part2)
            .collect::<Result<Vec<GuideEntry>>>()?;
        Ok(Self(entries))
    }
}

impl Guide {
    fn score(&self) -> usize {
        self.0.iter().map(GuideEntry::score).sum()
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

impl GuideEntry {
    fn form_str_part2(s: &str) -> Result<Self> {
        let (opponent, me) = s
            .trim()
            .split_once(' ')
            .with_context(|| format!(r#"entry not separated by space {s}"#))?;
        let opponent = opponent.parse::<Hand>()?;
        let me = match me {
            "X" => opponent.beats(),
            "Y" => opponent.draws_to(),
            "Z" => opponent.beaten_by(),
            _ => bail!(r#"character does not map to any action "{me}""#),
        };

        Ok(Self { opponent, me })
    }
}

impl GuideEntry {
    fn score(&self) -> usize {
        let shape_score = match self.me {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        };
        let win_score = match self.me.play(&self.opponent) {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        };

        shape_score + win_score
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Hand {
    fn play(&self, against: &Hand) -> Outcome {
        use Hand::*;
        match (self, against) {
            (a, b) if a == b => Outcome::Draw,
            (Scissors, Paper) | (Paper, Rock) | (Rock, Scissors) => Outcome::Win,
            _ => Outcome::Lose,
        }
    }

    fn beats(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    fn beaten_by(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn draws_to(&self) -> Hand {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HAND: &str = "A";
    const ENTRY: &str = "B Z";
    const ENTRY_1: &str = "A Z";
    const ENTRY_2: &str = "C Z";
    const ENTRY_3: &str = "B X";
    const ENTRY_4: &str = "B Y";
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

    #[test]
    fn test_hand_play() {
        use Outcome::{Draw, Lose, Win};

        let rock = Hand::Rock;
        let paper = Hand::Paper;
        let scissors = Hand::Scissors;

        assert_eq!(rock.play(&paper), Lose);
        assert_eq!(rock.play(&scissors), Win);

        assert_eq!(paper.play(&scissors), Lose);
        assert_eq!(paper.play(&rock), Win);

        assert_eq!(scissors.play(&paper), Win);
        assert_eq!(scissors.play(&rock), Lose);

        assert_eq!(rock.play(&rock), Draw);
    }

    #[test]
    fn test_hand_beats() {
        use Hand::*;
        assert_eq!(Rock.beats(), Scissors);
        assert_eq!(Paper.beats(), Rock);
        assert_eq!(Scissors.beats(), Paper);
    }

    #[test]
    fn test_hand_beaten_by() {
        use Hand::*;
        assert_eq!(Rock.beaten_by(), Paper);
        assert_eq!(Paper.beaten_by(), Scissors);
        assert_eq!(Scissors.beaten_by(), Rock);
    }

    #[test]
    fn test_score_entry() {
        let entry = GuideEntry {
            opponent: Hand::Paper,
            me: Hand::Scissors,
        };
        let result = entry.score();
        assert_eq!(result, 9);
    }

    #[test]
    fn test_score_guide() {
        let guide = GUIDE.parse::<Guide>().unwrap();
        let score = guide.score();
        assert_eq!(score, 15);
    }

    #[test]
    fn test_score_guide_part_2() {
        let guide = Guide::from_str_part2(GUIDE).unwrap();
        let score = guide.score();
        assert_eq!(score, 12);
    }
}
