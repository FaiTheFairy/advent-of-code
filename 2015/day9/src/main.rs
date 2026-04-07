use std::{collections::HashMap, fs, str::FromStr};

use anyhow::{Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input: Input = fs::read_to_string("input.txt")?.parse()?;

    let sol1: u32 = input.shortest_route().context("no route found")?;
    println!("Part 1: {sol1}");

    let sol2: u32 = input.longest_route().context("no route found")?;
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Input {
    city_names: Vec<String>,
    distances: Vec<Vec<u32>>,
}

impl Input {
    fn shortest_route(&self) -> Option<u32> {
        self.route_lengths().min()
    }

    fn longest_route(&self) -> Option<u32> {
        self.route_lengths().max()
    }

    fn route_lengths(&self) -> impl Iterator<Item = u32> + '_ {
        (0..self.city_names.len())
            .permutations(self.city_names.len())
            .filter_map(|route: Vec<usize>| self.route_length(&route))
    }

    fn route_length(&self, route: &[usize]) -> Option<u32> {
        route
            .iter()
            .copied()
            .tuple_windows::<(usize, usize)>()
            .map(|(a, b)| self.distance(a, b))
            .try_fold(0u32, |acc: u32, edge: Option<u32>| {
                edge.map(|weight: u32| acc + weight)
            })
    }

    fn distance(&self, a: usize, b: usize) -> Option<u32> {
        self.distances
            .get(a)
            .and_then(|row: &Vec<u32>| row.get(b))
            .copied()
            .filter(|&d: &u32| d != 0)
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let edges: Vec<Edge> = s
            .lines()
            .map(str::trim)
            .filter(|line: &&str| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        let mut city_to_index: HashMap<String, usize> = HashMap::new();

        for edge in &edges {
            let next_a: usize = city_to_index.len();
            city_to_index.entry(edge.from.clone()).or_insert(next_a);

            let next_b: usize = city_to_index.len();
            city_to_index.entry(edge.to.clone()).or_insert(next_b);
        }

        let mut city_names: Vec<String> = vec![String::new(); city_to_index.len()];
        for (name, index) in &city_to_index {
            city_names[*index] = name.clone();
        }

        let mut distances: Vec<Vec<u32>> = vec![vec![0; city_names.len()]; city_names.len()];

        for edge in edges {
            let a: usize = *city_to_index
                .get(&edge.from)
                .context("missing city index for source")?;
            let b: usize = *city_to_index
                .get(&edge.to)
                .context("missing city index for destination")?;

            distances[a][b] = edge.distance;
            distances[b][a] = edge.distance;
        }

        Ok(Self {
            city_names,
            distances,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Edge {
    from: String,
    to: String,
    distance: u32,
}

impl FromStr for Edge {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s.split_once(" = ").context("missing '='")?;
        let distance: u32 = rhs.trim().parse().context("invalid distance")?;

        let (from, to) = lhs.split_once(" to ").context("missing 'to'")?;

        Ok(Self {
            from: from.trim().to_string(),
            to: to.trim().to_string(),
            distance,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
";

    #[test]
    fn test_parse_edge() {
        let result: Edge = "London to Dublin = 464".parse().unwrap();

        let expected = Edge {
            from: "London".to_string(),
            to: "Dublin".to_string(),
            distance: 464,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_shortest_route_example() {
        let input: Input = EXAMPLE.parse().unwrap();
        assert_eq!(input.shortest_route(), Some(605));
    }

    #[test]
    fn test_longest_route_example() {
        let input: Input = EXAMPLE.parse().unwrap();
        assert_eq!(input.longest_route(), Some(982));
    }
}
