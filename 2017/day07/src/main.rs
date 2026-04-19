use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{Context, Result};
use derive_more::{Add, AsRef, Display, From, FromStr, Sub};

fn main() -> Result<()> {
    let tower: Tower = std::fs::read_to_string("input.txt")?.parse()?;

    let sol1 = tower.root().context("no root found")?;
    println!("Part 1: {sol1}");

    let sol2 = tower
        .unbalanced_corrected()
        .context("no unbalanced nodes found")?;
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Tower {
    programs: HashMap<ProgramName, ProgramSpec>,
}

impl Tower {
    fn root(&self) -> Option<&ProgramName> {
        let children: HashSet<&ProgramName> = self
            .programs
            .values()
            .flat_map(|p| p.children.iter())
            .collect();

        self.programs.keys().find(|name| !children.contains(name))
    }

    fn total_weight(&self, program: &ProgramName) -> Option<Weight> {
        let program = self.programs.get(program)?;
        let children_total = program.children.iter().try_fold(Weight(0), |acc, child| {
            Some(acc + self.total_weight(child)?)
        })?;

        Some(program.weight + children_total)
    }

    fn unbalanced_corrected(&self) -> Option<Weight> {
        let root = self.root()?;
        self.find_imbalance_from(root)
    }

    fn find_imbalance_from(&self, name: &ProgramName) -> Option<Weight> {
        let program = self.programs.get(name)?;

        for child in &program.children {
            if let Some(answer) = self.find_imbalance_from(child) {
                return Some(answer);
            }
        }

        if program.children.is_empty() {
            return None;
        }

        let child_weights: Vec<(&ProgramName, Weight)> = program
            .children
            .iter()
            .map(|child| Some((child, self.total_weight(child)?)))
            .collect::<Option<_>>()?;

        let first = child_weights.first()?.1;

        if child_weights.iter().all(|(_, weight)| *weight == first) {
            return None;
        }

        let mut counts: HashMap<Weight, usize> = HashMap::new();
        for (_, weight) in &child_weights {
            *counts.entry(*weight).or_insert(0) += 1;
        }

        let correct_weight = counts
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(weight, _)| *weight)?;

        let wrong_weight = counts
            .iter()
            .min_by_key(|(_, count)| *count)
            .map(|(weight, _)| *weight)?;

        let bad_child = child_weights
            .iter()
            .find(|(_, weight)| *weight == wrong_weight)
            .map(|(child, _)| *child)?;

        let bad_program = self.programs.get(bad_child)?;

        let delta = correct_weight.0 as i64 - wrong_weight.0 as i64;
        let corrected: u32 = (i64::from(bad_program.weight.0) + delta).try_into().ok()?;

        Some(corrected.into())
    }
}

impl FromStr for Tower {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut programs = HashMap::with_capacity(s.lines().count());

        for line in s.lines().map(str::trim).filter(|line| !line.is_empty()) {
            let mut iter = line.split_whitespace();

            let name: ProgramName = iter
                .next()
                .expect("empty lines were filtered")
                .to_owned()
                .into();

            let weight: Weight = iter
                .next()
                .context("entry missing weight")?
                .trim_matches(['(', ')'])
                .to_owned()
                .parse()?;

            let children: Vec<ProgramName> = if iter.next().is_some_and(|s| s == "->") {
                iter.map(|s| s.trim_end_matches(',').to_owned().into())
                    .collect()
            } else {
                Vec::new()
            };

            programs.insert(name, ProgramSpec { weight, children });
        }

        Ok(Self { programs })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ProgramSpec {
    weight: Weight,
    children: Vec<ProgramName>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, From, AsRef, Display)]
struct ProgramName(String);

#[derive(Copy, Clone, Debug, PartialEq, Eq, From, FromStr, Add, Hash, Sub, Display)]
struct Weight(u32);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    #[test]
    fn test_corrected_weight() {
        let tower: Tower = EXAMPLE.parse().unwrap();
        assert_eq!(tower.unbalanced_corrected().unwrap(), 60.into());
    }

    #[test]
    fn test_total_weight() {
        let tower: Tower = EXAMPLE.parse().unwrap();
        assert_eq!(
            tower.total_weight(&"ugml".to_owned().into()).unwrap(),
            251.into()
        );
    }

    #[test]
    fn test_tower_root() {
        let result: Tower = EXAMPLE.parse().unwrap();
        assert_eq!(result.root().unwrap().as_ref(), "tknk");
    }
}
