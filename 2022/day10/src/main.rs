#![allow(unused)]

use std::{fs, str::FromStr};

use anyhow::{Context, Result, bail};

const INTERESTING_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

fn main() -> Result<()> {
    let input: Input = fs::read_to_string("input.txt")?.parse()?;
    let mut machine = Machine::default();

    let sol1 = machine
        .interesting_strengths(&input.instructions, &INTERESTING_CYCLES)
        .iter()
        .sum::<isize>();
    println!("Part 1: {sol1}");

    println!("Part 2: {}", machine.screen);
    Ok(())
}

struct Input {
    instructions: Vec<Instruction>,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self { instructions })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Machine {
    cycle: usize,
    register: Register,
    screen: Crt,
}

impl Machine {
    fn run(&mut self, instructions: &[Instruction]) {
        for &instruction in instructions {
            for _ in 0..instruction.cycles() {
                self.tick();
            }

            self.register.apply_instruction(instruction);
        }
    }

    fn tick(&mut self) {
        self.draw_pixel();
        self.cycle += 1;
    }

    fn draw_pixel(&mut self) {
        if self.is_on_sprite() {
            self.screen.push(Pixel::Lit);
        } else {
            self.screen.push(Pixel::Dark);
        }
    }

    fn is_on_sprite(&self) -> bool {
        let pos = (self.cycle % self.screen.width).cast_signed();
        let sprite_mid = self.register.0;

        (sprite_mid - 1..=sprite_mid + 1).contains(&pos)
    }

    fn interesting_strengths(
        &mut self,
        instructions: &[Instruction],
        intersting_cycles: &[usize],
    ) -> Vec<isize> {
        let mut out = Vec::with_capacity(intersting_cycles.len());

        for &instruction in instructions {
            for _ in 0..instruction.cycles() {
                self.tick();
                if intersting_cycles.contains(&self.cycle) {
                    out.push(self.register.0 * self.cycle.cast_signed());
                }
            }

            self.register.apply_instruction(instruction);
        }
        out
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Crt {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

impl Crt {
    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    fn get(&self, row: usize, col: usize) -> Option<&Pixel> {
        self.pixels.get(self.idx(row, col))
    }

    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Pixel> {
        let index = self.idx(row, col);
        self.pixels.get_mut(index)
    }

    fn push(&mut self, pixel: Pixel) {
        self.pixels.push(pixel);
    }
}

impl Default for Crt {
    fn default() -> Self {
        Self {
            width: 40,
            height: 6,
            pixels: Vec::with_capacity(40 * 6),
        }
    }
}

impl std::fmt::Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, pixel) in self.pixels.iter().enumerate() {
            if idx.is_multiple_of(self.width) {
                f.write_str("\n")?;
            }
            write!(f, "{pixel}")?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Pixel {
    Lit,
    Dark,
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pixel::Lit => f.write_str("#"),
            Pixel::Dark => f.write_str("."),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Register(isize);

impl Register {
    fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Addx(val) => self.0 += val,
            Instruction::NoOp => {}
        }
    }
}

impl Default for Register {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Instruction {
    Addx(isize),
    NoOp,
}

impl Instruction {
    fn cycles(self) -> usize {
        match self {
            Instruction::Addx(_) => 2,
            Instruction::NoOp => 1,
        }
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        match iter.next().context("empty instruction")? {
            "addx" => {
                let arg = iter.next().context("addx missing arg")?.parse()?;
                Ok(Self::Addx(arg))
            }
            "noop" => Ok(Self::NoOp),
            _ => bail!("unknown instruction: {s}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_crt() {
        let result = Crt {
            width: 4,
            height: 2,
            pixels: vec![Pixel::Lit; 8],
        }
        .to_string();
        let expected = "
####
####";
        assert_eq!(result, expected);
    }
}
