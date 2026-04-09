use std::{
    hash::{DefaultHasher, Hash, Hasher},
    str::FromStr,
};

use anyhow::{Context, Result, bail};

fn main() -> Result<()> {
    let olympics: Olympics = std::fs::read_to_string("input.txt")?.parse()?;
    let race_time = 2503;

    let sol1 = olympics.solve_part_1(race_time)?;
    println!("Part 1: {sol1}");

    let sol2 = olympics
        .solve_part_2(race_time)
        .context("no solution found")?;
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Olympics {
    reindeers: Vec<Reindeer>,
}

impl Olympics {
    fn solve_part_1(&self, race_time: u32) -> Result<u32> {
        self.reindeers
            .iter()
            .map(|r| r.distance_after(race_time))
            .max()
            .context("no reindeers found")
    }

    fn solve_part_2(&self, race_time: u32) -> Option<u32> {
        let mut scores = vec![0u32; self.reindeers.len()];

        for duration in 1..=race_time {
            let distances: Vec<u32> = self
                .reindeers
                .iter()
                .map(|r| r.distance_after(duration))
                .collect();

            let lead: u32 = distances.iter().copied().max()?;

            for (idx, distance) in distances.into_iter().enumerate() {
                if distance == lead {
                    scores[idx] += 1;
                }
            }
        }

        scores.into_iter().max()
    }
}

impl FromStr for Olympics {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let reindeers: Vec<Reindeer> = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self { reindeers })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Reindeer {
    id: u64,
    speed: u32,
    flight_duration: u32,
    rest_duration: u32,
    points: u32,
}

impl Reindeer {
    fn cycle_time(&self) -> u32 {
        self.flight_duration + self.rest_duration
    }

    fn cycle_distance(&self) -> u32 {
        self.speed * self.flight_duration
    }

    fn distance_after(&self, duration: u32) -> u32 {
        let cycles = duration / self.cycle_time();
        let base_time = cycles * self.cycle_distance();
        let remaining_time = duration % self.cycle_time();
        if remaining_time < self.flight_duration {
            base_time + self.speed * remaining_time
        } else {
            base_time + self.cycle_distance()
        }
    }
}

impl FromStr for Reindeer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.trim().trim_end_matches('.').split_whitespace().collect();

        match tokens.as_slice() {
            [
                name,
                "can",
                "fly",
                speed,
                "km/s",
                "for",
                duration,
                "seconds,",
                "but",
                "then",
                "must",
                "rest",
                "for",
                rest,
                "seconds",
            ] => {
                let mut hasher = DefaultHasher::new();
                name.hash(&mut hasher);
                let id = hasher.finish();
                let speed = speed.parse()?;
                let duration = duration.parse()?;
                let rest = rest.parse()?;
                Ok(Self {
                    id,
                    speed,
                    flight_duration: duration,
                    rest_duration: rest,
                    points: 0,
                })
            }
            _ => bail!("unknown reindeer entry: {s}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMET: &str = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.";
    const DANCER: &str =
        "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    #[test]
    fn test_part_2() {
        let olympics: Olympics = [COMET, DANCER].join("\n").parse().unwrap();
        // (COMET.to_owned() + DANCER).parse().unwrap();
        assert_eq!(olympics.solve_part_2(1000), Some(689));
    }

    #[test]
    fn test_distance_after() {
        let comet: Reindeer = COMET.parse().unwrap();
        let dancer: Reindeer = DANCER.parse().unwrap();

        assert_eq!(comet.distance_after(1), 14);
        assert_eq!(dancer.distance_after(1), 16);

        assert_eq!(comet.distance_after(10), 140);
        assert_eq!(dancer.distance_after(10), 160);

        assert_eq!(comet.distance_after(1000), 1120);
        assert_eq!(dancer.distance_after(1000), 1056);
    }

    #[test]
    fn test_parse_reindeer() {
        let result: Reindeer =
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."
                .parse()
                .unwrap();

        let name = "Comet";
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        let id = hasher.finish();
        let expected = Reindeer {
            id,
            speed: 14,
            flight_duration: 10,
            rest_duration: 127,
            points: 0,
        };

        assert_eq!(result, expected);
    }
}
