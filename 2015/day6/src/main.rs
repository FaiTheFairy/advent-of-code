use std::{fs, str::FromStr};

use anyhow::{Context, Result, bail};
use grid::*;

fn main() -> Result<()> {
    let instructions: Instructions = fs::read_to_string("input.txt")?.parse()?;

    let mut grid = Grid::from_vec(vec![Light::Off; 1_000_000], 1000);
    for entry in &instructions.0 {
        entry.apply(&mut grid);
    }
    let sol1 = grid.iter().filter(|&&l| l == Light::On).count();
    println!("Part 1: {sol1}");

    let mut grid_var = Grid::from_vec(vec![VariableLight { brightness: 0 }; 1_000_000], 1000);
    for entry in &instructions.0 {
        entry.apply_var(&mut grid_var);
    }
    let sol2: usize = grid_var.iter().map(|l| l.brightness).sum();
    println!("Part 2: {sol2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Instructions(Vec<Entry>);

impl FromStr for Instructions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self(instructions))
    }
}

/// row, column
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Coord {
    row: usize,
    col: usize,
}

impl FromStr for Coord {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (row, col) = s
            .trim()
            .split_once(',')
            .context("missing comma separator")?;
        let row = row.parse()?;
        let col = col.parse()?;

        Ok(Self { row, col })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Entry {
    action: Action,
    rectangle: Rectangle,
}

impl Entry {
    fn apply(self, grid: &mut Grid<Light>) {
        for Coord { row, col } in self.rectangle.contained_coords() {
            let light = grid.get_mut(row, col).expect("out of bounds");
            *light = self.action.apply(*light);
        }
    }

    fn apply_var(self, grid: &mut Grid<VariableLight>) {
        for Coord { row, col } in self.rectangle.contained_coords() {
            let light = grid.get_mut(row, col).expect("out of bounds");
            *light = self.action.apply_var(*light);
        }
    }
}

impl FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let first_digit_pos = s
            .bytes()
            .position(|b| b.is_ascii_digit())
            .context("no digit in rectangle entry")?;

        let (action, rest) = s.split_at(first_digit_pos);
        let action = action.parse::<Action>()?;
        let (upper_left, lower_right) = rest
            .split_once(" through ")
            .context("entry missing 'through'")?;

        let upper_left = upper_left.parse()?;
        let lower_right = lower_right.parse()?;

        let rectangle = Rectangle {
            upper_left,
            lower_right,
        };

        Ok(Self { action, rectangle })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Rectangle {
    upper_left: Coord,
    lower_right: Coord,
}

impl Rectangle {
    fn contained_coords(&self) -> impl Iterator<Item = Coord> {
        (self.upper_left.row..=self.lower_right.row).flat_map(move |row| {
            (self.upper_left.col..=self.lower_right.col).map(move |col| Coord { row, col })
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Action {
    fn apply(&self, light: Light) -> Light {
        match self {
            Action::TurnOn => Light::On,
            Action::TurnOff => Light::Off,
            Action::Toggle => match light {
                Light::On => Light::Off,
                Light::Off => Light::On,
            },
        }
    }

    fn apply_var(&self, var_light: VariableLight) -> VariableLight {
        match self {
            Action::TurnOn => VariableLight {
                brightness: var_light.brightness.saturating_add(1),
            },
            Action::TurnOff => VariableLight {
                brightness: var_light.brightness.saturating_sub(1),
            },
            Action::Toggle => VariableLight {
                brightness: var_light.brightness.saturating_add(2),
            },
        }
    }
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.trim() {
            "turn on" => Ok(Self::TurnOn),
            "turn off" => Ok(Self::TurnOff),
            "toggle" => Ok(Self::Toggle),
            _ => bail!("unknown action: {s}"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Light {
    On,
    Off,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct VariableLight {
    brightness: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contained_coords() {
        let result: Vec<Coord> = Rectangle {
            upper_left: Coord { row: 499, col: 499 },
            lower_right: Coord { row: 500, col: 500 },
        }
        .contained_coords()
        .collect();

        let expected = [
            Coord { row: 499, col: 499 },
            Coord { row: 499, col: 500 },
            Coord { row: 500, col: 499 },
            Coord { row: 500, col: 500 },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_entry() {
        let result: Entry = "turn on 0,0 through 999,999".parse().unwrap();
        let expected = Entry {
            action: Action::TurnOn,
            rectangle: Rectangle {
                upper_left: Coord { row: 0, col: 0 },
                lower_right: Coord { row: 999, col: 999 },
            },
        };

        assert_eq!(result, expected);
    }
}
