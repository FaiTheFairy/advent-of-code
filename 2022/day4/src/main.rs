use std::{fs, str::FromStr};

use anyhow::{Result, bail, ensure};

fn main() -> Result<()> {
    // part 1
    let input = fs::read_to_string("./input.txt")?;
    let elf_list = input.parse::<ElfList>()?;
    let sol1 = elf_list.count_one_contains_other();
    println!(
        "Part 1. number of pairs which contain a range that encompasses the other = {}",
        sol1
    );

    let sol2 = elf_list.count_overlapping();
    println!("Part 2. number of pairs that contain overlapping ranges = {sol2}");
    Ok(())
}

impl ElfList {
    fn count_one_contains_other(&self) -> usize {
        self.pairs
            .iter()
            .filter(|p| p.one_contains_the_other())
            .count()
    }

    fn count_overlapping(&self) -> usize {
        self.pairs.iter().filter(|p| p.is_overlapped()).count()
    }
}

#[derive(Debug, Clone, Copy)]
struct Elf {
    start: usize,
    end: usize,
}

impl Elf {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl FromStr for Elf {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once("-") {
            Some((start, end)) => {
                let start = start.parse::<usize>()?;
                let end = end.parse::<usize>()?;
                ensure!(
                    start <= end,
                    "Start must be smaller than end for elf range."
                );
                Ok(Elf::new(start, end))
            }
            None => bail!("String cannot be parsed to Elf '{}'", s),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ElfPair(Elf, Elf);

impl FromStr for ElfPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.split_once(",") {
            Some((elf1, elf2)) => {
                let elf1 = elf1.parse::<Elf>()?;
                let elf2 = elf2.parse::<Elf>()?;
                Ok(Self(elf1, elf2))
            }
            None => bail!("String cannot be parsed to Elf '{}'", s),
        }
    }
}

impl ElfPair {
    fn one_contains_the_other(&self) -> bool {
        self.0.start >= self.1.start && self.0.end <= self.1.end
            || self.0.start <= self.1.start && self.0.end >= self.1.end
    }

    fn is_overlapped(&self) -> bool {
        self.0.start.max(self.1.start) <= self.0.end.min(self.1.end)
        // (self.0.start..=self.0.end).any(|c| c <= self.1.end && c >= self.1.start)
        // let len1 = self.0.end - self.0.start;
        // let len2 = self.1.end - self.1.start;
        // if len1 < len2 {
        //     (self.0.start..self.0.end).any(|n| n >= self.1.start && n <= self.1.end)
        // } else {
        //     (self.1.start..self.1.end).any(|n| n >= self.0.start && n <= self.0.end)
        // }
    }
}

#[derive(Debug, Clone)]
struct ElfList {
    pairs: Vec<ElfPair>,
}

impl FromStr for ElfList {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let pairs = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse::<ElfPair>)
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { pairs })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_solve_part_1() -> Result<()> {
        let list = EXAMPLE.parse::<ElfList>()?;
        let result = list.count_one_contains_other();
        assert_eq!(result, 2usize);
        Ok(())
    }

    #[test]
    fn test_solve_part_2() -> Result<()> {
        let list = EXAMPLE.parse::<ElfList>()?;
        let result = list.count_overlapping();
        assert_eq!(result, 4usize);
        Ok(())
    }
}
