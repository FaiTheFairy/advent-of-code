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
    timers: [usize; 9],
}

impl FishSchool {
    fn pass_days(&mut self, days: usize) {
        for _ in 0..days {
            self.pass_day();
        }
    }

    fn pass_day(&mut self) {
        let zeros = self.timers[0];
        // This rotation means that all zeros producea fish at idx 8.
        // so we only need to reset them to six afterwards.
        self.timers.rotate_left(1);
        self.timers[6] += zeros
    }

    fn count(&self) -> usize {
        self.timers.iter().sum()
    }
}

impl FromStr for FishSchool {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = Self::default();

        for timer in s.trim().split(',').map(str::parse::<usize>) {
            let timer = timer?;
            anyhow::ensure!(timer <= 8, "invalid fish timer: {timer}");
            out.timers[timer] += 1;
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
            timers: [0, 1, 1, 2, 1, 0, 0, 0, 0],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_pass_day() {
        let mut school = EXAMPLE.parse::<FishSchool>().unwrap();
        school.pass_day();
        let expected_day1 = FishSchool {
            timers: [1, 1, 2, 1, 0, 0, 0, 0, 0],
        };
        assert_eq!(school, expected_day1);

        school.pass_day();
        let expected_day2 = FishSchool {
            timers: [1, 2, 1, 0, 0, 0, 1, 0, 1],
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

    #[test]
    fn test_part_2() {
        let mut school = EXAMPLE.parse::<FishSchool>().unwrap();
        school.pass_days(256);
        assert_eq!(school.count(), 26984457539);
    }
}
