#![allow(unused)]

use std::{collections::HashMap, str::FromStr};

use anyhow::{Context, Result};

pub fn solve_part_1(input: &str) -> Result<usize> {
    let map: OrbitMap = input.parse()?;
    let result = map.total_orbits();
    Ok(result)
}
pub fn solve_part_2(input: &str) -> Result<usize> {
    let map: OrbitMap = input.parse()?;
    map.orbital_transfer(&Object("YOU".into()), &Object("SAN".into()))
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Object(String);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Orbit {
    parent: Object,
    child: Object,
}

impl FromStr for Orbit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (parent, child) = s
            .trim()
            .split_once(')')
            .with_context(|| format!("invalid orbit line: {s:?}"))?;

        Ok(Self {
            parent: Object(parent.to_owned()),
            child: Object(child.to_owned()),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct OrbitMap {
    parents: HashMap<Object, Object>,
}

impl OrbitMap {
    fn orbital_transfer(&self, from: &Object, to: &Object) -> Result<usize> {
        let from_path = self.path_to_com(from)?;
        let to_path = self.path_to_com(to)?;

        let from_steps: HashMap<&Object, usize> = from_path
            .iter()
            .enumerate()
            .map(|(i, object)| (object, i))
            .collect();

        let transfers = to_path
            .iter()
            .enumerate()
            .find_map(|(j, object)| from_steps.get(object).map(|i| i + j))
            .context("no common ancestor found")?;

        Ok(transfers)
    }

    fn path_to_com(&self, start: &Object) -> Result<Vec<Object>> {
        let mut path = Vec::new();

        let mut current = self
            .parents
            .get(start)
            .with_context(|| format!("{start:?} has no parent"))?;

        path.push(current.clone());

        while let Some(parent) = self.parents.get(current) {
            path.push(parent.clone());
            current = parent;
        }

        Ok(path)
    }

    fn total_orbits(&self) -> usize {
        self.parents.keys().map(|object| self.depth(object)).sum()
    }

    fn depth(&self, object: &Object) -> usize {
        let mut count = 0;
        let mut current = object;

        while let Some(parent) = self.parents.get(current) {
            count += 1;
            current = parent;
        }

        count
    }
}

impl FromStr for OrbitMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let orbits: Vec<Orbit> = s
            .lines()
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        let parents: HashMap<Object, Object> = orbits
            .into_iter()
            .map(|orbit| (orbit.child, orbit.parent))
            .collect();

        Ok(Self { parents })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

    const EXAMPLE_2: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

    #[test]
    fn test_part_1() {
        let map: OrbitMap = EXAMPLE.parse().unwrap();
        assert_eq!(map.total_orbits(), 42);
    }

    #[test]
    fn test_part_2() {
        let map: OrbitMap = EXAMPLE_2.parse().unwrap();
        assert_eq!(
            map.orbital_transfer(&Object("YOU".into()), &Object("SAN".into()))
                .unwrap(),
            4
        );
    }
}
