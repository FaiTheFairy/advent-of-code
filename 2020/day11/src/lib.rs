#![allow(unused)]

use std::str::FromStr;

use anyhow::{Result, bail};
use grid::{Grid, grid};

pub fn solve_part_1(input: &str) -> Result<usize> {
    let layout: Layout = input.parse()?;
    let sol = layout.final_layout(RULES_1).count_occupied();
    Ok(sol)
}

pub fn solve_part_2(input: &str) -> Result<usize> {
    let layout: Layout = input.parse()?;
    let sol = layout.final_layout(RULES_2).count_occupied();
    Ok(sol)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Layout(Grid<Tile>);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rules {
    visibility: Visibility,
    occupied_threshold: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Visibility {
    Adjacent,
    LineOfSight,
}

pub const RULES_1: Rules = Rules {
    visibility: Visibility::Adjacent,
    occupied_threshold: 4,
};

pub const RULES_2: Rules = Rules {
    visibility: Visibility::LineOfSight,
    occupied_threshold: 5,
};

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),  // above
    (1, 0),   // below
    (0, 1),   // right
    (0, -1),  // left
    (-1, 1),  // upper right
    (-1, -1), // upper left
    (1, 1),   // lower right
    (1, -1),  // lower left
];

impl Layout {
    fn count_occupied(&self) -> usize {
        self.0
            .iter()
            .filter(|&&tile| tile == Tile::Occupied)
            .count()
    }

    fn final_layout(&self, rules: Rules) -> Self {
        let mut current = self.clone();

        loop {
            let next = current.with_rules_applied(rules);
            if next == current {
                return current;
            }
            current = next;
        }
    }

    fn with_rules_applied(&self, rules: Rules) -> Self {
        let mut next = self.clone();

        for ((row, col), tile) in next.0.indexed_iter_mut() {
            let occupied = self.count_visible_occupied(row, col, rules.visibility);

            *tile = match self.0[(row, col)] {
                Tile::Empty if occupied == 0 => Tile::Occupied,
                Tile::Occupied if occupied >= rules.occupied_threshold => Tile::Empty,
                other => other,
            };
        }

        next
    }

    fn count_visible_occupied(&self, row: usize, col: usize, visibility: Visibility) -> usize {
        DIRECTIONS
            .into_iter()
            .filter(|&(dr, dc)| {
                self.first_seen_seat(row, col, dr, dc, visibility) == Some(Tile::Occupied)
            })
            .count()
    }

    fn first_seen_seat(
        &self,
        row: usize,
        col: usize,
        dr: isize,
        dc: isize,
        visibility: Visibility,
    ) -> Option<Tile> {
        let mut r = row as isize + dr;
        let mut c = col as isize + dc;

        loop {
            let tile = self.0.get(r as usize, c as usize).copied()?;

            match tile {
                Tile::Floor if visibility == Visibility::LineOfSight => {
                    r += dr;
                    c += dc;
                }
                Tile::Floor => return Some(Tile::Floor),
                Tile::Empty => return Some(Tile::Empty),
                Tile::Occupied => return Some(Tile::Occupied),
            }
        }
    }
}

impl FromStr for Layout {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines: Vec<&str> = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect();

        let rows = lines.len();
        let cols = lines.first().map(|line| line.chars().count()).unwrap_or(0);

        if rows == 0 || cols == 0 {
            bail!("layout cannot be empty");
        }

        if lines.iter().any(|line| line.chars().count() != cols) {
            bail!("layout must be rectangular");
        }

        let mut data = Vec::with_capacity(rows * cols);
        for line in lines {
            for ch in line.chars() {
                data.push(ch.try_into()?);
            }
        }

        Ok(Self(Grid::from_vec(data, cols)))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Occupied,
    Floor,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'L' => Ok(Tile::Empty),
            '#' => Ok(Tile::Occupied),
            '.' => Ok(Tile::Floor),
            _ => bail!("unknown tole value: {value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    const RULE_1_APPLIED: &str = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

    const FINAL_1: &str = "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##";

    const FINAL_2: &str = "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#";

    #[test]
    fn test_solve_part_1() {
        let result = EXAMPLE
            .parse::<Layout>()
            .unwrap()
            .final_layout(RULES_1)
            .count_occupied();
        assert_eq!(result, 37);
    }

    #[test]
    fn test_solve_part_2() {
        let result = EXAMPLE
            .parse::<Layout>()
            .unwrap()
            .final_layout(RULES_2)
            .count_occupied();
        assert_eq!(result, 26);
    }

    #[test]
    fn test_final_layout_1() {
        let result = EXAMPLE.parse::<Layout>().unwrap().final_layout(RULES_1);
        let expected = FINAL_1.parse::<Layout>().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_final_layout_2() {
        let result = EXAMPLE.parse::<Layout>().unwrap().final_layout(RULES_2);
        let expected = FINAL_2.parse::<Layout>().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_apply_rule_1() {
        let result = EXAMPLE
            .parse::<Layout>()
            .unwrap()
            .with_rules_applied(RULES_1);
        let expected = RULE_1_APPLIED.parse::<Layout>().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_occupied() {
        let result = FINAL_1.parse::<Layout>().unwrap().count_occupied();
        assert_eq!(result, 37);
    }

    #[test]
    fn test_count_occupied_adjacent() {
        let layout: Layout = "###
#L#
###"
        .parse()
        .unwrap();
        let result = layout.count_visible_occupied(1, 1, Visibility::Adjacent);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_count_occupied_line_of_sight() {
        let layout: Layout = "#.#\n.L.\n#.#".parse().unwrap();
        let result = layout.count_visible_occupied(1, 1, Visibility::LineOfSight);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_parse_layout() {
        let result: Layout = "L.#\nLLL\n.##".parse().unwrap();
        let expected = Layout(grid![
            [Tile::Empty, Tile::Floor, Tile::Occupied]
            [Tile::Empty, Tile::Empty, Tile::Empty]
            [Tile::Floor, Tile::Occupied, Tile::Occupied]
        ]);
        assert_eq!(result, expected);
    }
}
