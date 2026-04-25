use anyhow::{Context, Result, bail, ensure};
use std::{collections::HashSet, fs, str::FromStr};

fn main() -> Result<()> {
    let input: Input = fs::read_to_string("input.txt")?.parse()?;

    let mut part_1 = input.clone();
    part_1.run_generations(Generation(20));
    println!("Part 1: {}", part_1.plant_sum());

    let mut part_2 = input;
    let sol2 = part_2.plant_sum_after_stabilization(Generation(50_000_000_000));
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Input {
    plant_indices: HashSet<PlantIndex>,
    rules: Rules,
    generation: Generation,
}

impl Input {
    fn run_generations(&mut self, generations: Generation) {
        for _ in 0..generations.0 {
            self.step();
        }
    }

    fn step(&mut self) {
        let min = self.min_plant_index();
        let max = self.max_plant_index();

        let mut next = HashSet::new();

        for idx in (min.0 - 2)..=(max.0 + 2) {
            let idx = PlantIndex(idx);
            let pattern = self.pattern_at(idx);

            if self.rules.produces_plant(pattern) {
                next.insert(idx);
            }
        }

        self.plant_indices = next;
        self.generation.0 += 1;
    }

    fn pattern_at(&self, center: PlantIndex) -> Pattern {
        let mut mask = 0;

        for offset in -2..=2 {
            let idx = PlantIndex(center.0 + offset);
            let bit = usize::from(self.plant_indices.contains(&idx));
            mask = (mask << 1) | bit;
        }

        Pattern(mask)
    }

    fn plant_sum(&self) -> isize {
        self.plant_indices.iter().map(|idx| idx.0).sum()
    }

    fn plant_sum_after_stabilization(&mut self, target: Generation) -> isize {
        let mut previous_sum = self.plant_sum();
        let mut previous_delta = 0;
        let mut stable_count = 0;

        while self.generation < target {
            self.step();

            let sum = self.plant_sum();
            let delta = sum - previous_sum;

            if delta == previous_delta {
                stable_count += 1;
            } else {
                stable_count = 0;
            }

            if stable_count >= 100 {
                let remaining = target.0 - self.generation.0;
                return sum + delta * remaining.cast_signed();
            }

            previous_sum = sum;
            previous_delta = delta;
        }

        self.plant_sum()
    }

    fn min_plant_index(&self) -> PlantIndex {
        self.plant_indices
            .iter()
            .min()
            .copied()
            .unwrap_or(PlantIndex(0))
    }

    fn max_plant_index(&self) -> PlantIndex {
        self.plant_indices
            .iter()
            .max()
            .copied()
            .unwrap_or(PlantIndex(0))
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut sections = s.split("\n\n");

        let initial = sections
            .next()
            .context("missing initial state")?
            .strip_prefix("initial state: ")
            .context("initial state missing prefix")?;

        let plant_indices = parse_initial_state(initial)?;

        let rules = sections.next().context("missing rules")?.parse()?;

        Ok(Self {
            plant_indices,
            rules,
            generation: Generation(0),
        })
    }
}

fn parse_initial_state(s: &str) -> Result<HashSet<PlantIndex>> {
    s.chars()
        .enumerate()
        .filter_map(|(idx, c)| {
            let pot = Pot::try_from(c);

            match pot {
                Ok(Pot::Plant) => Some(Ok(PlantIndex(idx.cast_signed()))),
                Ok(Pot::Empty) => None,
                Err(err) => Some(Err(err)),
            }
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PlantIndex(isize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Generation(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rules([bool; 32]);

impl Rules {
    fn produces_plant(self, pattern: Pattern) -> bool {
        self.0[pattern.0]
    }
}

impl FromStr for Rules {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut rules = [false; 32];

        for line in s.lines().map(str::trim).filter(|line| !line.is_empty()) {
            let rule: Rule = line.parse()?;
            rules[rule.pattern.0] = rule.produces.is_plant();
        }

        Ok(Self(rules))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rule {
    pattern: Pattern,
    produces: Pot,
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (pattern, produces) = s.split_once(" => ").context("rule missing ' => '")?;

        let pattern = pattern.parse()?;

        let mut chars = produces.chars();
        let produces = chars.next().context("empty rule output")?.try_into()?;
        ensure!(
            chars.next().is_none(),
            "rule output must be exactly one char"
        );

        Ok(Self { pattern, produces })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pattern(usize);

impl FromStr for Pattern {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut mask = 0;
        let mut len = 0;

        for c in s.chars() {
            let pot = Pot::try_from(c)?;
            let bit = usize::from(pot.is_plant());

            mask = (mask << 1) | bit;
            len += 1;
        }

        ensure!(len == 5, "pattern must be exactly 5 pots");
        ensure!(mask < 32, "pattern mask out of range");

        Ok(Self(mask))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pot {
    Plant,
    Empty,
}

impl Pot {
    fn is_plant(self) -> bool {
        matches!(self, Self::Plant)
    }
}

impl TryFrom<char> for Pot {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '#' => Ok(Self::Plant),
            '.' => Ok(Self::Empty),
            _ => bail!("unknown pot char: '{value}'"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    #[test]
    fn pattern_parses_to_bitmask() {
        assert_eq!(".....".parse::<Pattern>().unwrap(), Pattern(0));
        assert_eq!("....#".parse::<Pattern>().unwrap(), Pattern(1));
        assert_eq!("...##".parse::<Pattern>().unwrap(), Pattern(3));
        assert_eq!("..#..".parse::<Pattern>().unwrap(), Pattern(4));
        assert_eq!("#####".parse::<Pattern>().unwrap(), Pattern(31));
    }

    #[test]
    fn part_1_example() {
        let mut input: Input = EXAMPLE.parse().unwrap();

        input.run_generations(Generation(20));

        assert_eq!(input.plant_sum(), 325);
    }
}
