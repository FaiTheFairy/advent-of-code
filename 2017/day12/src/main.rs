use std::{
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input: Network = fs::read_to_string("input.txt")?.parse()?;

    let sol1 = input.component(Id(0)).len();
    println!("Part 1: {sol1}");

    let sol2 = input.group_count();
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Id(usize);

impl From<usize> for Id {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl FromStr for Id {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Network(HashMap<Id, Vec<Id>>);

impl Network {
    /// Returns the set of noeds where every node can reach every other node.
    fn component(&self, start: Id) -> HashSet<Id> {
        let mut visited = HashSet::new();
        let mut stack = vec![start];

        while let Some(current) = stack.pop() {
            if !visited.insert(current) {
                continue;
            }

            if let Some(neighbors) = self.0.get(&current) {
                stack.extend(neighbors.iter().copied());
            }
        }

        visited
    }

    /// Returns a vector of all components.
    fn groups(&self) -> Vec<HashSet<Id>> {
        let mut groups = Vec::new();
        let mut seen = HashSet::new();

        for &id in self.0.keys() {
            if seen.contains(&id) {
                continue;
            }

            let group = self.component(id);
            seen.extend(group.iter().copied());
            groups.push(group);
        }

        groups
    }

    fn group_count(&self) -> usize {
        self.groups().len()
    }
}

impl FromStr for Network {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut map = HashMap::with_capacity(s.lines().count());

        for line in s.lines().map(str::trim).filter(|l| !l.is_empty()) {
            let (id, id_list) = line
                .split_once("<->")
                .context("cannot find '<->' in entry")?;

            let id: Id = id.trim().parse()?;
            let id_list: Vec<Id> = id_list
                .split(',')
                .map(str::trim)
                .map(str::parse)
                .collect::<Result<_, _>>()?;

            map.insert(id, id_list);
        }

        Ok(Self(map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

    #[test]
    fn test_component_size() {
        let network: Network = EXAMPLE.parse().unwrap();
        assert_eq!(network.component(Id(0)).len(), 6);
    }

    #[test]
    fn test_group_count() {
        let network: Network = EXAMPLE.parse().unwrap();
        assert_eq!(network.group_count(), 2);
    }

    #[test]
    fn test_parse_network() {
        let result: Network = EXAMPLE.parse().unwrap();
        let expected = Network(HashMap::from([
            (Id(0), vec![Id(2)]),
            (Id(1), vec![Id(1)]),
            (Id(2), vec![Id(0), Id(3), Id(4)]),
            (Id(3), vec![Id(2), Id(4)]),
            (Id(4), vec![Id(2), Id(3), Id(6)]),
            (Id(5), vec![Id(6)]),
            (Id(6), vec![Id(4), Id(5)]),
        ]));
        assert_eq!(result, expected);
    }
}
