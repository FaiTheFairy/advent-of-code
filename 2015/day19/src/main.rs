use std::{collections::HashSet, fs, str::FromStr};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input: Input = fs::read_to_string("input.txt")?.parse()?;

    let variants = input.generate_unique_variants();
    let sol1 = variants.len();
    println!("Part 1: {sol1}");

    let sol2 = input
        .solve_part_2()
        .context("no solution found for part 2")?;
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    rules: Vec<Rule>,
    medicine: String,
}

impl Input {
    fn solve_part_2(&self) -> Option<usize> {
        let rules = self.reversed_rules();
        let mut molecule = self.medicine.clone();
        let mut steps = 0;

        while molecule != "e" {
            let mut changed = false;

            for rule in &rules {
                if let Some((idx, _)) = molecule.match_indices(&rule.from).next() {
                    let mut next =
                        String::with_capacity(molecule.len() - rule.from.len() + rule.to.len());
                    next.push_str(&molecule[..idx]);
                    next.push_str(&rule.to);
                    next.push_str(&molecule[idx + rule.from.len()..]);
                    molecule = next;
                    steps += 1;
                    changed = true;
                    break;
                }
            }

            if !changed {
                return None;
            }
        }

        Some(steps)
    }

    fn reversed_rules(&self) -> Vec<Rule> {
        let mut rules: Vec<Rule> = self.rules.iter().map(Rule::reversed).collect();
        rules.sort_by_key(|rule| std::cmp::Reverse(rule.from.len()));
        rules
    }

    fn generate_unique_variants(&self) -> HashSet<String> {
        let mut out = HashSet::new();

        for rule in &self.rules {
            out.extend(generate_once(&self.medicine, rule));
        }

        out
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (rules, medicine) = s.split_once("\n\n").context("input missing blank line")?;

        let rules: Vec<Rule> = rules
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        let medicine = medicine.trim().to_string();

        Ok(Self { rules, medicine })
    }
}

/// Generates one new molecule per position where `rule` can apply
fn generate_once(molecule: &str, rule: &Rule) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();

    for (idx, _) in molecule.match_indices(&rule.from) {
        let mut next = String::with_capacity(molecule.len() - rule.from.len() + rule.to.len());

        next.push_str(&molecule[..idx]);
        next.push_str(&rule.to);
        next.push_str(&molecule[idx + rule.from.len()..]);

        out.push(next);
    }

    out
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {
    from: String,
    to: String,
}

impl Rule {
    fn reversed(&self) -> Rule {
        Rule {
            from: self.to.clone(),
            to: self.from.clone(),
        }
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (from, to) = s
            .trim()
            .split_once(" => ")
            .map(|(f, t)| (f.to_string(), t.to_string()))
            .context("rule missing ' => '")?;

        Ok(Self { from, to })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "H => HO
H => OH
O => HH

HOH";

    #[test]
    fn test_generate_variants() {
        let result = EXAMPLE.parse::<Input>().unwrap().generate_unique_variants();
        let expected: HashSet<String> = ["HOOH", "HOHO", "OHOH", "HOOH", "HHHH"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_once() {
        let rule = Rule {
            from: "H".to_string(),
            to: "HO".to_string(),
        };
        let result = generate_once("HOH", &rule);
        let expected = vec!["HOOH".to_string(), "HOHO".to_string()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_input() {
        let result: Input = EXAMPLE.parse().unwrap();
        let expected = Input {
            rules: vec![
                Rule {
                    from: "H".to_string(),
                    to: "HO".to_string(),
                },
                Rule {
                    from: "H".to_string(),
                    to: "OH".to_string(),
                },
                Rule {
                    from: "O".to_string(),
                    to: "HH".to_string(),
                },
            ],
            medicine: "HOH".to_string(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_rule() {
        let result: Rule = "H => HO".parse().unwrap();
        let expected = Rule {
            from: "H".to_string(),
            to: "HO".to_string(),
        };
        assert_eq!(result, expected);
    }
}
