use anyhow::Result;
use rayon::prelude::*;
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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct FishSchool {
    fish: Vec<Lanternfish>,
}

impl FishSchool {
    fn pass_days(&mut self, days: usize) {
        for _ in 0..days {
            // dbg!(&self);
            self.pass_day();
        }
    }

    fn pass_day(&mut self) {
        let count = self
            .fish
            // .iter()
            .par_iter()
            .filter(|&fish| fish.internal_timer == 0)
            .count();
        // need to parallelize this?
        // for fish in self.fish.iter_mut() {
        //     fish.pass_day();
        // }
        self.fish.par_iter_mut().for_each(|f| f.pass_day());
        for _ in 0..count {
            // dbg!(&self.fish);
            self.fish.push(Lanternfish { internal_timer: 8 });
        }
    }

    fn count(&self) -> usize {
        self.fish.len()
    }
}

impl FromStr for FishSchool {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fish = s
            .trim()
            .split(',')
            // .map(|d| dbg!(d))
            .map(|d| d.parse::<u8>())
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(|d| Lanternfish { internal_timer: *d })
            .collect::<Vec<_>>();
        Ok(FishSchool { fish })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Lanternfish {
    internal_timer: u8,
}

impl Lanternfish {
    fn pass_day(&mut self) {
        if self.internal_timer == 0 {
            self.internal_timer = 6;
        } else {
            self.internal_timer -= 1;
        }
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
            fish: vec![
                Lanternfish { internal_timer: 3 },
                Lanternfish { internal_timer: 4 },
                Lanternfish { internal_timer: 3 },
                Lanternfish { internal_timer: 1 },
                Lanternfish { internal_timer: 2 },
            ],
        };

        assert_eq!(result, expected);
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
