#![allow(unused)]
use std::{
    collections::{BTreeMap, HashSet, VecDeque},
    fs,
    str::FromStr,
};

use anyhow::{Context, Result, bail};

fn main() -> Result<()> {
    let map: AsteroidMap = fs::read_to_string("input.txt")?.parse()?;

    let station = map.best_station().context("empty grid")?;
    println!("Part 1: {}", station.visible);

    let vaporized = map.vaporization_order(station.location);
    let asteroid_200 = vaporized[199];

    println!("Part 2: {}", asteroid_200.vaporization_code());

    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    /// Returns the *reduced direction vector* from `self` to `other`
    ///
    /// Two asteroids are in the same line-of-sight if they share the same reduced (dx, dy)
    fn direction_to(self, other: Self) -> Direction {
        let dx = other.x.cast_signed() - self.x.cast_signed();
        let dy = other.y.cast_signed() - self.y.cast_signed();
        Direction::new(dx, dy)
    }

    fn distance_squared_to(self, other: Self) -> u32 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        dx.pow(2) + dy.pow(2)
    }

    fn vaporization_code(self) -> u32 {
        self.x * 100 + self.y
    }
}

/// A normalized direction (dx, dy) reduced by gcd
///
/// Example:
/// ( 4, 2) -> (2, 1)
/// (10, 5) -> (2, 1)
///
/// These represent the same ray
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Direction {
    dx: i32,
    dy: i32,
}

impl Direction {
    fn new(dx: i32, dy: i32) -> Self {
        assert!(dx != 0 || dy != 0);

        let gcd = gcd(dx.abs(), dy.abs());

        Self {
            dx: dx / gcd,
            dy: dy / gcd,
        }
    }

    fn angle(self) -> Angle {
        Angle::from(self)
    }
}

/// A sortable angle wrapper
///
/// This can be created from `Direction` such that:
///
/// - "up" (0, -1) is FIRST
/// - rotation is clockwise
///
/// Standard atan2 gives:
///   atan2(y, x) -> counterclockwise from +x axis
///
/// We instead do:
///   atan2(dx, -dy)
///
/// Swapping the axes rotates the frame, and negating dy flips it so "up" becomes 0 rad
///
/// Then we normalize to [0, 2pi)
#[derive(Copy, Clone, Debug, PartialEq)]
struct Angle(f64);

impl From<Direction> for Angle {
    fn from(direction: Direction) -> Self {
        let radians = f64::from(direction.dx).atan2(f64::from(-direction.dy));
        let radians = radians.rem_euclid(std::f64::consts::TAU);

        Self(radians)
    }
}

// needed to sort angles in a BTreeMap
impl Eq for Angle {}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Angle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AsteroidMap {
    asteroids: Vec<Point>,
}

impl AsteroidMap {
    /// Returns number of asteroids visible from `station`
    fn visible_from(&self, station: Point) -> usize {
        self.asteroids
            .iter()
            .copied()
            .filter(|asteroid| *asteroid != station)
            .map(|asteroid| station.direction_to(asteroid))
            .collect::<HashSet<_>>()
            .len()
    }

    fn best_station(&self) -> Option<Station> {
        self.asteroids
            .iter()
            .copied()
            .map(|location| Station {
                location,
                visible: self.visible_from(location),
            })
            .max_by_key(|station| station.visible)
    }

    fn vaporization_order(&self, station: Point) -> Vec<Point> {
        // group asteroids by angle (i.e., by direction)
        let mut rays: BTreeMap<Angle, Vec<Point>> = BTreeMap::new();

        for asteroid in self.asteroids.iter().copied() {
            if asteroid == station {
                continue;
            }

            let angle = station.direction_to(asteroid).angle();

            // group all asteroids along the same ray
            rays.entry(angle).or_default().push(asteroid);
        }

        // sort each ray by distance (closest first)
        let mut rays: Vec<VecDeque<Point>> = rays
            .into_values()
            .map(|mut asteroids| {
                asteroids.sort_unstable_by_key(|asteroid| station.distance_squared_to(*asteroid));
                VecDeque::from(asteroids)
            })
            .collect();

        // simulate rotating laser
        let mut vaporized = Vec::new();

        // keep rotating until all rays are empty
        while rays.iter().any(|ray| !ray.is_empty()) {
            for ray in &mut rays {
                // laser hits the first asteroid in each direction
                if let Some(asteroid) = ray.pop_front() {
                    vaporized.push(asteroid);
                }
            }
        }

        vaporized
    }
}

impl FromStr for AsteroidMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut asteroids = Vec::with_capacity(s.bytes().filter(|b| *b == b'#').count());
        for (y, line) in s.trim().lines().enumerate() {
            for (x, cell) in line.trim().bytes().enumerate() {
                match cell {
                    b'#' => asteroids.push(Point {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    }),
                    b'.' => {}
                    other => bail!("unknown cell: {}", other as char),
                }
            }
        }

        Ok(Self { asteroids })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Station {
    location: Point,
    visible: usize,
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let rem = a % b;
        a = b;
        b = rem;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = ".#..#
.....
#####
....#
...##";

    const EXAMPLE_2: &str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

    #[test]
    fn asteroid_200_vaporization_code() {
        let map: AsteroidMap = EXAMPLE_2.parse().unwrap();
        let station = map.best_station().unwrap();
        let vaporized = map.vaporization_order(station.location);
        let asteroid_200 = vaporized[199];
        let result = asteroid_200.vaporization_code();
        assert_eq!(result, 802);
    }

    #[test]
    fn best_station() {
        let result = EXAMPLE_1
            .parse::<AsteroidMap>()
            .unwrap()
            .best_station()
            .unwrap();
        let expected = Station {
            location: Point { x: 3, y: 4 },
            visible: 8,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_asteroid_map() {
        let result: AsteroidMap = EXAMPLE_1.parse().unwrap();

        let expected = AsteroidMap {
            asteroids: vec![
                Point { x: 1, y: 0 },
                Point { x: 4, y: 0 },
                Point { x: 0, y: 2 },
                Point { x: 1, y: 2 },
                Point { x: 2, y: 2 },
                Point { x: 3, y: 2 },
                Point { x: 4, y: 2 },
                Point { x: 4, y: 3 },
                Point { x: 3, y: 4 },
                Point { x: 4, y: 4 },
            ],
        };

        assert_eq!(result, expected);
    }
}
