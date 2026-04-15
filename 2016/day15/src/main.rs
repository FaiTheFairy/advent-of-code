use std::str::FromStr;

use anyhow::{Context, Result, bail};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let machine: Machine = input.parse()?;

    let sol1 = machine.first_press_time();
    println!("Part 1: {sol1}");

    let mut machine2 = machine.clone();
    machine2.disks.push(Disk {
        number: machine2.disks.len() + 1,
        positions: 11,
        start: 0,
    });

    let sol2 = machine2.first_press_time();
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Machine {
    disks: Vec<Disk>,
}

impl Machine {
    fn first_press_time(&self) -> usize {
        let mut time = 0;

        loop {
            if self.disks.iter().copied().all(|disk| disk.is_open_at(time)) {
                return time;
            }

            time += 1;
        }
    }
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let disks = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { disks })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Disk {
    number: usize,
    positions: usize,
    start: usize,
}

impl Disk {
    fn is_open_at(self, press_time: usize) -> bool {
        (self.start + press_time + self.number).is_multiple_of(self.positions)
    }
}

impl FromStr for Disk {
    type Err = anyhow::Error;

    /// Parses:
    /// `Disc #1 has 5 positions; at time=0, it is at position 4.`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.len() != 12 {
            bail!("invalid disk line: {s}");
        }

        let number = parts[1]
            .trim_start_matches('#')
            .parse()
            .context("invalid disk number")?;

        let positions = parts[3].parse().context("invalid positions count")?;

        let start = parts[11]
            .trim_end_matches('.')
            .parse()
            .context("invalid starting position")?;

        Ok(Self {
            number,
            positions,
            start,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.
";

    #[test]
    fn test_parse_disk() {
        let result: Disk = "Disc #1 has 5 positions; at time=0, it is at position 4."
            .parse()
            .unwrap();

        let expected = Disk {
            number: 1,
            positions: 5,
            start: 4,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_machine() {
        let result: Machine = EXAMPLE.parse().unwrap();

        let expected = Machine {
            disks: vec![
                Disk {
                    number: 1,
                    positions: 5,
                    start: 4,
                },
                Disk {
                    number: 2,
                    positions: 2,
                    start: 1,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_part_1() {
        let machine: Machine = EXAMPLE.parse().unwrap();
        assert_eq!(machine.first_press_time(), 5);
    }

    #[test]
    fn test_disk_open_at() {
        let disk = Disk {
            number: 1,
            positions: 5,
            start: 4,
        };

        assert!(disk.is_open_at(0));
        assert!(!disk.is_open_at(1));
        assert!(!disk.is_open_at(2));
        assert!(!disk.is_open_at(3));
        assert!(!disk.is_open_at(4));
        assert!(disk.is_open_at(5));
    }
}
