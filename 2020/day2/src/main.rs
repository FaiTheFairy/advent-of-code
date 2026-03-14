use std::{fs, str::FromStr};

use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let list = fs::read_to_string("input.txt")?.parse::<List>()?;
    let sol1 = list.solve_part_1();
    println!("Part 1. there are {sol1} valid passwords");

    let sol2 = list.solve_part_2();
    println!("Part 2. there are {sol2} valid passwords");

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct List(Vec<ListEntry>);

impl List {
    fn solve_part_1(&self) -> usize {
        self.0
            .iter()
            .filter(|&list_entry| {
                let ListEntry { rule, password } = list_entry;
                let PasswordRule {
                    min_repeat,
                    max_repeat,
                    letter,
                } = rule;

                let count = password.chars().filter(|c| c == letter).count();

                count >= *min_repeat && count <= *max_repeat
            })
            .count()
    }

    fn solve_part_2(&self) -> usize {
        self.0
            .iter()
            .filter(|&entry| {
                let ListEntry { rule, password } = entry;
                let PasswordRule {
                    min_repeat,
                    max_repeat,
                    letter,
                } = rule;

                let chars: Vec<char> = password.chars().collect();

                let cond1 = chars.get(min_repeat - 1).is_some_and(|c| c == letter);
                let cond2 = chars.get(max_repeat - 1).is_some_and(|c| c == letter);

                cond1 ^ cond2
            })
            .count()
    }
}

impl FromStr for List {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let entries = s
            .lines()
            .map(|line| line.parse::<ListEntry>())
            .collect::<Result<_, _>>()?;

        Ok(Self(entries))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ListEntry {
    rule: PasswordRule,
    password: String,
}

impl FromStr for ListEntry {
    type Err = anyhow::Error;

    /// Parses "1-3 a: abcde" to `ListEntry`
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (rule, password) = s
            .trim()
            .split_once(": ")
            .ok_or_else(|| anyhow!("malformed line entry: {s}"))?;

        let rule = rule.parse::<PasswordRule>()?;
        let password = password.to_string();

        Ok(Self { rule, password })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PasswordRule {
    min_repeat: usize,
    max_repeat: usize,
    letter: char,
}

impl FromStr for PasswordRule {
    type Err = anyhow::Error;

    /// Parses "1-3 a" to `PasswordRule`
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (range, letter) = s
            .trim()
            .split_once(' ')
            .ok_or_else(|| anyhow!("Error parsing rule: {s}"))?;

        let letter = letter.parse::<char>()?;
        let (min_repeat, max_repeat) = range
            .split_once('-')
            .ok_or_else(|| anyhow!("range in rule malformed: {s}"))?;

        let min_repeat = min_repeat.parse::<usize>()?;
        let max_repeat = max_repeat.parse::<usize>()?;

        Ok(Self {
            min_repeat,
            max_repeat,
            letter,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn test_solve_part_1() {
        let result = EXAMPLE.parse::<List>().unwrap().solve_part_1();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_parse_input() {
        let result = EXAMPLE.parse::<List>().unwrap();
        let expected = List(vec![
            ListEntry {
                rule: PasswordRule {
                    min_repeat: 1,
                    max_repeat: 3,
                    letter: 'a',
                },
                password: "abcde".to_string(),
            },
            ListEntry {
                rule: PasswordRule {
                    min_repeat: 1,
                    max_repeat: 3,
                    letter: 'b',
                },
                password: "cdefg".to_string(),
            },
            ListEntry {
                rule: PasswordRule {
                    min_repeat: 2,
                    max_repeat: 9,
                    letter: 'c',
                },
                password: "ccccccccc".to_string(),
            },
        ]);
        assert_eq!(result, expected);
    }
}
