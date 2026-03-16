use std::str::FromStr;

use anyhow::{Result, anyhow};

pub fn solve_part_1(input: &str) -> Result<usize> {
    input
        .parse::<XmasData>()?
        .first_invalid(25)
        .ok_or_else(|| anyhow!("no values satisfy the condition"))
}

pub fn solve_part_2(input: &str) -> Result<usize> {
    let data = input.parse::<XmasData>()?;
    let target = data
        .first_invalid(25)
        .ok_or_else(|| anyhow!("no values satisfy the condition"))?;

    data.encryption_weakness(target)
        .ok_or_else(|| anyhow!("no contingous set of values satisfy the condition"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct XmasData(Vec<usize>);

impl XmasData {
    fn is_valid_at(&self, idx: usize, preamble_len: usize) -> bool {
        let target = self.0[idx];
        let window = &self.0[idx - preamble_len..idx];

        for i in 0..window.len() {
            for j in i + 1..window.len() {
                if window[i] + window[j] == target {
                    return true;
                }
            }
        }

        false
    }

    fn first_invalid(&self, preamble_len: usize) -> Option<usize> {
        (preamble_len..self.0.len())
            .find(|idx| !self.is_valid_at(*idx, preamble_len))
            .map(|idx| self.0[idx])
    }

    fn encryption_weakness(&self, target: usize) -> Option<usize> {
        let mut start = 0;
        let mut sum = 0;

        for end in 0..self.0.len() {
            sum += self.0[end];

            while sum > target {
                sum -= self.0[start];
                start += 1;
            }

            if sum == target && end > start {
                let range = &self.0[start..=end];
                let min = *range.iter().min()?;
                let max = *range.iter().max()?;
                return Some(min + max);
            }
        }

        None
    }
}

impl FromStr for XmasData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let nums: Vec<usize> = s
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self(nums))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";

    #[test]
    fn test_encryption_weakness() {
        let xmas_data: XmasData = EXAMPLE.parse().unwrap();
        let first_invalid = xmas_data.first_invalid(5).unwrap();
        let result = xmas_data.encryption_weakness(first_invalid).unwrap();
        assert_eq!(result, 62);
    }

    #[test]
    fn test_first_invalid() {
        let result = EXAMPLE
            .parse::<XmasData>()
            .unwrap()
            .first_invalid(5)
            .unwrap();
        assert_eq!(result, 127);
    }

    #[test]
    fn test_parse_xmas_data() {
        let result: XmasData = EXAMPLE.parse().unwrap();
        let expected = XmasData(vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ]);
        assert_eq!(result, expected);
    }
}
