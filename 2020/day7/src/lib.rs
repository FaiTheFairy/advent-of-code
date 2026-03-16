use std::{collections::HashMap, str::FromStr};

use anyhow::{Result, anyhow};

/// Returns the number of bag colors that can eventually contain
/// at least one shiny gold bag.
///
/// # Examples
///
/// ```
/// use day7::solve_part_1;
///
/// let input =
/// "light red bags contain 1 bright white bag, 2 muted yellow bags.
/// bright white bags contain 1 shiny gold bag.
/// muted yellow bags contain no other bags.
/// shiny gold bags contain no other bags.";
///
/// assert_eq!(solve_part_1(input).unwrap(), 2);
/// ```
pub fn solve_part_1(input: &str) -> Result<usize> {
    let rules: Rules = input.parse()?;

    Ok(rules.count_possible_outer_bags(&shiny_gold()))
}

pub fn solve_part_2(input: &str) -> Result<usize> {
    let rules = input.parse::<Rules>()?;
    Ok(rules.count_total_bags_inside(&shiny_gold()))
}

type Contents = Vec<(usize, Bag)>;

/// Bag containment rules keyed by outer bag color.
///
/// Each map entry stores the bags directly contained inside that outer bag,
/// as `(count, inner_bag)` pairs.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Rules(HashMap<Bag, Contents>);

impl Rules {
    fn count_total_bags_inside(&self, outer: &Bag) -> usize {
        self.0
            .get(outer)
            .map(|inners| {
                inners
                    .iter()
                    .map(|(count, bag)| count * (1 + self.count_total_bags_inside(bag)))
                    .sum()
            })
            .unwrap_or(0)
    }

    fn can_eventually_contain(&self, outer: &Bag, target: &Bag) -> bool {
        self.0.get(outer).is_some_and(|inners| {
            inners
                .iter()
                .any(|(_, inner)| inner == target || self.can_eventually_contain(inner, target))
        })
    }

    fn count_possible_outer_bags(&self, target: &Bag) -> usize {
        self.0
            .keys()
            .filter(|outer| self.can_eventually_contain(outer, target))
            .count()
    }

    fn parse_rule_line(line: &str) -> Result<(Bag, Contents)> {
        let (outer, inners) = line
            .split_once(" bags contain ")
            .ok_or_else(|| anyhow!("malformed input: {line}"))?;

        let inners = if inners == "no other bags." {
            Vec::new()
        } else {
            inners
                .split(',')
                .map(Rules::parse_inner_bag)
                .collect::<Result<_, _>>()?
        };

        Ok((Bag::new(outer), inners))
    }

    fn parse_inner_bag(s: &str) -> Result<(usize, Bag)> {
        let s = s.trim().trim_end_matches('.');
        let s = s.trim_end_matches(" bags").trim_end_matches(" bag");

        let (count, color) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("malformed inner bag: {s}"))?;

        Ok((count.parse::<usize>()?, Bag::new(color)))
    }
}

impl FromStr for Rules {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let rules = s
            .lines()
            .map(Self::parse_rule_line)
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(Self(rules))
    }
}

/// A bag identified by its color.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bag(String);

impl Bag {
    fn new(color: impl Into<String>) -> Self {
        Self(color.into())
    }
}

fn shiny_gold() -> Bag {
    Bag::new("shiny gold")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const EXAMPLE_SHORT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
faded blue bags contain no other bags.";

    const EXAMPLE_PART_2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn test_count_total_bags_inside_example_2() {
        let rules = EXAMPLE_PART_2.parse::<Rules>().unwrap();
        let result = rules.count_total_bags_inside(&shiny_gold());
        assert_eq!(result, 126);
    }

    #[test]
    fn test_count_total_bags_inside_example() {
        let rules = EXAMPLE.parse::<Rules>().unwrap();
        let result = rules.count_total_bags_inside(&Bag::new("shiny gold"));
        assert_eq!(result, 32)
    }

    #[test]
    fn test_count_can_eventually_contain() {
        let rules = EXAMPLE.parse::<Rules>().unwrap();
        let result = rules.count_possible_outer_bags(&shiny_gold());
        assert_eq!(result, 4);
    }

    #[test]
    fn test_can_eventually_contain() {
        let rules = EXAMPLE.parse::<Rules>().unwrap();
        let target = shiny_gold();
        assert!(rules.can_eventually_contain(&Bag::new("bright white"), &target));
        assert!(rules.can_eventually_contain(&Bag::new("muted yellow"), &target));
        assert!(rules.can_eventually_contain(&Bag::new("dark orange"), &target));
        assert!(rules.can_eventually_contain(&Bag::new("light red"), &target));

        assert!(!rules.can_eventually_contain(&Bag::new("dark olive"), &target));
        assert!(!rules.can_eventually_contain(&Bag::new("faded blue"), &target));
    }

    #[test]
    fn test_parse_rules() {
        let result = EXAMPLE_SHORT.parse::<Rules>().unwrap();
        let expected = Rules(HashMap::from([
            (
                Bag::new("light red"),
                vec![(1, Bag::new("bright white")), (2, Bag::new("muted yellow"))],
            ),
            (
                Bag::new("dark orange"),
                vec![(3, Bag::new("bright white")), (4, Bag::new("muted yellow"))],
            ),
            (Bag::new("bright white"), vec![(1, shiny_gold())]),
            (Bag::new("faded blue"), vec![]),
        ]));
        assert_eq!(result, expected);
    }
}
