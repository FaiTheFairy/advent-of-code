#![allow(unused)]

use std::str::FromStr;

use anyhow::{Result, anyhow};

pub fn solve_part_1(input: &str) -> Result<usize> {
    let range: PasswordRange = input.parse()?;
    let sol = (range.start..=range.end)
        .map(Password)
        .filter(Password::meets_criteria)
        .count();

    Ok(sol)
}
pub fn solve_part_2(input: &str) -> Result<usize> {
    let range: PasswordRange = input.parse()?;
    let sol = (range.start..=range.end)
        .map(Password)
        .filter(Password::meets_criteria_v2)
        .count();

    Ok(sol)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Password(usize);

impl Password {
    fn meets_criteria_v2(&self) -> bool {
        self.has_adjacent_equal_v2() && self.is_monotonically_increasing()
    }

    fn meets_criteria(&self) -> bool {
        self.has_adjacent_equal() && self.is_monotonically_increasing()
    }

    fn has_adjacent_equal_v2(&self) -> bool {
        let digits = self.as_digits();
        let mut run_len = 1;

        for [a, b] in digits.array_windows() {
            if a == b {
                run_len += 1;
            } else {
                if run_len == 2 {
                    return true;
                }
                run_len = 1;
            }
        }

        run_len == 2
        // let digits = self.as_digits();
        // if digits[0] == digits[1] && digits[1] != digits[2] {
        //     return true;
        // }

        // if digits[digits.len() - 3] != digits[digits.len() - 2]
        //     && digits[digits.len() - 2] == digits[digits.len() - 1]
        // {
        //     return true;
        // }

        // for [a, b, c, d] in digits.array_windows() {
        //     if b == c && a != b && c != d {
        //         return true;
        //     }
        // }
        // false
    }

    fn has_adjacent_equal(&self) -> bool {
        for [a, b] in self.as_digits().array_windows() {
            if a == b {
                return true;
            }
        }
        false
    }

    fn is_monotonically_increasing(&self) -> bool {
        for [a, b] in self.as_digits().array_windows() {
            if b < a {
                return false;
            }
        }
        true
    }

    fn as_digits(&self) -> Vec<u8> {
        self.0
            .to_string()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|c| c as u8)
            .collect()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct PasswordRange {
    start: usize,
    end: usize,
}

impl FromStr for PasswordRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (start, end) = s
            .trim()
            .split_once('-')
            .ok_or(anyhow!("password range missing '-'"))?;

        let start = start.parse()?;
        let end = end.parse()?;

        Ok(Self { start, end })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "235741-706948";

    #[test]
    fn test_meets_criteria_v2() {
        assert!(!Password(111111).meets_criteria_v2());
        assert!(!Password(223450).meets_criteria_v2());
        assert!(!Password(123789).meets_criteria_v2());
        assert!(!Password(123444).meets_criteria_v2());
        assert!(Password(111122).meets_criteria_v2());
    }

    #[test]
    fn test_meets_criteria() {
        assert!(Password(111111).meets_criteria());
        assert!(!Password(223450).meets_criteria());
        assert!(!Password(123789).meets_criteria());
    }

    #[test]
    fn test_parse_password_range() {
        let result: PasswordRange = EXAMPLE.parse().unwrap();
        let expected = PasswordRange {
            start: 235741,
            end: 706948,
        };
        assert_eq!(result, expected);
    }
}
