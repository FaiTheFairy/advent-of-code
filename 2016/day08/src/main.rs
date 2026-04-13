use std::{fmt::Write, fs, str::FromStr};

use anyhow::{Context, Result, bail, ensure};

fn main() -> Result<()> {
    let input: Input = fs::read_to_string("input.txt")?.parse()?;
    let mut screen = Screen::new(50, 6);

    screen.apply_instructions(input.instructions())?;
    let sol1 = screen.count_lit();
    println!("Part 1: {sol1}");

    println!("Part 2: \n{screen}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Input(Vec<Instruction>);

impl Input {
    fn instructions(&self) -> &[Instruction] {
        &self.0
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let inner = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self(inner))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Instruction {
    Rectangle { width: usize, height: usize },
    RotateRow { row: usize, shift: usize },
    RotateCol { col: usize, shift: usize },
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();

        match tokens.as_slice() {
            ["rect", dims] => {
                let (width, height) = dims
                    .split_once('x')
                    .context("rect dimensions missing 'x'")?;
                let width = width.parse()?;
                let height = height.parse()?;
                Ok(Self::Rectangle { width, height })
            }
            ["rotate", "row", row, "by", shift] => {
                let row = row
                    .strip_prefix("y=")
                    .context("rotate row missing 'y='")?
                    .parse()?;
                let shift = shift.parse()?;
                Ok(Self::RotateRow { row, shift })
            }
            ["rotate", "column", col, "by", shift] => {
                let col = col
                    .strip_prefix("x=")
                    .context("rotate column missing 'x='")?
                    .parse()?;
                let shift = shift.parse()?;
                Ok(Self::RotateCol { col, shift })
            }
            _ => bail!("unknown instruction: {s}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Screen {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        Screen {
            width,
            height,
            pixels: vec![Pixel::Off; height * width],
        }
    }

    fn count_lit(&self) -> usize {
        self.pixels.iter().filter(|l| l.is_on()).count()
    }

    fn apply_instructions(&mut self, instructions: &[Instruction]) -> Result<()> {
        for instruction in instructions {
            self.apply_instruction(*instruction)?;
        }
        Ok(())
    }

    fn apply_instruction(&mut self, instruction: Instruction) -> Result<()> {
        match instruction {
            Instruction::Rectangle { width, height } => self.rect(width, height),
            Instruction::RotateRow { row, shift } => self.rotate_row(row, shift),
            Instruction::RotateCol { col, shift } => self.rotate_col(col, shift),
        }
    }

    /// Turns on all of the pixels in a rectangle at the top-left of the screen
    /// which is `width` wide and `height` tall.
    fn rect(&mut self, width: usize, height: usize) -> Result<()> {
        ensure!(width <= self.width, "rect width exceeds screen width");
        ensure!(height <= self.height, "rect height exceeds screen height");

        for row in 0..height {
            for col in 0..width {
                let index = self.index(row, col);
                self.pixels[index] = Pixel::On;
            }
        }
        Ok(())
    }

    /// Shifts all of the pixels in `row` (0 is the top row) **right** by `shift` pixels.
    /// Pixels that would fall off the right end appear at the left end of the row.
    fn rotate_row(&mut self, row: usize, shift: usize) -> Result<()> {
        ensure!(row < self.height, "row out of bounds");

        let shift = shift % self.width;

        if shift == 0 {
            return Ok(());
        }

        let old_row = {
            let mut old_row = Vec::with_capacity(self.width);
            for col in 0..self.width {
                let index = self.index(row, col);
                old_row.push(self.pixels[index]);
            }
            old_row
        };

        for col in 0..self.width {
            let src_col = (col + self.width - shift) % self.width;
            let index = self.index(row, col);
            self.pixels[index] = old_row[src_col];
        }

        Ok(())
    }

    /// Shifts all of the pixels in `col` (0 is the left column) **down** by `shift` pixels.
    /// Pixels that would fall off the bottom appear at the top of the column.
    fn rotate_col(&mut self, col: usize, shift: usize) -> Result<()> {
        ensure!(col < self.width, "column out of bounds");

        let shift = shift % self.height;

        if shift == 0 {
            return Ok(());
        }

        let old_col = {
            let mut old_col = Vec::with_capacity(self.height);
            for row in 0..self.height {
                let index = self.index(row, col);
                old_col.push(self.pixels[index]);
            }
            old_col
        };

        for row in 0..self.height {
            let src_row = (row + self.height - shift) % self.height;
            let index = self.index(row, col);
            self.pixels[index] = old_col[src_row];
        }

        Ok(())
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
}

impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, pixel) in self.pixels.iter().enumerate() {
            if i.is_multiple_of(self.width) && i != 0 {
                f.write_char('\n')?;
            }
            write!(f, "{pixel}")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Pixel {
    On,
    Off,
}

impl Pixel {
    /// Returns `true` if the pixel is [`On`].
    ///
    /// [`On`]: Pixel::On
    #[must_use]
    fn is_on(self) -> bool {
        matches!(self, Self::On)
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pixel::On => f.write_char('#'),
            Pixel::Off => f.write_char('.'),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_screen() -> Screen {
        Screen::new(7, 3)
    }

    #[test]
    fn test_parse_instruction_rect() {
        let result: Instruction = "rect 3x2".parse().unwrap();
        let expected = Instruction::Rectangle {
            width: 3,
            height: 2,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_instruction_rotate_row() {
        let result: Instruction = "rotate row y=0 by 4".parse().unwrap();
        let expected = Instruction::RotateRow { row: 0, shift: 4 };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_instruction_rotate_col() {
        let result: Instruction = "rotate column x=1 by 1".parse().unwrap();
        let expected = Instruction::RotateCol { col: 1, shift: 1 };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example() {
        let mut screen = example_screen();
        screen.rect(3, 2).unwrap();
        assert_eq!(
            screen.to_string(),
            "###....
###....
......."
        );

        screen.rotate_col(1, 1).unwrap();
        assert_eq!(
            screen.to_string(),
            "#.#....
###....
.#....."
        );

        screen.rotate_row(0, 4).unwrap();
        assert_eq!(
            screen.to_string(),
            "....#.#
###....
.#....."
        );

        screen.rotate_col(1, 1).unwrap();
        assert_eq!(
            screen.to_string(),
            ".#..#.#
#.#....
.#....."
        );
    }
}
