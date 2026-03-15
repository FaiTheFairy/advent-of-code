use std::{fs, str::FromStr};

use anyhow::{Result, anyhow, bail, ensure};

fn main() -> Result<()> {
    let list = fs::read_to_string("input.txt")?.parse::<List>()?;
    let sol1 = list.solve_part_1()?;
    println!("Part 1. Highest seat ID in boarding passes = {sol1}");

    let sol2 = list.solve_part_2()?;
    println!("Part 2. my id = {sol2}");

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct List(Vec<BoardingPass>);

impl List {
    fn solve_part_1(&self) -> Result<usize> {
        self.0
            .iter()
            .map(BoardingPass::seat_id)
            .max()
            .ok_or_else(|| anyhow!("can't solve part 1 for empty list"))
    }

    fn solve_part_2(&self) -> Result<usize> {
        let mut ids: Vec<usize> = self.0.iter().map(BoardingPass::seat_id).collect();
        ids.sort_unstable();

        for [id1, id2] in ids.array_windows() {
            if id1 + 2 == *id2 {
                return Ok(id1 + 1);
            }
        }

        bail!("no solution found");
    }
}

impl FromStr for List {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut out = Vec::with_capacity(s.lines().count());

        for line in s.trim().lines() {
            let boarding_pass = line.parse::<BoardingPass>()?;
            out.push(boarding_pass);
        }

        Ok(Self(out))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BoardingPass {
    row: [RowDir; 7],
    col: [ColDir; 3],
}

impl BoardingPass {
    fn row(&self) -> usize {
        let dirs = &self.row;

        let mut low = 0;
        let mut high = 127;

        for dir in dirs {
            let mid = (high + low) / 2;
            match dir {
                RowDir::Front => high = mid,
                RowDir::Back => low = mid + 1,
            }
        }

        low
    }

    fn col(&self) -> usize {
        let dirs = &self.col;

        let mut low = 0;
        let mut high = 7;

        for dir in dirs {
            let mid = (high + low) / 2;
            match dir {
                ColDir::Left => high = mid,
                ColDir::Right => low = mid + 1,
            }
        }

        low
    }

    fn seat_id(&self) -> usize {
        let row = self.row();
        let col = self.col();

        row * 8 + col
    }
}

impl FromStr for BoardingPass {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        ensure!(
            bytes.len() == 10,
            "boarding pass must be exactly 10 characters"
        );
        ensure!(
            bytes[..7].iter().all(|b| matches!(b, b'F' | b'B')),
            "first 7 characters must only indicate rows (F or B)"
        );
        ensure!(
            bytes[7..].iter().all(|b| matches!(b, b'L' | b'R')),
            "last 3 characters must only indicate columns (L or R)"
        );

        let row: [RowDir; 7] = [
            bytes[0].try_into()?,
            bytes[1].try_into()?,
            bytes[2].try_into()?,
            bytes[3].try_into()?,
            bytes[4].try_into()?,
            bytes[5].try_into()?,
            bytes[6].try_into()?,
        ];

        let col: [ColDir; 3] = [
            bytes[7].try_into()?,
            bytes[8].try_into()?,
            bytes[9].try_into()?,
        ];

        Ok(Self { row, col })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RowDir {
    Front,
    Back,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ColDir {
    Left,
    Right,
}

impl TryFrom<u8> for RowDir {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            b'F' => Ok(Self::Front),
            b'B' => Ok(Self::Back),
            _ => bail!("unknown letter for row direction (not from [F, B])"),
        }
    }
}

impl TryFrom<u8> for ColDir {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            b'L' => Ok(Self::Left),
            b'R' => Ok(Self::Right),
            _ => bail!("unknown letter for column direction (not from [L, R])"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_LIST: &str = "BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL";
    const EXAMPLE_PASS: &str = "BFFFBBFRRR";
    const EXAMPLE_PASS_2: &str = "FBFBBFFRLR";

    #[test]
    fn test_solve_part_1() {
        let result = EXAMPLE_LIST
            .parse::<List>()
            .unwrap()
            .solve_part_1()
            .unwrap();
        assert_eq!(result, 820);
    }

    #[test]
    fn test_boarding_pass_seat_id() {
        let result = EXAMPLE_PASS_2.parse::<BoardingPass>().unwrap().seat_id();
        assert_eq!(result, 357);
    }

    #[test]
    fn test_boarding_pass_col() {
        let result = EXAMPLE_PASS_2.parse::<BoardingPass>().unwrap().col();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_boarding_pass_row() {
        let result = EXAMPLE_PASS_2.parse::<BoardingPass>().unwrap().row();
        assert_eq!(result, 44);
    }

    #[test]
    fn test_parse_list() {
        let result = EXAMPLE_LIST.parse::<List>().unwrap();
        let expected = List(vec![
            "BFFFBBFRRR".parse::<BoardingPass>().unwrap(),
            "FFFBBBFRRR".parse::<BoardingPass>().unwrap(),
            "BBFFBBFRLL".parse::<BoardingPass>().unwrap(),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_boarding_pass() {
        let result = EXAMPLE_PASS.parse::<BoardingPass>().unwrap();
        let expected = BoardingPass {
            row: [
                RowDir::Back,
                RowDir::Front,
                RowDir::Front,
                RowDir::Front,
                RowDir::Back,
                RowDir::Back,
                RowDir::Front,
            ],
            col: [ColDir::Right, ColDir::Right, ColDir::Right],
        };
        assert_eq!(result, expected);
    }
}
