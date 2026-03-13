use anyhow::{Result, anyhow, ensure};
use std::{collections::HashMap, fs, str::FromStr};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let sol1 = solve_part_1(&input)?;
    println!(
        "Part 1. difference between most and least common element counts after 10 steps = {sol1}"
    );

    let sol2 = solve_part_2(&input)?;
    println!(
        "Part 2. difference between most and least common element counts after 40 steps = {sol2}"
    );

    Ok(())
}

fn solve_part_1(input: &str) -> Result<usize> {
    let mut input = input.parse::<Input>()?;
    input.step_for(10);
    Ok(input.diff_between_most_and_least_common())
}

fn solve_part_2(input: &str) -> Result<usize> {
    let input = input.parse::<Input>()?;
    let mut pair_counts = input.initial_pair_counts();

    for _ in 0..40 {
        pair_counts = input.step_pair_counts(&pair_counts);
    }

    let char_counts = input.char_counts_from_pairs(&pair_counts);

    let max = char_counts.values().max().copied().unwrap();
    let min = char_counts.values().min().copied().unwrap();

    Ok(max - min)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    polymer: String,
    rules: HashMap<[char; 2], char>,
}

impl Input {
    fn step_pair_counts(
        &self,
        pair_counts: &HashMap<[char; 2], usize>,
    ) -> HashMap<[char; 2], usize> {
        let mut next = HashMap::new();

        for (&[a, b], &count) in pair_counts {
            if let Some(&insert) = self.rules.get(&[a, b]) {
                next.entry([a, insert])
                    .and_modify(|n| *n += count)
                    .or_insert(count);

                next.entry([insert, b])
                    .and_modify(|n| *n += count)
                    .or_insert(count);
            } else {
                next.entry([a, b])
                    .and_modify(|n| *n += count)
                    .or_insert(count);
            }
        }

        next
    }

    fn initial_pair_counts(&self) -> HashMap<[char; 2], usize> {
        let chars: Vec<char> = self.polymer.chars().collect();
        let mut out = HashMap::new();

        for window in chars.array_windows::<2>() {
            out.entry(*window)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        out
    }

    fn char_counts_from_pairs(
        &self,
        pair_counts: &HashMap<[char; 2], usize>,
    ) -> HashMap<char, usize> {
        let mut counts = HashMap::new();

        for (&[a, _], &count) in pair_counts {
            *counts.entry(a).or_insert(0) += count;
        }

        // adds the final character of the original polymer
        let last = self.polymer.chars().last().unwrap();
        *counts.entry(last).or_insert(0) += 1;

        counts
    }

    fn step_for(&mut self, times: usize) {
        for _ in 0..times {
            self.step();
        }
    }

    fn diff_between_most_and_least_common(&self) -> usize {
        self.count_most_common() - self.count_least_common()
    }

    fn count_most_common(&self) -> usize {
        let counts = self.char_counts();

        counts.values().max().copied().unwrap()
    }

    fn count_least_common(&self) -> usize {
        let counts = self.char_counts();

        counts.values().min().copied().unwrap()
    }

    fn char_counts(&self) -> HashMap<char, usize> {
        let mut counts = HashMap::new();

        for c in self.polymer.chars() {
            counts.entry(c).and_modify(|n| *n += 1).or_insert(1);
        }
        counts
    }

    fn step(&mut self) {
        let chars: Vec<char> = self.polymer.chars().collect();

        if chars.len() < 2 {
            return;
        }

        let mut next = String::new();

        for window in chars.array_windows::<2>() {
            next.push(window[0]);

            if let Some(&insert) = self.rules.get(window) {
                next.push(insert);
            }
        }

        next.push(*chars.last().unwrap());
        self.polymer = next;
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    /// parses "CH -> B" to
    /// `PairInsertion { adjacent: ['C', 'H'], insert: 'B' }`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let polymer = lines.next().ok_or(anyhow!("Empty first line"))?.to_owned();

        let mut rules = HashMap::with_capacity(s.lines().count() - 1);

        // we skip the empty line
        for rule in lines.skip(1) {
            let (adjacent, insert) = parse_rule(rule)?;
            rules.insert(adjacent, insert);
        }

        Ok(Self { polymer, rules })
    }
}

/// Parses "CH -> B" to `(['C', 'H'], 'B')`
fn parse_rule(s: &str) -> Result<([char; 2], char)> {
    let (left, right) = s
        .trim()
        .split_once(" -> ")
        .ok_or_else(|| anyhow!("invalid rule format: {s}"))?;
    ensure!(
        left.chars().count() == 2,
        "rule must have 2 adjacent chars: {s}"
    );
    ensure!(
        right.chars().count() == 1,
        "rule must have 1 inserted char: {s}"
    );
    let mut left_chars = left.chars();
    let adjacent = [left_chars.next().unwrap(), left_chars.next().unwrap()];
    let insert = right.chars().next().unwrap();
    Ok((adjacent, insert))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(EXAMPLE).unwrap();
        assert_eq!(result, 2188189693529);
    }

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(EXAMPLE).unwrap();
        assert_eq!(result, 1588);
    }

    #[test]
    fn applies_rules_once() {
        let mut rules = HashMap::new();
        rules.insert(['C', 'H'], 'B');
        rules.insert(['H', 'H'], 'N');
        rules.insert(['C', 'B'], 'H');
        rules.insert(['N', 'N'], 'C');
        rules.insert(['B', 'H'], 'H');
        rules.insert(['N', 'C'], 'B');
        rules.insert(['N', 'B'], 'B');
        rules.insert(['B', 'N'], 'B');
        rules.insert(['B', 'B'], 'N');
        rules.insert(['B', 'C'], 'B');
        rules.insert(['C', 'C'], 'N');
        rules.insert(['C', 'N'], 'C');

        let mut input = Input {
            polymer: "NNCB".to_owned(),
            rules,
        };
        input.step();

        assert_eq!(input.polymer, "NCNBCHB");
    }

    #[test]
    fn test_parse_input() {
        let result = "NNCB

CH -> B
HH -> N
CB -> H"
            .parse::<Input>()
            .unwrap();

        let expected = Input {
            polymer: "NNCB".to_owned(),
            rules: HashMap::from([(['C', 'H'], 'B'), (['H', 'H'], 'N'), (['C', 'B'], 'H')]),
        };

        assert_eq!(result, expected);
    }
}
