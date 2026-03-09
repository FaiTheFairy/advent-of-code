use anyhow::{Context, Result, bail};
use std::{fs, str::FromStr};

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let mut game = input.parse::<BingoGame>()?;
    let (winning_idx, last_called) = game.play_till_first_win()?;
    let score = game.boards[winning_idx].score(last_called);
    println!("Part 1. score of first winning board = {score}");

    let (winning_idx, last_called) = game.play_till_last_win()?;
    let score = game.boards[winning_idx].score(last_called);
    println!("Part 2. score of last winning board = {score}");

    Ok(())
}

#[derive(Debug, Clone)]
struct BingoGame {
    draw: Vec<u8>,
    boards: Vec<BingoBoard>,
}

impl BingoGame {
    /// Returns winner index and final number called
    fn play_till_first_win(&mut self) -> Result<(usize, u8)> {
        for &number in &self.draw {
            for (idx, board) in self.boards.iter_mut().enumerate() {
                board.mark(number);

                if board.has_won() {
                    return Ok((idx, number));
                }
            }
        }
        bail!("no winning board found")
    }

    fn play_till_last_win(&mut self) -> Result<(usize, u8)> {
        let mut winners = Vec::new();
        for &number in &self.draw {
            for (idx, board) in self.boards.iter_mut().enumerate() {
                // need to skip changing the board state if we already counted it as a win.
                if winners.iter().any(|(i, _)| *i == idx) {
                    continue;
                }

                board.mark(number);

                if board.has_won() {
                    winners.push((idx, number));
                }
            }
        }
        winners.last().context("no winners").copied()
    }
}

impl FromStr for BingoGame {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (first_line, rest) = s.split_once("\n\n").context("draw numbers not found")?;
        let draw = first_line
            .split(',')
            .map(|d| d.parse::<u8>())
            .collect::<Result<Vec<u8>, _>>()?;

        let boards = rest
            .split("\n\n")
            .map(|b| b.trim().parse::<BingoBoard>())
            .collect::<Result<Vec<BingoBoard>, _>>()?;

        Ok(Self { draw, boards })
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct BingoBoard {
    numbers: [u8; 25],
    marked: [bool; 25],
}

impl BingoBoard {
    fn mark(&mut self, number: u8) {
        for i in 0..25 {
            if self.numbers[i] == number {
                self.marked[i] = true;
            }
        }
    }

    fn row_complete(&self, row: usize) -> bool {
        let start = row * 5;
        self.marked[start..start + 5].iter().all(|&b| b)
    }

    fn col_complete(&self, col: usize) -> bool {
        (0..5).all(|row| self.marked[row * 5 + col])
    }

    fn has_won(&self) -> bool {
        (0..5).any(|i| self.row_complete(i) || self.col_complete(i))
    }

    fn score(&self, last_called: u8) -> u32 {
        let sum_unmarked: u32 = self
            .numbers
            .iter()
            .zip(self.marked.iter())
            .filter(|(_, m)| !*m)
            .map(|(&n, _)| n as u32)
            .sum();

        sum_unmarked * last_called as u32
    }
}

impl FromStr for BingoBoard {
    type Err = anyhow::Error;

    /// Parses
    /// ```text
    /// 22 13 17 11  0
    ///  8  2 23  4 24
    /// 21  9 14 16  7
    ///  6 10  3 18  5
    ///  1 12 20 15 19
    /// ```
    /// to `BingoBoard` with all values marked `false`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = [0u8; 25];
        let mut idx = 0usize;
        for line in s.lines() {
            let line = line.trim();
            for n in line.split_whitespace() {
                let n = n.parse::<u8>()?;
                numbers[idx] = n;
                idx += 1;
            }
        }
        Ok(Self {
            numbers,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_solve_part_1() {
        let mut game = EXAMPLE.parse::<BingoGame>().unwrap();
        let (winning_idx, last_called) = game.play_till_first_win().unwrap();
        let result = game.boards[winning_idx].score(last_called);
        assert_eq!(result, 4512);
    }

    #[test]
    fn test_solve_part_2() {
        let mut game = EXAMPLE.parse::<BingoGame>().unwrap();
        let (winning_idx, last_called) = game.play_till_last_win().unwrap();
        dbg!(winning_idx, last_called);
        let result = game.boards[winning_idx].score(last_called);
        dbg!(result);
        assert_eq!(result, 1924);
    }

    #[test]
    fn test_parse_bingo_board() {
        let input = "22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19";

        let result = input.parse::<BingoBoard>().unwrap();
        let expected = BingoBoard {
            numbers: [
                22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20,
                15, 19,
            ],
            marked: [false; 25],
        };

        assert_eq!(result, expected)
    }
}
