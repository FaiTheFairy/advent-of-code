#![allow(dead_code)]

use std::{
    collections::HashSet,
    fmt::Write,
    fs,
    ops::{Add, AddAssign, SubAssign},
    str::FromStr,
};

use anyhow::{Context, Result};

const BOUND_AREA_RATIO: f64 = 0.5;

fn main() -> Result<()> {
    let mut input: Input = fs::read_to_string("input.txt")?.parse()?;

    input.run_until_message();
    println!("Part 1:\ntime: {}\n{input}", input.time);

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Input {
    lights: Vec<Light>,
    time: u64,
}

impl Input {
    fn run_until_message(&mut self) {
        let mut previous_area = self.bounded_area();

        loop {
            self.pass_second();
            let area = self.bounded_area();

            if area > previous_area {
                self.rewind_second();
                break;
            }

            previous_area = area;
        }
    }

    fn pass_second(&mut self) {
        for light in &mut self.lights {
            light.pass_second();
        }

        self.time += 1;
    }

    fn rewind_second(&mut self) {
        for light in &mut self.lights {
            light.rewind_second();
        }

        self.time -= 1;
    }

    fn bounded_area(&self) -> usize {
        let bounds = self.bounds();

        let width = bounds.max_x.abs_diff(bounds.min_x) + 1;
        let height = bounds.max_y.abs_diff(bounds.min_y) + 1;

        width * height
    }

    fn bounds(&self) -> Bounds {
        let min_x = self
            .lights
            .iter()
            .map(|light| light.position.x)
            .min()
            .unwrap_or(0);
        let min_y = self
            .lights
            .iter()
            .map(|light| light.position.y)
            .min()
            .unwrap_or(0);
        let max_x = self
            .lights
            .iter()
            .map(|light| light.position.x)
            .max()
            .unwrap_or(0);
        let max_y = self
            .lights
            .iter()
            .map(|light| light.position.y)
            .max()
            .unwrap_or(0);
        Bounds {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lights = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self { lights, time: 0 })
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bounds = self.bounds();

        let points: HashSet<_> = self.lights.iter().map(|light| light.position).collect();

        for y in bounds.min_y..=bounds.max_y {
            for x in bounds.min_x..=bounds.max_x {
                if points.contains(&Coordinate { x, y }) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            if y < bounds.max_y {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Bounds {
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Light {
    position: Coordinate,
    velocity: Coordinate,
}

impl Light {
    fn pass_second(&mut self) {
        self.position += self.velocity;
    }

    fn rewind_second(&mut self) {
        self.position -= self.velocity;
    }
}

impl FromStr for Light {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s
            .strip_prefix("position=")
            .context("point missing 'position='")?;

        let (position, velocity) = s.split_once("velocity=").context("missing 'velocity='")?;

        let position = position.trim().parse().context("invalid position")?;
        let velocity = velocity.trim().parse().context("invalid velocity")?;

        Ok(Self { position, velocity })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl FromStr for Coordinate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (x, y) = s
            .strip_prefix('<')
            .context("coordinate missing '<'")?
            .strip_suffix('>')
            .context("coordinate missing '>'")?
            .split_once(',')
            .context("coordinate missing ','")?;

        let x = x.trim().parse().context("invalid x coordinate")?;
        let y = y.trim().parse().context("invalid y coordinate")?;

        Ok(Self { x, y })
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Coordinate {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>
";

    #[test]
    fn write_message() {
        let mut result: Input = EXAMPLE.parse().unwrap();
        result.run_until_message();

        println!("{}", result.time);
        println!("{result}");

        assert_eq!(result.time, 3);
    }

    #[test]
    fn after_one_second() {
        let mut result = "position=< 9,  1> velocity=< 0,  2>"
            .parse::<Light>()
            .unwrap();
        result.pass_second();
        let expected = Light {
            position: Coordinate { x: 9, y: 3 },
            velocity: Coordinate { x: 0, y: 2 },
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_point() {
        let result: Light = "position=< 9,  1> velocity=< 0,  2>".parse().unwrap();
        let expected = Light {
            position: Coordinate { x: 9, y: 1 },
            velocity: Coordinate { x: 0, y: 2 },
        };
        assert_eq!(result, expected);
    }
}
