#![allow(unused)]

use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;

pub fn solve_part_1(input: &str) -> Result<usize> {
    let sol = input.parse::<Adapters>()?.joltage_difference_product();
    Ok(sol)
}

pub fn solve_part_2(input: &str) -> Result<usize> {
    let sol = input.parse::<Adapters>()?.count_valid_arrangements();
    Ok(sol)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Adapters(Vec<usize>);

impl Adapters {
    fn count_valid_arrangements(&self) -> usize {
        let chain = self.sorted_with_endpoints();
        let mut ways = vec![0usize; chain.len()];
        ways[0] = 1;

        for i in 1..chain.len() {
            for j in (0..i).rev() {
                if chain[i] - chain[j] > 3 {
                    break;
                }
                ways[i] += ways[j]
            }
        }

        *ways.last().unwrap()
    }

    fn sorted_with_endpoints(&self) -> Vec<usize> {
        let mut sorted = self.0.clone();
        sorted.sort_unstable();

        let mut out = Vec::with_capacity(sorted.len() + 2);
        out.push(0);
        out.extend(sorted);
        out.push(out.last().unwrap() + 3);
        out
    }

    fn joltage_difference_product(&self) -> usize {
        let chain = self.sorted_with_endpoints();
        let mut ones = 0;
        let mut threes = 0;

        for [a, b] in chain.array_windows() {
            match b - a {
                1 => ones += 1,
                3 => threes += 1,
                _ => (),
            }
        }

        ones * threes
    }
}

impl FromStr for Adapters {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let values = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse::<usize>)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const EXAMPLE_2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_count_valid_arrangements() {
        let result_1 = EXAMPLE_1
            .parse::<Adapters>()
            .unwrap()
            .count_valid_arrangements();
        assert_eq!(result_1, 8);

        let result_2 = EXAMPLE_2
            .parse::<Adapters>()
            .unwrap()
            .count_valid_arrangements();
        assert_eq!(result_2, 19208);
    }

    #[test]
    fn test_joltage_difference() {
        let result_1 = EXAMPLE_1
            .parse::<Adapters>()
            .unwrap()
            .joltage_difference_product();
        assert_eq!(result_1, 35);

        let result_2 = EXAMPLE_2
            .parse::<Adapters>()
            .unwrap()
            .joltage_difference_product();
        assert_eq!(result_2, 220);
    }
}
