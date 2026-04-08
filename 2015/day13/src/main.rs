#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{Context, Result, bail};
use itertools::Itertools;

fn main() -> Result<()> {
    let dinner: Dinner = std::fs::read_to_string("input.txt")?.parse()?;

    let sol1 = dinner.best_happiness().context("no seating found")?;
    println!("Part 1: {sol1}");

    let mut dinner = dinner;
    dinner.add_me();
    let sol2 = dinner.best_happiness().context("no seating found")?;
    println!("Part 2: {sol2}");

    Ok(())
}

type Happiness = i32;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Dinner {
    people: Vec<String>,
    happiness: HashMap<(String, String), Happiness>,
}

impl Dinner {
    fn best_happiness(&self) -> Option<Happiness> {
        let first = self.people.first()?.clone();

        self.people[1..]
            .iter()
            .permutations(self.people.len() - 1)
            .map(|perm| {
                let seating: Vec<&String> = std::iter::once(&first).chain(perm).collect();
                self.score_seating(&seating)
            })
            .max()
    }

    fn score_seating(&self, seating: &[&String]) -> Happiness {
        let n = seating.len();
        let mut total = 0;

        for i in 0..n {
            let a = seating[i];
            let b = seating[(i + 1) % n];

            total += self.happiness_between(a, b);
            total += self.happiness_between(b, a);
        }

        total
    }

    fn happiness_between(&self, from: &str, to: &str) -> Happiness {
        *self
            .happiness
            .get(&(from.to_string(), to.to_string()))
            .unwrap_or(&0)
    }

    fn add_me(&mut self) {
        let me = "me".to_string();

        for person in &self.people {
            self.happiness.insert((me.clone(), person.clone()), 0);
            self.happiness.insert((person.clone(), me.clone()), 0);
        }

        self.people.push(me);
    }
}

impl FromStr for Dinner {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let entries: Vec<Entry> = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        let mut people_set: HashSet<String> = HashSet::new();
        let mut happiness: HashMap<(String, String), Happiness> = HashMap::new();

        for entry in entries {
            people_set.insert(entry.from.clone());
            people_set.insert(entry.to.clone());
            happiness.insert((entry.from, entry.to), entry.delta);
        }

        let mut people: Vec<String> = people_set.into_iter().collect();
        people.sort_unstable();

        Ok(Self { people, happiness })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Entry {
    from: String,
    to: String,
    delta: Happiness,
}

impl FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.trim().trim_end_matches('.');
        let tokens: Vec<&str> = s.split_whitespace().collect();

        match tokens.as_slice() {
            [
                from,
                "would",
                sign,
                amount,
                "happiness",
                "units",
                "by",
                "sitting",
                "next",
                "to",
                to,
            ] => {
                let amount: Happiness = amount.parse()?;
                let delta = match *sign {
                    "gain" => amount,
                    "lose" => -amount,
                    _ => bail!("unknown sign: {sign}"),
                };

                Ok(Self {
                    from: (from).to_string(),
                    to: to.to_string(),
                    delta,
                })
            }
            _ => bail!("invalid entry: {s}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn test_best_happiness_with_me() {
        let mut dinner: Dinner = EXAMPLE.parse().unwrap();
        dinner.add_me();
        assert_eq!(dinner.best_happiness(), Some(286));
    }

    #[test]
    fn test_best_happiness() {
        let dinner: Dinner = EXAMPLE.parse().unwrap();
        assert_eq!(dinner.best_happiness(), Some(330));
    }
}
