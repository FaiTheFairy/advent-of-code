use std::{collections::HashMap, fs, str::FromStr};

use anyhow::{Result, anyhow, bail, ensure};

fn main() -> Result<()> {
    let graph = fs::read_to_string("input.txt")?.parse::<Graph>()?;

    let sol1 = graph.count_paths_part_1();
    println!(
        "Part 1. number of paths through the cave system that visit small caves at most once = {sol1}"
    );

    let sol2 = graph.count_paths_part_2();
    println!(
        "Part 2. number of paths through the cave system allowing one small cave to be visited twice = {sol2}"
    );

    Ok(())
}

type CaveId = usize;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Graph {
    caves: Vec<Cave>,
    edges: Vec<Vec<CaveId>>,
    start: CaveId,
    end: CaveId,
}

impl Graph {
    fn new() -> Self {
        Self::default()
    }

    fn count_paths_part_1(&self) -> usize {
        self.count_paths(true)
    }

    fn count_paths_part_2(&self) -> usize {
        self.count_paths(false)
    }

    fn count_paths(&self, used_double_visit: bool) -> usize {
        let mut visited_small = vec![0u8; self.caves.len()];
        self.count_paths_from(self.start, &mut visited_small, used_double_visit)
    }

    fn count_paths_from(
        &self,
        current: CaveId,
        visited_small: &mut [u8],
        used_double_visit: bool,
    ) -> usize {
        if current == self.end {
            return 1;
        }

        let mut used_double_visit = used_double_visit;

        match self.caves[current].kind {
            CaveKind::Start => {}
            // return early if we're at end
            CaveKind::End => unreachable!("we return early if we reach `End`."),
            CaveKind::Big => {}
            CaveKind::Small => match visited_small[current] {
                0 => {
                    visited_small[current] += 1;
                }
                1 if !used_double_visit => {
                    visited_small[current] += 1;
                    used_double_visit = true;
                }
                1 | 2 => return 0,
                _ => unreachable!("visited_small entries should never be larger than 2"),
            },
        }

        let mut total = 0;

        for &next in &self.edges[current] {
            if self.caves[next].kind == CaveKind::Start {
                continue;
            }
            total += self.count_paths_from(next, visited_small, used_double_visit);
        }

        if self.caves[current].kind == CaveKind::Small {
            visited_small[current] -= 1;
        }

        total
    }

    fn get_or_insert_cave(
        &mut self,
        ids: &mut HashMap<String, CaveId>,
        label: &str,
    ) -> Result<CaveId> {
        if let Some(&id) = ids.get(label) {
            return Ok(id);
        }

        let cave = Cave::try_new(label)?;
        let id = self.caves.len();

        self.caves.push(cave);
        self.edges.push(Vec::new());
        ids.insert(label.to_owned(), id);

        Ok(id)
    }

    fn add_undirected_edge(&mut self, a: CaveId, b: CaveId) {
        self.edges[a].push(b);
        self.edges[b].push(a);
    }
}

impl FromStr for Graph {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut graph = Graph::new();
        let mut ids = HashMap::new();

        for (line_no, line) in s.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let (left, right) = line
                .split_once('-')
                .ok_or_else(|| anyhow!("invalid edge format on line {}", line_no + 1))?;

            let a = graph.get_or_insert_cave(&mut ids, left)?;
            let b = graph.get_or_insert_cave(&mut ids, right)?;

            graph.add_undirected_edge(a, b);
        }

        ensure!(!graph.caves.is_empty(), "graph cannot be empty");

        graph.start = graph
            .caves
            .iter()
            .position(|c| c.kind == CaveKind::Start)
            .ok_or_else(|| anyhow!("missing start cave"))?;

        graph.end = graph
            .caves
            .iter()
            .position(|c| c.kind == CaveKind::End)
            .ok_or_else(|| anyhow!("missing end cave"))?;

        Ok(graph)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cave {
    name: String,
    kind: CaveKind,
}

impl Cave {
    fn try_new(label: &str) -> Result<Self> {
        ensure!(!label.is_empty(), "cave label cannot be empty");
        ensure!(
            label.chars().all(|c| c.is_ascii_alphabetic()),
            "cave labels must be ASCII alphabetic: {label}"
        );

        let kind = match label {
            "start" => CaveKind::Start,
            "end" => CaveKind::End,
            _ if label.chars().all(|c| c.is_ascii_uppercase()) => CaveKind::Big,
            _ if label.chars().all(|c| c.is_ascii_lowercase()) => CaveKind::Small,
            _ => bail!("invalid cave label: {label}"),
        };

        Ok(Self {
            name: label.to_owned(),
            kind,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CaveKind {
    Start,
    End,
    Big,
    Small,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const MEDIUM_EXAMPLE: &str = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    #[test]
    fn graph_parses_small_example() {
        let graph = SMALL_EXAMPLE.parse::<Graph>().unwrap();

        assert_eq!(graph.caves.len(), 6);
        assert_eq!(graph.edges.len(), 6);
        assert_eq!(graph.caves[graph.start].kind, CaveKind::Start);
        assert_eq!(graph.caves[graph.end].kind, CaveKind::End);
    }

    #[test]
    fn counts_paths_for_official_examples() {
        let small = SMALL_EXAMPLE.parse::<Graph>().unwrap();
        assert_eq!(small.count_paths_part_1(), 10);
        assert_eq!(small.count_paths_part_2(), 36);

        let medium = MEDIUM_EXAMPLE.parse::<Graph>().unwrap();
        assert_eq!(medium.count_paths_part_1(), 19);
        assert_eq!(medium.count_paths_part_2(), 103);
    }
}
