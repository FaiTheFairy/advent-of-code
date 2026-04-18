#![allow(dead_code)]

use std::{fs, str::FromStr};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let spreadsheet: Spreadsheet = fs::read_to_string("input.txt")?.parse()?;

    let sol1 = spreadsheet.checksum();
    println!("Part 1: {sol1}");

    let sol2 = spreadsheet
        .sum_division_result()
        .context("no evenly divisible values in spreadsheet")?;
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Spreadsheet(Vec<Row>);

impl Spreadsheet {
    fn sum_division_result(&self) -> Option<u32> {
        self.rows().map(|r| r.division_result()).sum()
    }

    fn checksum(&self) -> u32 {
        self.rows()
            .map(|row| row.iter().max().unwrap_or(0) - row.iter().min().unwrap_or(0))
            .sum()
    }

    fn rows(&self) -> impl Iterator<Item = &Row> {
        self.0.iter()
    }
}

impl FromStr for Spreadsheet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let inner = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self(inner))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Row(Vec<u32>);

impl Row {
    fn iter(&self) -> impl Iterator<Item = u32> {
        self.0.iter().copied()
    }

    fn numbers_that_evenly_divide(&self) -> Option<(u32, u32)> {
        for (i, num_1) in self.iter().enumerate() {
            for num_2 in self.iter().skip(i + 1) {
                if num_1.is_multiple_of(num_2) || num_2.is_multiple_of(num_1) {
                    return Some((num_1.max(num_2), num_1.min(num_2)));
                }
            }
        }

        None
    }

    fn division_result(&self) -> Option<u32> {
        if let Some((a, b)) = self.numbers_that_evenly_divide() {
            Some(a / b)
        } else {
            None
        }
    }
}

impl FromStr for Row {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let inner = s
            .split_whitespace()
            .map(|s| s.parse::<u32>())
            .collect::<Result<_, _>>()?;

        Ok(Self(inner))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "5 1 9 5
7 5 3
2 4 6 8";

    const EXAMPLE_2: &str = "5 9 2 8
9 4 7 3
3 8 6 5";

    #[test]
    fn test_sum_division_result() {
        let result = EXAMPLE_2
            .parse::<Spreadsheet>()
            .unwrap()
            .sum_division_result()
            .unwrap();

        assert_eq!(result, 9)
    }

    #[test]
    fn test_numbers_that_evenly_divide() {
        let spreadsheet: Spreadsheet = EXAMPLE_2.parse().unwrap();
        let mut rows = spreadsheet.rows();

        assert_eq!(
            rows.next().unwrap().numbers_that_evenly_divide(),
            Some((8, 2))
        );
        assert_eq!(
            rows.next().unwrap().numbers_that_evenly_divide(),
            Some((9, 3))
        );
        assert_eq!(
            rows.next().unwrap().numbers_that_evenly_divide(),
            Some((6, 3))
        );
    }

    #[test]
    fn test_checksum() {
        let result = EXAMPLE.parse::<Spreadsheet>().unwrap().checksum();
        assert_eq!(result, 18);
    }

    #[test]
    fn test_parse_spreadsheet() {
        let result: Spreadsheet = EXAMPLE.parse().unwrap();
        let expected = Spreadsheet(vec![
            Row(vec![5, 1, 9, 5]),
            Row(vec![7, 5, 3]),
            Row(vec![2, 4, 6, 8]),
        ]);
        assert_eq!(result, expected);
    }
}
