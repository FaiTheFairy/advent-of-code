#![allow(unused)]

use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use anyhow::{Result, anyhow};

pub fn solve_part_1(input: &str) -> Result<usize> {
    let input: Input = input.parse()?;

    Ok(input
        .updates
        .iter()
        .filter(|update| update.is_valid(&input.rules))
        .map(Update::middle)
        .sum())
}

pub fn solve_part_2(input: &str) -> Result<usize> {
    let input: Input = input.parse()?;

    Ok(input
        .updates
        .iter()
        .filter(|update| !update.is_valid(&input.rules))
        .map(|update| update.corrected(&input.rules))
        .map(|update| update.middle())
        .sum())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Input {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (rules, updates) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("missing empty blank line between rules and updates"))?;

        let rules: Vec<Rule> = rules
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        let updates: Vec<Update> = updates
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self { rules, updates })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Rule {
    before: usize,
    after: usize,
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (before, after) = s
            .trim()
            .split_once('|')
            .ok_or_else(|| anyhow!("rule line missing '|': {s}"))?;

        let before = before.parse()?;
        let after = after.parse()?;

        Ok(Self { before, after })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Update(Vec<usize>);

impl Update {
    fn corrected(&self, rules: &[Rule]) -> Self {
        let mut pages = self.0.clone();

        pages.sort_by(|a, b| {
            for rule in rules {
                if rule.before == *a && rule.after == *b {
                    return Ordering::Less;
                }
                if rule.before == *b && rule.after == *a {
                    return Ordering::Greater;
                }
            }
            Ordering::Equal
        });

        Self(pages)
    }

    fn is_valid(&self, rules: &[Rule]) -> bool {
        let positions: HashMap<usize, usize> = self
            .0
            .iter()
            .copied()
            .enumerate()
            .map(|(idx, page)| (page, idx))
            .collect();

        for rule in rules {
            let Some(&before_idx) = positions.get(&rule.before) else {
                continue;
            };
            let Some(&after_idx) = positions.get(&rule.after) else {
                continue;
            };

            if before_idx > after_idx {
                return false;
            }
        }

        true
    }

    fn middle(&self) -> usize {
        self.0[self.0.len() / 2]
    }
}

impl FromStr for Update {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let pages = s
            .trim()
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self(pages))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_parse_rule() {
        let rule: Rule = "47|53".parse().unwrap();
        assert_eq!(
            rule,
            Rule {
                before: 47,
                after: 53
            }
        );
    }

    #[test]
    fn test_parse_update() {
        let update: Update = "75,47,61,53,29".parse().unwrap();
        assert_eq!(update, Update(vec![75, 47, 61, 53, 29]));
    }

    #[test]
    fn test_update_is_valid() {
        let input: Input = EXAMPLE.parse().unwrap();
        let update = Update(vec![75, 47, 61, 53, 29]);
        assert!(update.is_valid(&input.rules));
    }

    #[test]
    fn test_update_is_invalid() {
        let input: Input = EXAMPLE.parse().unwrap();
        let update = Update(vec![75, 97, 47, 61, 53]);
        assert!(!update.is_valid(&input.rules));
    }

    #[test]
    fn test_solve_part_1_example() {
        assert_eq!(solve_part_1(EXAMPLE).unwrap(), 143);
    }
}
