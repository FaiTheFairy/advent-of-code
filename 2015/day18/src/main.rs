use anyhow::{Context, Result, bail, ensure};
use grid::Grid;

use std::str::FromStr;

fn main() -> Result<()> {
    let light_grid: LightGrid = std::fs::read_to_string("input.txt")?.parse()?;

    let mut light_grid_v1 = light_grid.clone();
    light_grid_v1.step_v1_n_times(100);
    let sol1 = light_grid_v1.grid.iter().filter(|l| l.is_on()).count();
    println!("Part 1: {sol1}");

    let mut light_grid_v2 = light_grid.clone();
    light_grid_v2.step_v2_n_times(100);
    let sol2 = light_grid_v2.grid.iter().filter(|l| l.is_on()).count();
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LightGrid {
    grid: Grid<Light>,
}

impl LightGrid {
    fn step_v1_n_times(&mut self, n: usize) {
        for _ in 0..n {
            self.step_v1();
        }
    }

    fn step_v1(&mut self) {
        let current = self.clone();

        for ((row, col), light) in self.grid.indexed_iter_mut() {
            let lit_neighbors = current.count_lit_neighbors(row, col);

            *light = match (*light, lit_neighbors) {
                (Light::On, 2 | 3) | (Light::Off, 3) => Light::On,
                _ => Light::Off,
            }
        }
    }

    fn step_v2_n_times(&mut self, n: usize) {
        for _ in 0..n {
            self.step_v2();
        }
    }

    fn step_v2(&mut self) {
        self.turn_on_corners();
        let current = self.clone();
        for ((row, col), light) in self.grid.indexed_iter_mut() {
            let lit_neighbors = current.count_lit_neighbors(row, col);

            *light = match (*light, lit_neighbors) {
                (Light::On, 2 | 3) | (Light::Off, 3) => Light::On,
                _ => Light::Off,
            };
        }

        self.turn_on_corners();
    }

    fn turn_on_corners(&mut self) {
        let last_col = self.grid.cols() - 1;
        let last_row = self.grid.rows() - 1;

        self.grid[(0, 0)] = Light::On;
        self.grid[(0, last_col)] = Light::On;
        self.grid[(last_row, 0)] = Light::On;
        self.grid[(last_row, last_col)] = Light::On;
    }

    fn count_lit_neighbors(&self, row: usize, col: usize) -> usize {
        self.neighbors(row, col)
            .map(|(r, c)| self.grid[(r, c)])
            .filter(|l| l.is_on())
            .count()
    }

    fn neighbors(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
        #[rustfmt::skip]
        const OFFSETS: [(isize, isize); 8] = [
            (-1, -1),
            (-1,  0),
            (-1,  1),
            ( 0, -1),
            ( 0,  1),
            ( 1, -1),
            ( 1,  0),
            ( 1,  1),
        ];

        OFFSETS.into_iter().filter_map(move |(dr, dc)| {
            let new_row = row.checked_add_signed(dr)?;
            let new_col = col.checked_add_signed(dc)?;

            if new_row < self.grid.rows() && new_col < self.grid.cols() {
                Some((new_row, new_col))
            } else {
                None
            }
        })
    }
}

impl FromStr for LightGrid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines: Vec<&str> = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect();

        let cols = lines.first().context("empty grid")?.chars().count();

        ensure!(
            lines.iter().all(|line| line.chars().count() == cols),
            "grid is not rectangular"
        );

        let lights: Vec<Light> = lines
            .iter()
            .flat_map(|line| line.chars())
            .map(Light::try_from)
            .collect::<Result<_, _>>()?;

        let grid: Grid<Light> = Grid::from_vec(lights, cols);

        Ok(Self { grid })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Light {
    On,
    Off,
}

impl Light {
    /// Returns `true` if the light is [`On`].
    ///
    /// [`On`]: Light::On
    #[must_use]
    fn is_on(self) -> bool {
        matches!(self, Self::On)
    }
}

impl TryFrom<char> for Light {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::On),
            '.' => Ok(Self::Off),
            _ => bail!("unknown light state: {value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Light::*;
    use grid::grid;

    const EXAMPLE: &str = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";

    const EXAMPLE_1_STEP_V1: &str = "..##..
..##.#
...##.
......
#.....
#.##..";

    const EXAMPLE_1_STEP_V2: &str = "#.##.#
####.#
...##.
......
#...#.
#.####";

    const EXAMPLE_4_STEPS_V1: &str = "......
......
..##..
..##..
......
......";

    const EXAMPLE_5_STEPS_V2: &str = "##.###
.##..#
.##...
.##...
#.#...
##...#";

    #[test]
    fn test_step_v2_n_times() {
        let mut grid: LightGrid = EXAMPLE.parse().unwrap();

        grid.step_v2_n_times(5);
        assert_eq!(grid, EXAMPLE_5_STEPS_V2.parse().unwrap());
    }

    #[test]
    fn test_step_v2() {
        let mut grid: LightGrid = EXAMPLE.parse().unwrap();

        grid.step_v2();
        assert_eq!(grid, EXAMPLE_1_STEP_V2.parse().unwrap());
    }

    #[test]
    fn test_step_v1_n_times() {
        let mut grid: LightGrid = EXAMPLE.parse().unwrap();

        grid.step_v1_n_times(4);
        assert_eq!(grid, EXAMPLE_4_STEPS_V1.parse().unwrap());
    }

    #[test]
    fn test_step_v1() {
        let mut grid: LightGrid = EXAMPLE.parse().unwrap();

        grid.step_v1();
        assert_eq!(grid, EXAMPLE_1_STEP_V1.parse().unwrap());
    }

    #[test]
    fn test_parse_lightgrid() {
        let result: LightGrid = EXAMPLE.parse().unwrap();
        let expected = LightGrid {
            grid: grid![
                [Off, On, Off, On, Off, On]
                [Off, Off, Off, On, On, Off]
                [On, Off, Off, Off, Off, On]
                [Off, Off, On, Off, Off, Off]
                [On, Off, On, Off, Off, On]
                [On, On, On, On, Off, Off]
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_light() {
        assert_eq!(Light::try_from('.').unwrap(), Off);
        assert_eq!(Light::try_from('#').unwrap(), On);

        assert!(Light::try_from('-').is_err());
    }
}
