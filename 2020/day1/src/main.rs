use anyhow::{Result, anyhow};
use std::{fs, str::FromStr};

fn main() -> Result<()> {
    let input = &fs::read_to_string("input.txt")?.parse::<ExpenseReport>()?;
    let sol1 = input.solve_part_1()?;
    println!("Part 1. the product of the first two entries that sum up to 2020 = {sol1}");

    let sol2 = input.solve_part_2()?;
    println!("Part 2. the product of the first three entries that sum up to 2020 = {sol2}");

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExpenseReport(Vec<usize>);

impl ExpenseReport {
    fn solve_part_1(&self) -> Result<usize> {
        for (idx, i) in self.0.iter().enumerate() {
            for j in self.0.iter().skip(idx) {
                if i + j == 2020 {
                    return Ok(i * j);
                }
            }
        }

        Err(anyhow!("No solution found"))
    }

    fn solve_part_2(&self) -> Result<usize> {
        for (idx1, i) in self.0.iter().enumerate() {
            for (idx2, j) in self.0.iter().skip(idx1).enumerate() {
                for k in self.0.iter().skip(idx1 + idx2) {
                    if i + j + k == 2020 {
                        return Ok(i * j * k);
                    }
                }
            }
        }

        Err(anyhow!("No solution found"))
    }
}

impl FromStr for ExpenseReport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries = s
            .lines()
            .map(|l| l.parse::<usize>())
            .collect::<Result<_, _>>()?;

        Ok(Self(entries))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1721
979
366
299
675
1456";

    #[test]
    fn test_solve_part_2() {
        let result = EXAMPLE
            .parse::<ExpenseReport>()
            .unwrap()
            .solve_part_2()
            .unwrap();
        let expected = 241861950;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_part_1() {
        let result = EXAMPLE
            .parse::<ExpenseReport>()
            .unwrap()
            .solve_part_1()
            .unwrap();
        let expected = 514579;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_expense_report() {
        let result = EXAMPLE.parse::<ExpenseReport>().unwrap();
        let expected = ExpenseReport(vec![1721, 979, 366, 299, 675, 1456]);
        assert_eq!(result, expected);
    }
}
