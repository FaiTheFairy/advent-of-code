use std::str::FromStr;

use anyhow::{Result, anyhow};

pub fn solve_part_1(input: &str) -> Result<usize> {
    let input: Input = input.parse()?;
    let (id, wait) = input.first_bus_and_wait()?;
    Ok(id.0 as usize * wait as usize)
}

pub fn solve_part_2(input: &str) -> Result<u128> {
    let input: Input = input.parse()?;
    Ok(input.buses.earliest_aligned_timestamp())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Input {
    earliest_departure: u128,
    buses: Buses,
}

impl Input {
    fn first_bus_and_wait(&self) -> Result<(BusId, u128)> {
        self.buses
            .in_service()
            .map(|id| (id, id.wait_time(self.earliest_departure)))
            .min_by_key(|&(_, wait)| wait)
            .ok_or_else(|| anyhow!("no in-service buses found"))
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut lines = s.lines().map(str::trim).filter(|line| !line.is_empty());

        let earliest_departure = lines
            .next()
            .ok_or_else(|| anyhow!("missing earliest departure line"))?
            .parse::<u128>()?;

        let buses = lines
            .next()
            .ok_or_else(|| anyhow!("missing buses line"))?
            .parse::<Buses>()?;

        Ok(Self {
            earliest_departure,
            buses,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Buses(Vec<BusSlot>);

impl Buses {
    fn in_service(&self) -> impl Iterator<Item = BusId> + '_ {
        self.0.iter().filter_map(|slot| match slot {
            BusSlot::InService(id) => Some(*id),
            BusSlot::OutOfService => None,
        })
    }

    fn indexed_in_service(&self) -> impl Iterator<Item = (usize, BusId)> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(offset, slot)| match slot {
                BusSlot::InService(id) => Some((offset, *id)),
                BusSlot::OutOfService => None,
            })
    }

    fn earliest_aligned_timestamp(&self) -> u128 {
        let mut timestamp = 0u128;
        let mut step = 1u128;

        for (offset, id) in self.indexed_in_service() {
            let id = id.0;
            let offset = offset as u128;

            while !(timestamp + offset).is_multiple_of(id) {
                timestamp += step;
            }

            step = lcm(step, id);
        }

        timestamp
    }
}

impl FromStr for Buses {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let buses = s
            .split(',')
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<Vec<BusSlot>, _>>()?;

        Ok(Self(buses))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum BusSlot {
    InService(BusId),
    OutOfService,
}

impl FromStr for BusSlot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s == "x" {
            return Ok(Self::OutOfService);
        }

        Ok(Self::InService(BusId(s.parse()?)))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct BusId(u128);

impl BusId {
    fn wait_time(self, t0: u128) -> u128 {
        let id = self.0;
        // Distance to the next multiple of `id`. The final `% id` makes the
        // exact-departure case return 0 instead of `id`.
        (id - (t0 % id)) % id
    }
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn lcm(a: u128, b: u128) -> u128 {
    a / gcd(a, b) * b
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(EXAMPLE).unwrap();
        assert_eq!(result, 295);
    }

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(EXAMPLE).unwrap();
        assert_eq!(result, 1_068_781);
    }

    #[test]
    fn test_part_2_other_examples() {
        assert_eq!(solve_part_2("0\n17,x,13,19").unwrap(), 3417);
        assert_eq!(solve_part_2("0\n67,7,59,61").unwrap(), 754_018);
        assert_eq!(solve_part_2("0\n67,x,7,59,61").unwrap(), 779_210);
        assert_eq!(solve_part_2("0\n67,7,x,59,61").unwrap(), 1_261_476);
        assert_eq!(solve_part_2("0\n1789,37,47,1889").unwrap(), 1_202_161_486);
    }

    #[test]
    fn test_parse_input() {
        let result = EXAMPLE.parse::<Input>().unwrap();
        let expected = Input {
            earliest_departure: 939,
            buses: Buses(vec![
                BusSlot::InService(BusId(7)),
                BusSlot::InService(BusId(13)),
                BusSlot::OutOfService,
                BusSlot::OutOfService,
                BusSlot::InService(BusId(59)),
                BusSlot::OutOfService,
                BusSlot::InService(BusId(31)),
                BusSlot::InService(BusId(19)),
            ]),
        };
        assert_eq!(result, expected);
    }
}
