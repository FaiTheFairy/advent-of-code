#![allow(dead_code)]

use std::{fs, num::ParseIntError, str::FromStr};

use anyhow::{Result, anyhow, bail, ensure};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let worksheet: Worksheet = input.parse()?;
    let sol1 = worksheet.eval_total();
    println!("Part 1: {sol1}");

    let worksheet_rtl = Worksheet::parse_rtl(&input)?;
    let sol2 = worksheet_rtl.eval_total();
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Worksheet(Vec<Problem>);

impl Worksheet {
    fn eval_total(&self) -> usize {
        self.0.iter().map(Problem::eval).sum()
    }

    fn parse_rtl(s: &str) -> Result<Self> {
        let lines: Vec<&str> = s.lines().collect();
        let [line_1, line_2, line_3, line_4, op_line]: [&str; 5] = lines
            .try_into()
            .map_err(|_| anyhow!("expected exactly 5 lines"))?;

        let width = [
            line_1.len(),
            line_2.len(),
            line_3.len(),
            line_4.len(),
            op_line.len(),
        ]
        .into_iter()
        .max()
        .unwrap_or(0);

        let rows: [Vec<char>; 5] = [line_1, line_2, line_3, line_4, op_line].map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.resize(width, ' ');
            chars
        });

        let mut problems = Vec::new();
        let mut col = width;

        while col > 0 {
            while col > 0 && col_is_blank(&rows, col - 1) {
                col -= 1;
            }

            if col == 0 {
                break;
            }

            let end = col;

            while col > 0 && !col_is_blank(&rows, col - 1) {
                col -= 1;
            }

            let start = col;

            let op_str: String = rows[4][start..end]
                .iter()
                .copied()
                .filter(|ch| !ch.is_whitespace())
                .collect();

            let operation: Operation = op_str.parse()?;

            let mut values = Vec::new();

            for c in (start..end).rev() {
                let digits: String = rows[..4]
                    .iter()
                    .map(|row| row[c])
                    .filter(|ch| ch.is_ascii_digit())
                    .collect();

                if !digits.is_empty() {
                    values.push(digits.parse()?);
                }
            }

            ensure!(
                !values.is_empty(),
                "no values found for block {start}..{end}"
            );

            problems.push(Problem { values, operation });
        }

        Ok(Self(problems))
    }
}

fn col_is_blank(rows: &[Vec<char>; 5], col: usize) -> bool {
    rows.iter().all(|row| row[col].is_whitespace())
}

impl FromStr for Worksheet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines: Vec<&str> = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect();

        let [line_1, line_2, line_3, line_4, op_line]: [&str; 5] = lines
            .try_into()
            .map_err(|_| anyhow!("expected exactly 5 lines"))?;

        let row1 = parse_number_row(line_1)?;
        let row2 = parse_number_row(line_2)?;
        let row3 = parse_number_row(line_3)?;
        let row4 = parse_number_row(line_4)?;

        let operations: Vec<Operation> = op_line
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        let len = row1.len();
        ensure!(row2.len() == len, "row 2 length mismatch");
        ensure!(row3.len() == len, "row 3 length mismatch");
        ensure!(row4.len() == len, "row 4 length mismatch");
        ensure!(operations.len() == len, "operator row length mismatch");

        let problems = row1
            .into_iter()
            .zip(row2)
            .zip(row3)
            .zip(row4)
            .zip(operations)
            .map(|((((a, b), c), d), operation)| Problem {
                values: vec![a, b, c, d],
                operation,
            })
            .collect();

        Ok(Self(problems))
    }
}

fn parse_number_row(line: &str) -> Result<Vec<usize>, ParseIntError> {
    line.split_whitespace().map(str::parse).collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Problem {
    values: Vec<usize>,
    operation: Operation,
}

impl Problem {
    fn eval(&self) -> usize {
        match self.operation {
            Operation::Add => self.values.iter().sum(),
            Operation::Multiply => self.values.iter().product(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Operation {
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Multiply),
            _ => bail!("unknown operation: {s}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
  1  2    5  6
*   +   *   +";

    #[test]
    fn test_part_1_example() {
        let worksheet: Worksheet = EXAMPLE.parse().unwrap();
        assert_eq!(worksheet.eval_total(), 21_251_384);
    }

    #[test]
    fn test_part_2_example() {
        let worksheet = Worksheet::parse_rtl(EXAMPLE).unwrap();
        assert_eq!(worksheet.eval_total(), 32_722_226);
    }
}
