use anyhow::Result;
use std::{fs, str::FromStr};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let mut school = input.parse::<FishSchool>()?;

    // part 1 (after 80 days)
    school.pass_days(80);
    let count80 = school.count();
    println!("Part 1. after 80 days, a total of {count80} lanternfish are in the school.");

    // part 2 (after 256 days)
    school.pass_days(256 - 80);
    let count256 = school.count();
    println!("Part 2. after 256 days, a total of {count256} lanternfish are in the school.");

    Ok(())
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct FishSchool {
    count: [usize; 9],
}

impl FishSchool {
    fn pass_days(&mut self, days: usize) {
        for _ in 0..days {
            self.pass_day();
        }
    }

    fn pass_day(&mut self) {
        let count_zero = self.count[0];
        self.count[0] = 0;
        for i in 0..8 {
            self.count[i] += self.count[i + 1];
            self.count[i + 1] = 0;
        }
        self.count[6] += count_zero;
        self.count[8] += count_zero;
    }

    fn count(&self) -> usize {
        self.count.iter().sum()
    }
}

impl FromStr for FishSchool {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = Self::default();
        let fish = s
            .trim()
            .split(',')
            .map(|d| d.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        for i in 0..=8 {
            out.count[i] += fish.iter().filter(|&d| *d == i).count()
        }

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn test_parse_fish_school() {
        let result = EXAMPLE.parse::<FishSchool>().unwrap();
        let expected = FishSchool {
            count: [0, 1, 1, 2, 1, 0, 0, 0, 0],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_pass_day() {
        let mut school = EXAMPLE.parse::<FishSchool>().unwrap();
        school.pass_day();
        let expected_day1 = FishSchool {
            count: [1, 1, 2, 1, 0, 0, 0, 0, 0],
        };
        assert_eq!(school, expected_day1);

        school.pass_day();
        let expected_day2 = FishSchool {
            count: [1, 2, 1, 0, 0, 0, 1, 0, 1],
        };
        assert_eq!(school, expected_day2);
    }

    #[test]
    fn test_part_1() {
        let mut school = EXAMPLE.parse::<FishSchool>().unwrap();

        school.pass_days(18);
        let result18 = school.count();
        assert_eq!(result18, 26);

        school.pass_days(80 - 18);
        let result80 = school.count();
        assert_eq!(result80, 5934);
    }
}
