use std::fs;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use anyhow::ensure;

type Weight = u64;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct QuantumEntanglement(u64);

impl QuantumEntanglement {
    fn one() -> Self {
        Self(1)
    }

    fn get(self) -> u64 {
        self.0
    }
}

impl std::ops::Mul<Weight> for QuantumEntanglement {
    type Output = Self;

    fn mul(self, rhs: Weight) -> Self::Output {
        Self(self.0 * rhs)
    }
}

fn main() -> Result<()> {
    let input: String =
        fs::read_to_string(Path::new("input.txt")).context("failed to read input.txt")?;

    let mut weights: Vec<Weight> = parse_input(&input)?;
    weights.sort_unstable_by(|a, b| b.cmp(a));

    let part_1: QuantumEntanglement =
        best_qe(&weights, 3)?.context("no valid 3-group partition found")?;
    let part_2: QuantumEntanglement =
        best_qe(&weights, 4)?.context("no valid 4-group partition found")?;

    println!("part 1: {}", part_1.get());
    println!("part 2: {}", part_2.get());

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<Weight>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.trim()
                .parse::<Weight>()
                .with_context(|| format!("failed to parse weight: {line:?}"))
        })
        .collect()
}

fn best_qe(weights: &[Weight], group_count: usize) -> Result<Option<QuantumEntanglement>> {
    ensure!(group_count >= 2, "group_count must be at least 2");

    let total: Weight = weights.iter().copied().sum();
    ensure!(
        total.is_multiple_of(group_count as Weight),
        "total weight {total} is not divisible by {group_count}"
    );

    let target: Weight = total / group_count as Weight;

    for first_group_size in 1..=weights.len() {
        let mut best: Option<QuantumEntanglement> = None;
        let mut chosen: Vec<usize> = Vec::with_capacity(first_group_size);

        search_first_group(
            weights,
            group_count,
            target,
            first_group_size,
            0,
            0,
            QuantumEntanglement::one(),
            &mut chosen,
            &mut best,
        );

        if best.is_some() {
            return Ok(best);
        }
    }

    Ok(None)
}

#[allow(clippy::too_many_arguments)]
fn search_first_group(
    weights: &[Weight],
    group_count: usize,
    target: Weight,
    target_size: usize,
    start: usize,
    sum: Weight,
    qe: QuantumEntanglement,
    chosen: &mut Vec<usize>,
    best: &mut Option<QuantumEntanglement>,
) {
    if chosen.len() == target_size {
        if sum != target {
            return;
        }

        if let Some(current_best) = *best
            && qe >= current_best
        {
            return;
        }

        let remaining: Vec<Weight> = remove_indices(weights, chosen);

        if can_partition(&remaining, group_count - 1, target) {
            match best {
                Some(current_best) if qe >= *current_best => {}
                _ => *best = Some(qe),
            }
        }

        return;
    }

    if start >= weights.len() {
        return;
    }

    let slots_left: usize = target_size - chosen.len();
    if weights.len() - start < slots_left {
        return;
    }

    let mut previous_at_depth: Option<Weight> = None;

    for i in start..weights.len() {
        let weight: Weight = weights[i];

        if previous_at_depth == Some(weight) {
            continue;
        }

        let next_sum: Weight = sum + weight;
        if next_sum > target {
            continue;
        }

        let next_qe: QuantumEntanglement = qe * weight;
        if let Some(current_best) = *best
            && next_qe >= current_best
        {
            continue;
        }

        chosen.push(i);
        search_first_group(
            weights,
            group_count,
            target,
            target_size,
            i + 1,
            next_sum,
            next_qe,
            chosen,
            best,
        );
        chosen.pop();

        previous_at_depth = Some(weight);
    }
}

fn can_partition(weights: &[Weight], groups_left: usize, target: Weight) -> bool {
    if groups_left == 1 {
        return weights.iter().copied().sum::<Weight>() == target;
    }

    let total: Weight = weights.iter().copied().sum();
    if total != groups_left as Weight * target {
        return false;
    }

    let mut chosen: Vec<usize> = Vec::new();
    search_partition_subset(weights, groups_left, target, 0, 0, &mut chosen)
}

fn search_partition_subset(
    weights: &[Weight],
    groups_left: usize,
    target: Weight,
    start: usize,
    sum: Weight,
    chosen: &mut Vec<usize>,
) -> bool {
    if sum == target {
        let remaining: Vec<Weight> = remove_indices(weights, chosen);
        return can_partition(&remaining, groups_left - 1, target);
    }

    let mut previous_at_depth: Option<Weight> = None;

    for i in start..weights.len() {
        let weight: Weight = weights[i];

        if previous_at_depth == Some(weight) {
            continue;
        }

        let next_sum: Weight = sum + weight;
        if next_sum > target {
            continue;
        }

        chosen.push(i);
        if search_partition_subset(weights, groups_left, target, i + 1, next_sum, chosen) {
            return true;
        }
        chosen.pop();

        previous_at_depth = Some(weight);
    }

    false
}

fn remove_indices(weights: &[Weight], chosen: &[usize]) -> Vec<Weight> {
    let mut used: Vec<bool> = vec![false; weights.len()];

    for &index in chosen {
        used[index] = true;
    }

    weights
        .iter()
        .enumerate()
        .filter_map(|(index, &weight)| (!used[index]).then_some(weight))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
1
2
3
4
5
7
8
9
10
11
";

    #[test]
    fn test_example_part_1() {
        let mut weights: Vec<Weight> = parse_input(EXAMPLE).unwrap();
        weights.sort_unstable_by(|a, b| b.cmp(a));
        assert_eq!(best_qe(&weights, 3).unwrap(), Some(QuantumEntanglement(99)));
    }

    #[test]
    fn test_example_part_2() {
        let mut weights: Vec<Weight> = parse_input(EXAMPLE).unwrap();
        weights.sort_unstable_by(|a, b| b.cmp(a));
        assert_eq!(best_qe(&weights, 4).unwrap(), Some(QuantumEntanglement(44)));
    }
}
